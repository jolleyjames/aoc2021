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
    room_a: [Option<Amphipod>; 4],
    room_b: [Option<Amphipod>; 4],
    room_c: [Option<Amphipod>; 4],
    room_d: [Option<Amphipod>; 4],
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
                let dest: &[Option<Amphipod>; 4] = match amph {
                    Amphipod::Amber => &self.room_a,
                    Amphipod::Bronze => &self.room_b,
                    Amphipod::Copper => &self.room_c,
                    Amphipod::Desert => &self.room_d,
                };
                if dest.into_iter().all(|o| o.is_none() || o.unwrap() == amph) {
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
                        
                        let mut down_steps: usize = 0;
                        for space in dest {
                            if space.is_none() {
                                down_steps += 1;
                            } else {
                                break;
                            }
                        }
                        let across_steps = if ndx < room_loc {
                            room_loc - ndx
                        } else {
                            ndx - room_loc
                        };
                        let energy = (across_steps + down_steps) as u32 * amph.energy_per_step();
                        let mut new_burrow = self.clone();
                        let new_dest: &mut [Option<Amphipod>; 4] = match amph {
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
            let mut mover_ndx: Option<usize> = Option::None;
            for potential_mover_ndx in 0..room.len() {
                if room[potential_mover_ndx].is_some() {
                    if room[potential_mover_ndx].unwrap() != home_amph {
                        mover_ndx = Option::Some(potential_mover_ndx);
                        break;
                    } else if (potential_mover_ndx+1..room.len()).into_iter()
                        .map(|n| room[n].unwrap())
                        .any(|amph_below| amph_below != home_amph) {

                        mover_ndx = Option::Some(potential_mover_ndx);
                        break;
                    }
                }
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
                        let new_source: &mut [Option<Amphipod>; 4] = match home_amph {
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

fn load_burrow(file: &str, part: u8) -> Burrow {
    if ![1,2].contains(&part) {
        panic!("part was {}, must be 1 or 2", part);
    }
    let file = File::open(file).expect("could not open file");
    let mut buf_reader = BufReader::new(file);
    let mut buffer: String = String::new();
    // first 2 rows irrelevant -- there are 11 spots in the hallway
    buf_reader.read_line(&mut buffer).unwrap();
    buf_reader.read_line(&mut buffer).unwrap();
    // remaining rows: rooms
    let mut room_a: [Option<Amphipod>; 4] = [Option::None; 4];
    let mut room_b: [Option<Amphipod>; 4] = [Option::None; 4];
    let mut room_c: [Option<Amphipod>; 4] = [Option::None; 4];
    let mut room_d: [Option<Amphipod>; 4] = [Option::None; 4];
    let from_file: [usize; 2];
    if part == 1 {
        room_a[2] = Option::Some(Amphipod::Amber);
        room_a[3] = Option::Some(Amphipod::Amber);
        room_b[2] = Option::Some(Amphipod::Bronze);
        room_b[3] = Option::Some(Amphipod::Bronze);
        room_c[2] = Option::Some(Amphipod::Copper);
        room_c[3] = Option::Some(Amphipod::Copper);
        room_d[2] = Option::Some(Amphipod::Desert);
        room_d[3] = Option::Some(Amphipod::Desert);
        from_file = [0,1];
    } else {
        room_a[1] = Option::Some(Amphipod::Desert);
        room_a[2] = Option::Some(Amphipod::Desert);
        room_b[1] = Option::Some(Amphipod::Copper);
        room_b[2] = Option::Some(Amphipod::Bronze);
        room_c[1] = Option::Some(Amphipod::Bronze);
        room_c[2] = Option::Some(Amphipod::Amber);
        room_d[1] = Option::Some(Amphipod::Amber);
        room_d[2] = Option::Some(Amphipod::Copper);
        from_file = [0,3];
    }
    for ndx in from_file {
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
Run Day 23's exercise.

# Examples
```
assert_eq!(12521, aoc2021::day23::run(1, "test_inputs/day23.txt"));
assert_eq!(44169, aoc2021::day23::run(2, "test_inputs/day23.txt"));
```
 */
pub fn run(part: u8, file: &str) -> u32 {
    let burrow = load_burrow(file, part);
    let dest = Burrow {
        hallway: [Option::None; 11],
        room_a: [Option::Some(Amphipod::Amber); 4],
        room_b: [Option::Some(Amphipod::Bronze); 4],
        room_c: [Option::Some(Amphipod::Copper); 4],
        room_d: [Option::Some(Amphipod::Desert); 4],
    };
    least_energy(&burrow, &dest)
}
