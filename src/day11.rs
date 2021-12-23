use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct NeighborCounter<'a> {
    coord: &'a (i32, i32),
    row: i32,
    col: i32,
}

impl<'a> NeighborCounter<'a> {
    fn new(coord: &'a (i32, i32)) -> NeighborCounter {
        NeighborCounter {
            coord,
            row: coord.0 - 1,
            col: coord.1 - 2,
        }
    }
}

impl<'a> Iterator for NeighborCounter<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.coord.0 + 1 && self.col >= self.coord.1 + 1 {
            None
        } else {
            if self.col <= self.coord.1 {
                self.col += if self.row == self.coord.0 && self.col == self.coord.1 - 1 {
                    2
                } else {
                    1
                };
            } else {
                self.row += 1;
                self.col = &self.coord.1 - 1;
            }
            Some((self.row, self.col))
        }
    }
}

pub fn step(octopuses: &mut HashMap<(i32, i32), i32>) -> u32 {
    for (_, energy) in octopuses.iter_mut() {
        *energy += 1;
    }
    let mut flash_count = 0;

    // octopuses that will flash on this step
    let mut yet_to_flash: HashSet<(i32, i32)> = octopuses
        .iter()
        .filter(|o| o.1 >= &10)
        .map(|o| *o.0)
        .collect();
    let mut flashed: HashSet<(i32, i32)> = HashSet::new();
    while yet_to_flash.len() > 0 {
        let flasher = yet_to_flash.iter().next().unwrap().clone();
        yet_to_flash.remove(&flasher);
        // increment the neighbors' energy, if they have not flashed
        let neighbors: Vec<(i32, i32)> = NeighborCounter::new(&flasher)
            .filter(|n| octopuses.contains_key(n) && !flashed.contains(n))
            .collect();
        for n in neighbors {
            *octopuses.get_mut(&n).unwrap() += 1;
            if octopuses.get(&n).unwrap() >= &10 {
                yet_to_flash.insert(n);
            }
        }
        octopuses.insert(flasher, 0);
        flashed.insert(flasher);
        flash_count += 1;
    }
    flash_count
}

pub fn file_to_octopuses(file: &str) -> HashMap<(i32,i32),i32> {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    let mut octopuses = HashMap::new();
    let mut row = 0;
    for line in buf_reader.lines() {
        match line {
            Ok(line_str) => {
                for col in 0..line_str.len() {
                    octopuses.insert((row, col as i32), line_str[col..col+1].parse::<i32>().unwrap());
                }
            },
            _ => {panic!("Error reading line");},
        };
        row += 1;
    }
    octopuses
}
/**
Run part 1 of of day 11's exercise.

# Examples
```
let expected = 1656;
assert_eq!(expected, aoc2021::day11::run_part1("test_inputs/day11.txt"));
```
 */
pub fn run_part1(file: &str) -> u32 {
    let mut octopuses = file_to_octopuses(file);
    let mut flash_count = 0;
    for _ in 0..100 {
        flash_count += step(&mut octopuses);
    }
    flash_count
}

/**
Run part 2 of of day 11's exercise.

# Examples
```
let expected = 195;
assert_eq!(expected, aoc2021::day11::run_part2("test_inputs/day11.txt"));
```
 */
pub fn run_part2(file: &str) -> u32 {
    let mut octopuses = file_to_octopuses(file);
    let mut steps = 0;
    loop {
        if octopuses.values().filter(|v| v != &&0).count() == 0 {
            return steps;
        }
        step(&mut octopuses);
        steps += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let result: Vec<(i32, i32)> = NeighborCounter::new(&(74, 90)).collect();
        let expected = vec![
            (73, 89),
            (73, 90),
            (73, 91),
            (74, 89),
            (74, 91),
            (75, 89),
            (75, 90),
            (75, 91),
        ];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_step() {
        use super::*;
        let mut octopuses: HashMap<(i32, i32), i32> = HashMap::new();
        for row in 0..5 {
            for col in 0..5 {
                octopuses.insert((row, col), 1);
            }
        }
        for row in 1..4 {
            for col in 1..4 {
                if (row, col) != (2, 2) {
                    octopuses.insert((row, col), 9);
                }
            }
        }
        assert_eq!(9, step(&mut octopuses));
        for coord in vec![(0,0),(0,4),(4,0),(4,4)] {
            assert_eq!(&3, octopuses.get(&coord).unwrap());
        }
        for coord in vec![(0,1),(0,3),(1,0),(1,4),(3,0),(3,4),(4,1),(4,3)] {
            assert_eq!(&4, octopuses.get(&coord).unwrap());
        }
        for coord in vec![(0,2),(2,0),(2,4),(4,2)] {
            assert_eq!(&5, octopuses.get(&coord).unwrap());
        }
        for row in 1..4 {
            for col in 1..4 {
                assert_eq!(&0, octopuses.get(&(row,col)).unwrap());
            }
        }
        assert_eq!(0, step(&mut octopuses));
    }
}
