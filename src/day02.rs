use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl Direction {
    /**
    Generate Direction value from str.

    # Examples
    ```
    use aoc2021::day02::Direction;

    let s = "down";
    assert_eq!(Direction::Down, Direction::from_str("down"));
    ```
    */
    pub fn from_str(s: &str) -> Direction {
        let direction = if s.eq_ignore_ascii_case("forward") {
            Direction::Forward
        } else if s.eq_ignore_ascii_case("down") {
            Direction::Down
        } else if s.eq_ignore_ascii_case("up") {
            Direction::Up
        } else {
            panic!("Unrecognized direction {}", s);
        };
        direction
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Movement {
    pub direction: Direction,
    pub units: i32,
}

impl Movement {
    /**
    Generate Movement from str.

    # Eamples
    ```
    use aoc2021::day02::{Movement,Direction};

    let s = "up 100";
    assert_eq!(Movement {direction: Direction::Up, units: 100}, Movement::from_str(s));
    ```
    */
    pub fn from_str(s: &str) -> Movement {
        let mut split_iter = s.split(' ');
        let direction = Direction::from_str(split_iter.next().expect("direction missing"));
        let units = split_iter.next().expect("units missing")
            .parse::<i32>().expect("units not parsable as integer");
        Movement { direction, units }
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Position {
    pub horizontal: i32,
    pub depth: i32,
}

impl Position {
    /**
    Change this position in the specified direction and units.

    # Examples
    ```
    use aoc2021::day02::{Position,Direction,Movement};

    let mut p = Position{horizontal: 0, depth: 0};
    let m1 = Movement{ direction: Direction::Forward, units: 1};
    let m2 = Movement{ direction: Direction::Down, units: 2};
    let m3 = Movement{ direction: Direction::Up, units: 4};
    p.travel(&m1);
    assert_eq!(p, Position{horizontal: 1, depth: 0});
    p.travel(&m2);
    assert_eq!(p, Position{horizontal: 1, depth: 2});
    p.travel(&m3);
    assert_eq!(p, Position{horizontal: 1, depth: -2});
    ```
    */
    pub fn travel(&mut self, m: &Movement) {
        match m.direction {
            Direction::Forward => {self.horizontal += m.units;},
            Direction::Down => {self.depth += m.units;},
            Direction::Up => {self.depth -= m.units;},
        };
    }
}

/**
Run part 1 of Day 2's puzzle.

# Examples
```
use aoc2021::day02::Position;

let expected = Position{ horizontal: 15, depth: 10 };
assert_eq!(expected, aoc2021::day02::run_part1("test_inputs/day02.txt"));
```
*/
pub fn run_part1(file: &str) -> Position {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    
    buf_reader.lines()
        .map(|s| Movement::from_str(&s.unwrap()))
        .fold(Position{ horizontal: 0, depth: 0}, 
              |mut p: Position, m| {
                  p.travel(&m);
                  p
              })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_part1_result() {
        let expected = Position{ horizontal: 15, depth: 10 };
        assert_eq!(expected, run_part1("test_inputs/day02.txt"));
    }
}
