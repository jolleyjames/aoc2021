use std::collections::{HashMap,HashSet};
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
Find all the coordinates in the basin containing this low point.

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
let low_point = (0,1);
let result = aoc2021::day09::find_basin(&cave, &low_point);
assert_eq!(3, result.len());
assert!(result.contains(&&(0,1)));
assert!(result.contains(&&(0,0)));
assert!(result.contains(&&(1,0)));
let low_point = (2,2);
let result = aoc2021::day09::find_basin(&cave, &low_point);
assert_eq!(3, result.len());
assert!(result.contains(&&(2,2)));
assert!(result.contains(&&(2,1)));
assert!(result.contains(&&(1,2)));
```
 */
pub fn find_basin<'a>(cave: &'a HashMap<(i32, i32), u8>, low_point: &'a(i32, i32)) -> HashSet<&'a(i32, i32)> {
    let mut basin = HashSet::new();
    let mut process_queue = vec![low_point];
    while process_queue.len() > 0 {
        let coord = process_queue.pop().unwrap();
        let coord_height = cave.get(coord).unwrap();
        let mut neighbors = Vec::new();
        for neighbor in [
            (coord.0 - 1, coord.1),
            (coord.0 + 1, coord.1),
            (coord.0, coord.1 - 1),
            (coord.0, coord.1 + 1),
        ]{
            match cave.get_key_value(&neighbor) {
                None => (),
                Some(kv) => {neighbors.push(kv.0);}
            }
        }
        process_queue.extend(neighbors.iter()
            .filter(|neighbor| {
                if basin.contains(*neighbor) {
                    false
                } else {
                    match cave.get(&neighbor) {
                        None => false,
                        Some(neighbor_height) => 
                            neighbor_height > coord_height && neighbor_height != &9

                    }
                }
            }));
        basin.insert(coord);
    }
    basin
}

/**
Run the Day 9 exercise.

# Examples
```
assert_eq!(15, aoc2021::day09::run(1, "test_inputs/day09.txt"));
assert_eq!(1134, aoc2021::day09::run(2, "test_inputs/day09.txt"));
```
 */
pub fn run(part: i32, file: &str) -> u32 {
    if part != 1 && part != 2 {
        panic!("Unexpected part {}", part);
    }
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
    
    let low_points = find_low_points(&cave);
    if part == 1 {
        return low_points.iter()
            .map(|coord| *cave.get(*coord).unwrap() as u32 + 1)
            .sum();
    }

    let mut basin_sizes: Vec<usize> = low_points.iter()
        .map(|low_point| find_basin(&cave, &low_point).len())
        .collect();
    basin_sizes.sort_by(|a,b| a.cmp(b).reverse());
    let product = basin_sizes[0] * basin_sizes[1] * basin_sizes[2];

    product as u32
}