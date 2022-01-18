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
            Instruction {
                on,
                x_range: ranges[0],
                y_range: ranges[1],
                z_range: ranges[2],
            }
        }).collect()
}

/**
Run part 1 of day 22's exercise.

# Examples
```
assert_eq!(39, aoc2021::day22::run_part1("test_inputs/day22_0.txt"));
assert_eq!(590784, aoc2021::day22::run_part1("test_inputs/day22.txt"));
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
