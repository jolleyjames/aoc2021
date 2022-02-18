use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;

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
Run part 2 of the exercise.

# Examples
```
assert_eq!(39, aoc2021::day22::run_part2("test_inputs/day22_0.txt"));
assert_eq!(590784, aoc2021::day22::run_part2("test_inputs/day22_1a.txt"));
assert_eq!(2758514936282235, aoc2021::day22::run_part2("test_inputs/day22_2.txt"));
```
 */
pub fn run_part2(file: &str) -> i128 {
    let instructions: Vec<Instruction> = load_instructions(file);
    // Use these instructions to sum the total number of lit cubes.
    // The bool value indicates whether the volumne must be added (true)
    // or subtracted (false).
    let mut addends: Vec<(Instruction, bool)> = Vec::new();
    for instr in &instructions {
        let mut new_addends: Vec<(Instruction, bool)> = Vec::new();
        if instr.on {
            new_addends.push((instr.clone(), true));
        }
        for addend in &addends {
            if has_overlap(&addend.0, instr) {
                let new_instr = Instruction::new(
                    true, 
                    &(addend.0.x_range.0.max(instr.x_range.0), addend.0.x_range.1.min(instr.x_range.1)),
                    &(addend.0.y_range.0.max(instr.y_range.0), addend.0.y_range.1.min(instr.y_range.1)),
                    &(addend.0.z_range.0.max(instr.z_range.0), addend.0.z_range.1.min(instr.z_range.1))
                );
                new_addends.push((new_instr, !addend.1));
            }
        }
        addends.extend(new_addends);
    }
    let mut volume: i128 = 0;
    for addend in &addends {
        volume += addend.0.volume() * if addend.1 {
            1
        } else {
            -1
        };
    }
   
    volume
}
