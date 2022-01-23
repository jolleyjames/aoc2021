use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

#[derive(Clone, Copy, Debug)]
struct Instruction {
    on: bool,
    x_range: (i32,i32),
    y_range: (i32,i32),
    z_range: (i32,i32),
}

impl Instruction {
    fn volume(&self) -> i64 {
        (1 + self.x_range.1 - self.x_range.0) as i64 *
        (1 + self.y_range.1 - self.y_range.0) as i64 *
        (1 + self.z_range.1 - self.z_range.0) as i64
    }
    fn new(on: bool, x_range: &(i32,i32), y_range: &(i32,i32), z_range: &(i32,i32)) -> Instruction {
        if x_range.0 > x_range.1 {
            panic!("illegal x_range {:?}", x_range);
        }
        if y_range.0 > y_range.1 {
            panic!("illegal x_range {:?}", y_range);
        }
        if z_range.0 > z_range.1 {
            panic!("illegal x_range {:?}", z_range);
        }
        Instruction {on, x_range: *x_range, y_range: *y_range, z_range: *z_range}
    }
}

fn load_instructions(file: &str) -> Vec<Instruction> {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    buf_reader.lines()
        .map(|line_wrap| {
            let line = line_wrap.unwrap();
            let split_line: Vec<&str> = line.split(" ").collect();
            let on = split_line[0] == "on";
            let ranges: Vec<(i32,i32)> = line.split(",")
                .map(|s| {
                    let range_str = s.split("=").nth(1).unwrap();
                    let range_entries: Vec<i32> = range_str.split("..")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();
                    (range_entries[0],range_entries[1])
                }).collect();
            Instruction::new(on, &ranges[0],&ranges[1],&ranges[2])
        }).collect()
}

fn has_overlap(a: &Instruction, b: &Instruction) -> bool {
    a.x_range.1 >= b.x_range.0 && b.x_range.1 >= a.x_range.0 &&
    a.y_range.1 >= b.y_range.0 && b.y_range.1 >= a.y_range.0 &&
    a.z_range.1 >= b.z_range.0 && b.z_range.1 >= a.z_range.0     
}

/**
Run part 1 of day 22's exercise.

# Examples
```
assert_eq!(39, aoc2021::day22::run_part1("test_inputs/day22_0.txt"));
assert_eq!(590784, aoc2021::day22::run_part1("test_inputs/day22_1.txt"));
```
 */
pub fn run_part1(file: &str) -> usize {
    let mut instructions: Vec<Instruction> = load_instructions(file);
    instructions.retain(|i| i.x_range.0 >= -50 && i.x_range.1 <= 50 && i.y_range.0 >= -50 && i.y_range.1 <= 50 &&i.z_range.0 >= -50 && i.z_range.1 <= 50);
    let mut grid: HashSet<(i32,i32,i32)> = HashSet::new();
    for instruction in instructions {
        for x in instruction.x_range.0 .. instruction.x_range.1 + 1 {
            for y in instruction.y_range.0 .. instruction.y_range.1 + 1 {
                for z in instruction.z_range.0 .. instruction.z_range.1 + 1 {
                    let coord = (x,y,z);
                    if instruction.on {
                        grid.insert(coord);
                    } else {
                        grid.remove(&coord);
                    }
                }
            }
        }

    }
    grid.len()
}

/**
Find equivalent instructions of the first argument with the second argument removed.
 */
fn split_instructions(plus: &Instruction, minus: &Instruction) -> Vec<Instruction> {
    let on = true;
    let mut new: Vec<Instruction> = Vec::new();
    if minus.x_range.0-1 >= plus.x_range.0 {
        new.push(Instruction::new(on, &(plus.x_range.0, minus.x_range.0-1), &plus.y_range, &plus.z_range));
    }
    if plus.x_range.1 >= minus.x_range.1+1 {
        new.push(Instruction::new(on, &(minus.x_range.1+1, plus.x_range.1), &plus.y_range, &plus.z_range));
    }
    if minus.y_range.0-1 >= plus.y_range.0 {
        new.push(Instruction::new(on, &plus.x_range, &(plus.y_range.0, minus.y_range.0-1), &plus.z_range));
    }
    if plus.y_range.1 >= minus.y_range.1+1 {
        new.push(Instruction::new(on, &plus.x_range, &(minus.y_range.1+1, plus.y_range.1), &plus.z_range));
    }
    if minus.z_range.0-1 >= plus.z_range.0 {
        new.push(Instruction::new(on, &plus.x_range, &plus.y_range, &(plus.z_range.0, minus.z_range.0-1)));
    }
    if plus.z_range.1 >= minus.z_range.1+1 {
        new.push(Instruction::new(on, &plus.x_range, &plus.y_range, &(minus.z_range.1+1, plus.z_range.1)));
    }

    new    
}

pub fn run_part2(file: &str) -> i64 {
    let instructions: Vec<Instruction> = load_instructions(file);
    let mut overlaps: Vec<Vec<Instruction>> = Vec::new();
    for instruction in instructions {
        let mut overlapping_ndxs: Vec<usize> = Vec::new();
        for ndx in 0..overlaps.len() {
            for member in &overlaps[ndx] {
                if has_overlap(&instruction, &member) {
                    overlapping_ndxs.push(ndx);
                    break;
                }
            }
        }
        if overlapping_ndxs.len() == 0 {
            overlaps.push(vec![instruction]);
        } else {
            while overlapping_ndxs.len() > 1 {
                let remove = overlapping_ndxs.pop().unwrap();
                let keep = overlapping_ndxs[0];
                for moving_instruction in overlaps[remove].clone() {
                    overlaps[keep].push(moving_instruction);
                }
                overlaps.remove(remove);
            }
            overlaps[overlapping_ndxs[0]].push(instruction);
        }
    }
    // replace off instructions with corresponding sets of on instructions
    for set_ndx in 0..overlaps.len() {
        let mut ndx: usize = 0;
        while ndx < overlaps[set_ndx].len() {
            if !overlaps[set_ndx][ndx].on {
                // does this off instruction overlap with previous instructions?
                let mut on_ndx: usize = 0;
                while on_ndx < ndx {
                    if has_overlap(&overlaps[set_ndx][ndx], &overlaps[set_ndx][on_ndx]) {
                        let new_instr = split_instructions(&overlaps[set_ndx][on_ndx], &overlaps[set_ndx][ndx]);
                        let new_instr_len = new_instr.len();
                        overlaps[set_ndx].splice(on_ndx..on_ndx+1, new_instr);
                        on_ndx += new_instr_len;
                        ndx += new_instr_len;
                        ndx -= 1;
                    } else {
                        on_ndx += 1;
                    }
                }
                // remove this off instruction
                overlaps[set_ndx].remove(ndx);
            } else {
                ndx += 1;
            }
        }
    }
    let mut volume: i64 = 0;
    let (tx, rx) = mpsc::channel();
    let overlaps_len = overlaps.len();
    // remove all overlaps and sum remaining volumes
    while overlaps.len() > 0 {
        let mut overlap = overlaps.pop().unwrap();
        let overlaps_len_clone = overlaps_len;
        let this_overlap_number = overlaps_len - overlaps.len();
        let txcl = tx.clone();
        thread::spawn(move || {
            let mut last_instant: Instant = Instant::now();
            while overlap.len() > 0 {
                let mut updated: bool = false;
                for b_ndx in 1 .. overlap.len() {
                    //DEBUG
                    if Instant::now().duration_since(last_instant).as_secs() >= 5 {
                         println!("overlaps[{}/{}] compare 0 and {} / {}", this_overlap_number, overlaps_len_clone, b_ndx, overlap.len());
                         last_instant = Instant::now();
                    }
                    //END DEBUG
                    if has_overlap(&overlap[0], &overlap[b_ndx]) {
                        updated = true;
                        let x_range = (
                            overlap[0].x_range.0.max(overlap[b_ndx].x_range.0),
                            overlap[0].x_range.1.min(overlap[b_ndx].x_range.1),
                        );
                        let y_range = (
                            overlap[0].y_range.0.max(overlap[b_ndx].y_range.0),
                            overlap[0].y_range.1.min(overlap[b_ndx].y_range.1),
                        );
                        let z_range = (
                            overlap[0].z_range.0.max(overlap[b_ndx].z_range.0),
                            overlap[0].z_range.1.min(overlap[b_ndx].z_range.1),
                        );
                        let mut new = vec![Instruction::new(true, &x_range, &y_range, &z_range)];
                        new.extend(split_instructions(&overlap[0], &overlap[b_ndx]).iter());
                        new.extend(split_instructions(&overlap[b_ndx], &overlap[0]).iter());
                        overlap.remove(b_ndx);
                        overlap.remove(0);
                        overlap.extend(new.iter());
                        break;
                    }
                }
                if !updated {
                    txcl.send(overlap[0].volume()).unwrap();
                    overlap.remove(0);
                }
            }
        });
    };
    drop(tx);

    let mut last_instant: Instant = Instant::now();
    for vol in rx {
        volume += vol;
        //DEBUG
        if Instant::now().duration_since(last_instant).as_secs() >= 5 {
            println!("volume {}", volume);
            last_instant = Instant::now();
        }
        //END DEBUG
    }

    volume
}
