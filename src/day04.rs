use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/**
Convert a comma-separated string of ints to a Vec.

# Examples
```
let expected: Vec<u32> = vec![10, 1, 9, 2, 8, 3];
assert_eq!(expected, aoc2021::day04::cs_str_to_vecint("10,1,9,2,8,3"));
```
 */
pub fn cs_str_to_vecint(cs_numbers: &str) -> Vec<u32> {
    cs_numbers
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

/**
A representation of a bingo board, using BingoSpaces.
 */
pub struct BingoBoard {
    space_map_by_loc: HashMap<(usize, usize), u32>,
    space_map_by_value: HashMap<u32, (usize, usize)>,
    rows: usize,
    cols: usize,
    called: HashSet<u32>,
    winner: bool,
}

impl BingoBoard {
    /**
    Create a BingoBoard from the space-separated values in the slice of strings.
     */
    pub fn new(board_lines: &[&str]) -> BingoBoard {
        let mut space_map_by_loc = HashMap::new();
        let mut space_map_by_value = HashMap::new();
        let mut rows = 0;
        let mut cols = 0;

        let mut row_index: usize = 0;
        for line in board_lines {
            if row_index >= rows {
                rows = row_index + 1;
            }
            let v_line: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let mut col_index: usize = 0;
            for value in v_line {
                if col_index >= cols {
                    cols = col_index + 1;
                }
                space_map_by_loc.insert((row_index, col_index), value);
                space_map_by_value.insert(value, (row_index, col_index));
                col_index += 1;
            }
            row_index += 1;
        }

        BingoBoard {
            space_map_by_loc,
            space_map_by_value,
            rows,
            cols,
            called: HashSet::new(),
            winner: false,
        }
    }

    /**
    Is this board a winner?
     */
    pub fn is_winner(&self) -> bool {
        self.winner
    }

    /**
    A representation of the board: each value mapped to its (row,column) coordinate.
     */
    pub fn get_spaces(&self) -> &HashMap<u32, (usize, usize)> {
        &self.space_map_by_value
    }

    /**
    The values that have been called.
     */
    pub fn get_called(&self) -> &HashSet<u32> {
        &self.called
    }

    /**
    Accept a new value for this board. Returns true if the value caused the board to
    win, false otherwise.
     */
    pub fn accept_value(&mut self, value: u32) -> bool {
        self.called.insert(value);
        match self.space_map_by_value.get(&value) {
            Some(ndx) => {
                let coord = ndx;
                // was every value in this row called?
                let range = 0..self.cols;
                let count = range
                    .filter(|col| {
                        self.called
                            .contains(self.space_map_by_loc.get(&(coord.0, *col)).unwrap())
                    })
                    .count();
                if count == self.cols {
                    self.winner = true;
                    return true;
                }
                // was every value in this column called?
                let range = 0..self.rows;
                let count = range
                    .filter(|row| {
                        self.called
                            .contains(self.space_map_by_loc.get(&(*row, coord.1)).unwrap())
                    })
                    .count();
                if count == self.rows {
                    self.winner = true;
                    return true;
                }
            }
            None => (),
        }
        false
    }
}

/**
Run Day 4 exercise.

# Examples
```
let result = aoc2021::day04::run(1, "test_inputs/day04.txt");
assert_eq!(4512, result);
let result = aoc2021::day04::run(2, "test_inputs/day04.txt");
assert_eq!(1924, result);
```
 */
pub fn run(part: i32, file: &str) -> u32 {
    let file = File::open(file).expect("could not open file");
    let mut buf_reader = BufReader::new(file);

    let mut called = String::new();
    buf_reader.read_line(&mut called).unwrap();
    let called = cs_str_to_vecint(called.trim());

    let mut boards = Vec::new();

    let mut go = true;
    while go {
        let mut buf = String::new();
        match buf_reader.read_line(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    go = false;
                } else {
                    let mut lines: Vec<String> = Vec::new();
                    for _ in 0..5 {
                        buf.clear();
                        buf_reader.read_line(&mut buf).unwrap();
                        let input = String::from(buf.trim());
                        lines.push(input);
                    }
                    boards.push(BingoBoard::new(
                        lines
                            .iter()
                            .map(|s| s.as_str())
                            .collect::<Vec<&str>>()
                            .as_slice(),
                    ));
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    for call in called {
        for index in 0..boards.len() {
            if boards[index].accept_value(call) {
                // For part 1 return the first winning board.
                // For part 2 return the last board that wins.
                let perform_return = if part == 1 {
                    true
                } else {
                    let winner_count = boards.iter().filter(|b| b.is_winner()).count();
                    winner_count == boards.len()
                };
                if perform_return {
                    // this is the winner!
                    let sum: u32 = boards[index]
                        .get_spaces()
                        .keys()
                        .filter(|space| !boards[index].get_called().contains(space))
                        .sum();
                    return call * sum;
                }
            }
        }
    }
    panic!("Did not find winning board");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_row_winner() {
        let mut board = BingoBoard::new(&["1 2 3", "11 12 13", "21 22 23"]);
        assert!(!board.accept_value(2));
        assert!(!board.is_winner());
        assert!(!board.accept_value(3));
        assert!(!board.is_winner());
        assert!(board.accept_value(1));
        assert!(board.is_winner());
        assert!(!board.accept_value(11));
        assert!(board.is_winner());
    }

    #[test]
    fn test_game_col_winner() {
        let mut board = BingoBoard::new(&["1 2 3", "11 12 13", "21 22 23"]);
        assert!(!board.accept_value(2));
        assert!(!board.is_winner());
        assert!(!board.accept_value(12));
        assert!(!board.is_winner());
        assert!(board.accept_value(22));
        assert!(board.is_winner());
        assert!(!board.accept_value(11));
        assert!(board.is_winner());
    }
}
