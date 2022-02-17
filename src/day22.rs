use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Clone, Copy, Debug)]
struct Instruction {
    on: bool,
    x_range: (i32,i32),
    y_range: (i32,i32),
    z_range: (i32,i32),
}

impl Instruction {
    fn volume(&self) -> i128 {
        (1 + self.x_range.1 - self.x_range.0) as i128 *
        (1 + self.y_range.1 - self.y_range.0) as i128 *
        (1 + self.z_range.1 - self.z_range.0) as i128
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

pub fn run_part2(file: &str) -> i128 {
    let mut instructions: Vec<Instruction> = load_instructions(file);
    let mut last_instant: Instant = Instant::now();
    //DEBUG
    println!("instructions from file");
    for i in 0..instructions.len() {
        println!("[{}] {:?}", i, &instructions[i]);
    }
    //END DEBUG

    // replace off instructions with corresponding sets of on instructions
    let mut ndx: usize = 0;
    while ndx < instructions.len() {
        if !instructions[ndx].on {
            // does this off instruction overlap with previous instructions?
            let mut on_ndx: usize = 0;
            while on_ndx < ndx {
                if has_overlap(&instructions[ndx], &instructions[on_ndx]) {
                    //DEBUG
                    println!("overlap found: [{}] {:?}, [{}] {:?}", ndx, &instructions[ndx], on_ndx, &instructions[on_ndx]);
                    //END DEBUG
                    let new_instr = split_instructions(&instructions[on_ndx], &instructions[ndx]);
                    let new_instr_len = new_instr.len();
                    instructions.splice(on_ndx..on_ndx+1, new_instr);
                    on_ndx += new_instr_len;
                    ndx += new_instr_len;
                    ndx -= 1;
                } else {
                    on_ndx += 1;
                }
            }
            // remove this off instruction
            instructions.remove(ndx);
        } else {
            ndx += 1;
        }
    }
    //DEBUG
    println!("instructions after replacement");
    for i in 0..instructions.len() {
        println!("[{}] {:?}", i, &instructions[i]);
    }
    //END DEBUG
    
    let mut volume: i128 = 0;
    // remove all overlaps and sum remaining volumes
    let mut overlappers: HashSet<(usize,usize)> = HashSet::new();
    for a in 0..instructions.len()-1 {
        for b in a..instructions.len() {
            if has_overlap(&instructions[a],&instructions[b]) {
                overlappers.insert((a,b));
            }
        }
    }
    let mut combo: Vec<usize> = vec![0];
    while combo.len() > 0 {
        //DEBUG
        if Instant::now().duration_since(last_instant).as_secs() >= 10 {
            println!("combo: {:?}", combo);
            last_instant = Instant::now();
        }
        //END DEBUG
        let mut instr: Option<Instruction> = Option::Some(instructions[combo[0]]);
        for n in 1..combo.len() {
            if !overlappers.contains(&(combo[n-1],combo[n])) {
                instr = Option::None;
                break;
            } else {
                //DEBUG
                //println!("instr == {:?}", instr);
                //println!("instructions[{}] == {:?}", n, instructions[combo[n]]);                
                //END DEBUG
                let x_range = (instr.unwrap().x_range.0.max(instructions[combo[n]].x_range.0), instr.unwrap().x_range.1.min(instructions[combo[n]].x_range.1));
                let y_range = (instr.unwrap().y_range.0.max(instructions[combo[n]].y_range.0), instr.unwrap().y_range.1.min(instructions[combo[n]].y_range.1));
                let z_range = (instr.unwrap().z_range.0.max(instructions[combo[n]].z_range.0), instr.unwrap().z_range.1.min(instructions[combo[n]].z_range.1));
                if x_range.0 <= x_range.1 && y_range.0 <= y_range.1 && z_range.0 <= z_range.1 {
                    instr = Option::Some(Instruction::new(true, &x_range, &y_range, &z_range));
                    //DEBUG
                    //println!("instr.unwrap() == {:?}", instr.unwrap());
                    //END DEBUG
                } else {
                    instr = Option::None;
                    break;
                }
            }
        }
        if instr.is_some() {

            let multiplier: i128 = if combo.len()%2 == 1 {
                1
            } else {
                -1
            };
            //DEBUG
            //println!("{:?} {}", combo, instr.unwrap().volume() * multiplier);
            //END DEBUG
            let to_volume = instr.unwrap().volume() * multiplier;
            volume += to_volume;
            combo.push(combo[combo.len()-1]+1);
        } else {
            let combo_len_min1 = combo.len()-1;
            combo[combo_len_min1] += 1;
        }
        while combo.len() > 0 && combo[combo.len()-1] >= instructions.len() {
            combo.pop();
            if combo.len() > 0 {
                let combo_len_min1 = combo.len()-1;
                combo[combo_len_min1] += 1;
            }
        }
    }
    volume
}
