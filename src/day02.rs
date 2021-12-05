use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum Command {
    Forward,
    Down,
    Up,
}

impl Command {
    /**
    Generate Command value from str.

    # Examples
    ```
    use aoc2021::day02::Command;

    let s = "down";
    assert_eq!(Command::Down, Command::from_str("down"));
    ```
    */
    pub fn from_str(s: &str) -> Command {
        let command = if s.eq_ignore_ascii_case("forward") {
            Command::Forward
        } else if s.eq_ignore_ascii_case("down") {
            Command::Down
        } else if s.eq_ignore_ascii_case("up") {
            Command::Up
        } else {
            panic!("Unrecognized command {}", s);
        };
        command
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Instruction {
    pub command: Command,
    pub units: i32,
}

impl Instruction {
    /**
    Generate Instruction from str.

    # Eamples
    ```
    use aoc2021::day02::{Instruction,Command};

    let s = "up 100";
    assert_eq!(Instruction {command: Command::Up, units: 100}, Instruction::from_str(s));
    ```
    */
    pub fn from_str(s: &str) -> Instruction {
        let mut split_iter = s.split(' ');
        let command = Command::from_str(split_iter.next().expect("direction missing"));
        let units = split_iter.next().expect("units missing")
            .parse::<i32>().expect("units not parsable as integer");
        Instruction { command, units }
    }
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct State {
    pub horizontal: i32,
    pub depth: i32,
    pub aim: i32,
}

impl State {
    /**
    Create new State with fields initialized to 0.

    # Examples
    ```
    use aoc2021::day02::State;
    let expected = State {horizontal: 0, depth: 0, aim: 0};
    assert_eq!(expected, State::new());
    ```
    */
    pub fn new() -> State {
        State {horizontal: 0, depth: 0, aim: 0}
    }

    /**
    Change this position in the specified direction and units.

    # Examples
    ```
    use aoc2021::day02::{State,Command,Instruction};

    let mut s = State::new();
    let i1 = Instruction{ command: Command::Forward, units: 1};
    let i2 = Instruction{ command: Command::Down, units: 2};
    let i3 = Instruction{ command: Command::Up, units: 4};
    s.travel(&i1);
    assert_eq!(s, State{horizontal: 1, depth: 0, aim: 0});
    s.travel(&i2);
    assert_eq!(s, State{horizontal: 1, depth: 2, aim: 0});
    s.travel(&i3);
    assert_eq!(s, State{horizontal: 1, depth: -2, aim: 0});
    ```
    */
    pub fn travel(&mut self, m: &Instruction) {
        match m.command {
            Command::Forward => {self.horizontal += m.units;},
            Command::Down => {self.depth += m.units;},
            Command::Up => {self.depth -= m.units;},
        };
    }
}

/**
Run part 1 of Day 2's puzzle.

# Examples
```
use aoc2021::day02::State;

let expected = State{ horizontal: 15, depth: 10, aim: 0 };
assert_eq!(expected, aoc2021::day02::run_part1("test_inputs/day02.txt"));
```
*/
pub fn run_part1(file: &str) -> State {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    
    buf_reader.lines()
        .map(|s| Instruction::from_str(&s.unwrap()))
        .fold(State::new(), 
              |mut p: State, m| {
                  p.travel(&m);
                  p
              })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_part1_result() {
        let expected = State{ horizontal: 15, depth: 10, aim: 0};
        assert_eq!(expected, run_part1("test_inputs/day02.txt"));
    }
}
