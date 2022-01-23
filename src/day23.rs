use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn energy_per_step(&self) -> u32 {
        match self {
            &Amphipod::Amber => 1,
            &Amphipod::Bronze => 10,
            &Amphipod::Copper => 100,
            &Amphipod::Desert => 1000,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Burrow {
    hallway: [Option<Amphipod>; 11],
    room_a: [Option<Amphipod>; 2],
    room_b: [Option<Amphipod>; 2],
    room_c: [Option<Amphipod>; 2],
    room_d: [Option<Amphipod>; 2],
}

impl Burrow {

    fn room_locations() -> &'static[usize; 4] {
        &[2, 4, 6, 8]
    }

    fn next_states(&self) -> Vec<(Burrow, u32)> {
        let mut states: Vec<(Burrow, u32)> = Vec::new();
        // amphipods in the hallway must move to its assigned room
        for ndx in 0..self.hallway.len() {
            if self.hallway[ndx].is_some() {
                let amph = self.hallway[ndx].unwrap();
                let dest: &[Option<Amphipod>; 2] = match amph {
                    Amphipod::Amber => &self.room_a,
                    Amphipod::Bronze => &self.room_b,
                    Amphipod::Copper => &self.room_c,
                    Amphipod::Desert => &self.room_d,
                };
                if dest[0] == Option::None && match dest[1] {
                    Option::None => true,
                    Option::Some(a) => a == amph,
                } {
                    // the room is open. is the hallway clear?
                    let room_loc = Burrow::room_locations()[match amph {
                        Amphipod::Amber => 0,
                        Amphipod::Bronze => 1,
                        Amphipod::Copper => 2,
                        Amphipod::Desert => 3,
                    }];
                    let range = if ndx < room_loc {
                        ndx+1 .. room_loc+1
                    } else {
                        room_loc .. ndx
                    };
                    if self.hallway[range].iter()
                        .all(|space| space.is_none()) {
                        
                        let down_steps = match dest[1] {
                            None => 2,
                            Some(_) => 1,
                        };
                        let across_steps = if ndx < room_loc {
                            room_loc - ndx
                        } else {
                            ndx - room_loc
                        };
                        let energy = (across_steps + down_steps) as u32 * amph.energy_per_step();
                        let mut new_burrow = self.clone();
                        let new_dest: &mut [Option<Amphipod>; 2] = match amph {
                            Amphipod::Amber => &mut new_burrow.room_a,
                            Amphipod::Bronze => &mut new_burrow.room_b,
                            Amphipod::Copper => &mut new_burrow.room_c,
                            Amphipod::Desert => &mut new_burrow.room_d,
                        };
                        new_burrow.hallway[ndx] = Option::None;
                        new_dest[down_steps-1] = Option::Some(amph);
                        states.push((new_burrow, energy));
                    }
                }
            }
        }
        // amphipods in a room may move to the hallway
        let rooms = [&self.room_a, &self.room_b, &self.room_c, &self.room_d];
        let amphs = [Amphipod::Amber, Amphipod::Bronze, Amphipod::Copper, Amphipod::Desert];
        for room_data in amphs.into_iter().zip(rooms).zip(Burrow::room_locations()) {
            let ((home_amph, room,), loc) = room_data;
            // which (if any) amphipod can move from this room?
            let mover_ndx: Option<usize>;
            if room[0].is_some() {
                // this can move if it's not in its home, or if the partner is not its home
                if room[0].unwrap() != home_amph || room[1].unwrap() != home_amph {
                    mover_ndx = Option::Some(0);
                } else {
                    mover_ndx = Option::None;
                }
            } else if room[1].is_some() {
                // this can move if it's not in its home
                if room[1].unwrap() != home_amph {
                    mover_ndx = Option::Some(1);
                } else {
                    mover_ndx = Option::None;
                }
            } else {
                mover_ndx = Option::None;
            }
            // where can this amphipod go?
            let mut dests: Vec<usize> = Vec::new();
            if mover_ndx.is_some() {
                let mover_ndx = mover_ndx.unwrap();
                let mut potential_dest = loc - 1;
                while self.hallway[potential_dest].is_none() {
                    if !Burrow::room_locations().contains(&potential_dest) {
                        dests.push(potential_dest);
                    }
                    if potential_dest == 0 {
                        break;
                    } else {
                        potential_dest -= 1;
                    }
                }
                potential_dest = loc + 1;
                while potential_dest < self.hallway.len() && self.hallway[potential_dest].is_none() {
                    if !Burrow::room_locations().contains(&potential_dest) {
                        dests.push(potential_dest);
                    }
                    potential_dest += 1;
                }
                states.extend(dests.into_iter()
                    .map(|dest| {
                        let mut new_burrow = self.clone();
                        let new_source: &mut [Option<Amphipod>; 2] = match home_amph {
                            Amphipod::Amber => &mut new_burrow.room_a,
                            Amphipod::Bronze => &mut new_burrow.room_b,
                            Amphipod::Copper => &mut new_burrow.room_c,
                            Amphipod::Desert => &mut new_burrow.room_d,
                        };
                        let moving_amph = new_source[mover_ndx].unwrap();
                        new_burrow.hallway[dest] = Option::Some(moving_amph);
                        new_source[mover_ndx] = Option::None;
                        let up_steps = mover_ndx + 1;
                        let across_steps = if dest < *loc {
                            loc - dest
                        } else {
                            dest - loc
                        };
                        let energy = (up_steps + across_steps) as u32 * moving_amph.energy_per_step();
                        (new_burrow, energy)
                    })
                );
            }
        }

        states
    }

    fn heuristic(&self) -> u32 {
        let mut h: u32 = 0;
        // how much energy for each amphipod in the hallway to get into its home room?
        for hallway_ndx in 0..self.hallway.len() {
            if self.hallway[hallway_ndx].is_some() {
                let amph = self.hallway[hallway_ndx].unwrap();
                let home_ndx = Burrow::room_locations()[match amph {
                    Amphipod::Amber => 0,
                    Amphipod::Bronze => 1,
                    Amphipod::Copper => 2,
                    Amphipod::Desert => 3,
                }];
                let steps_across = if hallway_ndx > home_ndx {
                    hallway_ndx - home_ndx
                } else {
                    home_ndx - hallway_ndx
                };
                // x steps across plus one step down to its home
                let energy = (steps_across + 1) as u32 * amph.energy_per_step();
                h += energy;
            }
        }
        // how much energy for each amphipod in the wrong room to get into its home room?
        let amphs = [Amphipod::Amber, Amphipod::Bronze, Amphipod::Copper, Amphipod::Desert];
        let rooms = [&self.room_a, &self.room_b, &self.room_c, &self.room_d];
        for t in amphs.into_iter().zip(Burrow::room_locations()).zip(rooms) {
            let ((home_amph, home_loc), room) = t;
            for room_ndx in 0..room.len() {
                if room[room_ndx].is_some() {
                    let amph = room[room_ndx].unwrap();
                    if amph != home_amph {
                        // move (room_ndx+1) steps up ...
                        let up_steps = room_ndx + 1;
                        // ... move across to just above my home...
                        let my_home_loc = Burrow::room_locations()[match amph {
                            Amphipod::Amber => 0,
                            Amphipod::Bronze => 1,
                            Amphipod::Copper => 2,
                            Amphipod::Desert => 3,
                        }];
                        let across_steps = if my_home_loc > *home_loc {
                            my_home_loc - home_loc
                        } else {
                            home_loc - my_home_loc
                        };
                        // ... and move one step down to my room
                        let energy = (up_steps + across_steps + 1) as u32 * amph.energy_per_step();
                        h += energy;
                    }
                }
            } 
        }

        h
    }
}

fn load_burrow(file: &str) -> Burrow {
    let file = File::open(file).expect("could not open file");
    let mut buf_reader = BufReader::new(file);
    let mut buffer: String = String::new();
    // first 2 rows irrelevant -- there are 11 spots in the hallway
    buf_reader.read_line(&mut buffer).unwrap();
    buf_reader.read_line(&mut buffer).unwrap();
    // remaining rows: rooms
    let mut room_a: [Option<Amphipod>; 2] = [Option::None; 2];
    let mut room_b: [Option<Amphipod>; 2] = [Option::None; 2];
    let mut room_c: [Option<Amphipod>; 2] = [Option::None; 2];
    let mut room_d: [Option<Amphipod>; 2] = [Option::None; 2];
    for ndx in 0..2 {
        buffer.clear();
        buf_reader.read_line(&mut buffer).unwrap();
        let rooms = [&mut room_a, &mut room_b, &mut room_c, &mut room_d];
        for room_loc in rooms.into_iter().zip(Burrow::room_locations()) {
            room_loc.0[ndx] = match buffer.chars().nth(room_loc.1 + 1).unwrap() {
                'A' => Option::Some(Amphipod::Amber),
                'B' => Option::Some(Amphipod::Bronze),
                'C' => Option::Some(Amphipod::Copper),
                'D' => Option::Some(Amphipod::Desert),
                _ => panic!("unexpected character"),
            };
        }
    }
    let hallway = [Option::None; 11];
    Burrow{ hallway, room_a, room_b, room_c, room_d, }
}

fn least_energy(source: &Burrow, dest: &Burrow) -> u32 {
    let mut visited: HashMap<Burrow, u32> = HashMap::new();
    let mut queue: Vec<(Burrow, u32)> = vec![(*source, 0)];
    while !visited.contains_key(dest) {
        queue.sort_by(|a, b| {
            let h_a = a.1 + a.0.heuristic();
            let h_b = b.1 + b.0.heuristic();
            // put smaller values at the end 
            h_b.cmp(&h_a)
        });
        let (burrow, energy) = queue.pop().unwrap();
        visited.insert(burrow, energy);
        for neighbor in burrow.next_states() {
            if visited.contains_key(&neighbor.0) {
                continue;
            }
            let neighbor_in_map = visited.get(&neighbor.0);
            if neighbor_in_map.is_none() || *neighbor_in_map.unwrap() > energy + neighbor.1 {
                visited.insert(neighbor.0, energy + neighbor.1);
            }
            queue.push((neighbor.0, energy + neighbor.1));
        }
    }
    *visited.get(dest).unwrap()
}

/**
Run part 1 of Day 23's exercise.

# Examples
```
assert_eq!(12521, aoc2021::day23::run_part1("test_inputs/day23.txt"));
```
 */
pub fn run_part1(file: &str) -> u32 {
    let burrow = load_burrow(file);
    let dest = Burrow {
        hallway: [Option::None; 11],
        room_a: [Option::Some(Amphipod::Amber); 2],
        room_b: [Option::Some(Amphipod::Bronze); 2],
        room_c: [Option::Some(Amphipod::Copper); 2],
        room_d: [Option::Some(Amphipod::Desert); 2],
    };
    least_energy(&burrow, &dest)
}