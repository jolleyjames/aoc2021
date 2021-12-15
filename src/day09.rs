use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/**
Find the coordinates of the low points in this cave.

# Examples
```
use std::collections::HashMap;
let cave = HashMap::from([
    ((0,0),2),
    ((0,1),1),
    ((0,2),9),
    ((1,0),3),
    ((1,1),9),
    ((1,2),8),
    ((2,0),9),
    ((2,1),8),
    ((2,2),5),
]);
let result = aoc2021::day09::find_low_points(&cave);
assert_eq!(2, result.len());
assert!(result.contains(&&(0,1)));
assert!(result.contains(&&(2,2)));
```
*/
pub fn find_low_points<'a>(cave: &'a HashMap<(i32, i32), u8>) -> Vec<&'a (i32,i32)> {
    let mut low_points = Vec::new();
    for (coord, depth) in cave.iter() {
        let neighbors = vec![
            (coord.0 - 1, coord.1),
            (coord.0 + 1, coord.1),
            (coord.0, coord.1 - 1),
            (coord.0, coord.1 + 1),
        ];
        let higher_or_equal = neighbors.iter()
            .map(|n| cave.get(n))
            .filter(|o| match o {
                None => false,
                Some(v) => depth >= v,
            }).count();
        if higher_or_equal == 0 {
            low_points.push(coord);
        }
    }
    low_points
}

/**
Run part 1 of the Day 9 exercise.

# Examples
```
assert_eq!(15, aoc2021::day09::run_part1("test_inputs/day09.txt"));
```
 */
pub fn run_part1(file: &str) -> u32 {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);

    let mut cave = HashMap::new();
    let lines: Vec<String> = buf_reader.lines()
        .map(|line| line.unwrap())
        .collect();
    for line_number in 0..lines.len() {
        for char_number in 0..lines[line_number].len() {
            let depth = lines[line_number].chars().nth(char_number).unwrap()
                .to_digit(10).unwrap() as u8;
            cave.insert((line_number as i32, char_number as i32), depth);
        }
    }
    
    find_low_points(&cave).iter()
        .map(|coord| *cave.get(*coord).unwrap() as u32 + 1)
        .sum()
}