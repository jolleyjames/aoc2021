use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/**
Run part 1 of the Day 8 exercise:
Count the number output characters with a unique number of segments.

# Example
```
assert_eq!(26, aoc2021::day08::run_part1("test_inputs/day08.txt"));
```
 */
pub fn run_part1(file: &str) -> usize {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    buf_reader.lines()
        .map(|s| 
             s.unwrap().split('|').nth(1).unwrap()
              .split_whitespace()
              .filter(|s| [2,3,4,7].contains(&s.len()))
              .count())
        .sum()
}