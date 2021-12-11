use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/**
Convert string to a tuple containing two (x,y) coordinates.

# Examples
```
use std::collections::HashMap;

let expected = (1,10,100,1000);
assert_eq!(expected, aoc2021::day05::str_to_tuple("1,10 -> 100,1000"));
```
 */
pub fn str_to_tuple(s: &str) -> (i32, i32, i32, i32) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    }
    let captures = RE.captures(s).unwrap();
    let range = 1..5;
    let v: Vec<i32> = range
        .map(|n| captures.get(n).unwrap().as_str().parse::<i32>().unwrap())
        .collect();
    (v[0], v[1], v[2], v[3])
}

/**
Load the coordinates from the line into the HashMap.

# Examples
```
use std::collections::HashMap;

let mut map_counts = HashMap::new();
aoc2021::day05::line_coord_count(&mut map_counts, &(2,0,2,4));
aoc2021::day05::line_coord_count(&mut map_counts, &(2,4,2,0));
aoc2021::day05::line_coord_count(&mut map_counts, &(0,2,4,2));
aoc2021::day05::line_coord_count(&mut map_counts, &(4,2,0,2));
aoc2021::day05::line_coord_count(&mut map_counts, &(4,0,0,4));
aoc2021::day05::line_coord_count(&mut map_counts, &(0,4,4,0));
let expected = HashMap::from([
    ((2,0), 2),
    ((2,1), 2),
    ((2,2), 6),
    ((2,3), 2),
    ((2,4), 2),
    ((0,2), 2),
    ((1,2), 2),
    ((3,2), 2),
    ((4,2), 2),
    ((4,0), 2),
    ((3,1), 2),
    ((1,3), 2),
    ((0,4), 2),
]);
assert_eq!(expected, map_counts);
```
 */
pub fn line_coord_count(map_counts: &mut HashMap<(i32, i32), u32>, line: &(i32, i32, i32, i32)) {
    // horizontal line
    if line.0 == line.2 {
        let range = if line.1 < line.3 {
            line.1..(line.3 + 1)
        } else {
            line.3..(line.1 + 1)
        };
        for col in range {
            let coord = (line.0, col);
            map_counts.insert(
                coord,
                match map_counts.get(&coord) {
                    None => 1,
                    Some(c) => c + 1,
                },
            );
        }
    }
    // vertical line
    else if line.1 == line.3 {
        let range = if line.0 < line.2 {
            line.0..(line.2 + 1)
        } else {
            line.2..(line.0 + 1)
        };
        for row in range {
            let coord = (row, line.1);
            map_counts.insert(
                coord,
                match map_counts.get(&coord) {
                    None => 1,
                    Some(c) => c + 1,
                },
            );
        }
    }
    // diagonal line -- assuming all diagonals have slope of 1 or -1
    else {
        if (line.0 - line.2).abs() != (line.1 - line.3).abs() {
            panic!("Unexpected line {:?}", line);
        }
        let row_step = if line.2 > line.0 { 1 } else { -1 };
        let col_step = if line.3 > line.1 { 1 } else { -1 };
        let mut row = line.0;
        let mut col = line.1;
        loop {
            let coord = (row, col);
            map_counts.insert(
                coord,
                match map_counts.get(&coord) {
                    None => 1,
                    Some(c) => c + 1,
                },
            );
            if coord == (line.2, line.3) {
                break;
            }
            row += row_step;
            col += col_step;
        }
    }
}

/**
Run Day 5's exercise.

# Examples
```
let expected = 5;
let result = aoc2021::day05::run(1, "test_inputs/day05.txt");
assert_eq!(expected, result);
let expected = 12;
let result = aoc2021::day05::run(2, "test_inputs/day05.txt");
assert_eq!(expected, result);
```
 */
pub fn run(part: i32, file: &str) -> usize {
    let closure = if part == 1 {
        |t: &(i32, i32, i32, i32)| t.0 == t.2 || t.1 == t.3
    } else if part == 2 {
        |_: &(i32, i32, i32, i32)| true
    } else {
        panic!("part {} not implemented", part);
    };
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);

    let mut coord_counts = HashMap::new();
    buf_reader.lines().map(|s| str_to_tuple(&s.unwrap()))
        .filter(closure)
        .for_each(|t| line_coord_count(&mut coord_counts, &t));
    
    coord_counts.values().filter(|v| v >= &&2).count()

}
