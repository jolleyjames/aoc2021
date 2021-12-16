use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(PartialEq, Eq, Debug)]
pub struct Chunk {
    pub open_char: char,
    pub consumed: usize,
    pub subchunks: Vec<Chunk>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseResult {
    Ok(Chunk), // the chunk and the number of chars consumed
    Incomplete(i128), // the incomplete score of this chunk
    Corrupt(usize), // the index of the illegal closing character
    Empty, // if there are no chunks in the line
}

impl Chunk {
    /**
    Parse the string into a Chunk. Assumes the string starts with an opening
    character. Stops parsing when the open character closes.    
    # Examples
    ```
    use aoc2021::day10::{Chunk, ParseResult};
    assert_eq!(Chunk::parse(""), ParseResult::Empty);
    assert_eq!(Chunk::parse("["), ParseResult::Incomplete(2));
    assert_eq!(Chunk::parse("<}"), ParseResult::Corrupt(1));
    assert_eq!(Chunk::parse(">"), ParseResult::Corrupt(0));

    let expected = Chunk{open_char: '[', consumed: 2, subchunks: vec![]};
    assert_eq!(Chunk::parse("[]"), ParseResult::Ok(expected));
    let expected = Chunk{open_char: '<', consumed: 4,
       subchunks: vec![Chunk{open_char: '[', consumed: 2, subchunks: vec![]}]};
    assert_eq!(Chunk::parse("<[]>"), ParseResult::Ok(expected));
    let expected = Chunk{open_char: '<', consumed: 8,
       subchunks: vec![
           Chunk{open_char: '[', consumed: 2, subchunks: vec![]},
           Chunk{open_char: '(', consumed: 2, subchunks: vec![]},
           Chunk{open_char: '{', consumed: 2, subchunks: vec![]},
        ]};
    assert_eq!(Chunk::parse("<[](){}>"), ParseResult::Ok(expected));
    let expected = Chunk{open_char: '(', consumed: 10,
       subchunks: vec![
           Chunk{open_char: '<', consumed: 6, subchunks: vec![
               Chunk{open_char: '[', consumed: 2, subchunks: vec![]},
               Chunk{open_char: '(', consumed: 2, subchunks: vec![]},
           ]},
           Chunk{open_char: '[', consumed: 2, subchunks: vec![]},
        ]};
    assert_eq!(Chunk::parse("(<[]()>[])"), ParseResult::Ok(expected));
    let expected = Chunk{open_char: '(', consumed: 10,
       subchunks: vec![
           Chunk{open_char: '<', consumed: 6, subchunks: vec![
               Chunk{open_char: '[', consumed: 2, subchunks: vec![]},
               Chunk{open_char: '(', consumed: 2, subchunks: vec![]},
           ]},
           Chunk{open_char: '[', consumed: 2, subchunks: vec![]},
        ]};
    assert_eq!(Chunk::parse("(<[]()>[]){}<>[]()"), ParseResult::Ok(expected));
    assert_eq!(Chunk::parse("(<[]("), ParseResult::Incomplete( 5* (5*1 + 4 ) + 1 ));
    assert_eq!(Chunk::parse("(<[](}>[])"), ParseResult::Corrupt(5));
    assert_eq!(Chunk::parse("(<[]})>[])"), ParseResult::Corrupt(4));
    assert_eq!(Chunk::parse("(((({<>}<{<{<>}{[]{[]{}"), ParseResult::Incomplete(1480781));
    ```
     */
    pub fn parse(s: &str) -> ParseResult {
        let mut ndx: usize = 0;
        let mut is_open = false;
        let mut chunk = Chunk {open_char: '?', consumed: 0, subchunks: Vec::new()};

        loop {
            match s.chars().nth(ndx) {
                None => {
                    if ndx == 0 {
                        return ParseResult::Empty;
                    } else if is_open {
                        let score = match chunk.open_char {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => panic!("unexpected opening character"),
                        };
                        return ParseResult::Incomplete(score);
                    } else {
                        return ParseResult::Ok(chunk);
                    }
                },
                Some(c) => {
                    if !is_open {
                        if ")]}>".contains(c) {
                            return ParseResult::Corrupt(ndx);
                        }
                        assert!("([{<".contains(c));
                        is_open = true;
                        chunk.open_char = c;
                        chunk.consumed += 1;
                        ndx += 1;
                    } else {
                        if ")]}>".contains(c) {
                            if (c == ')' && chunk.open_char != '(') ||
                               (c == ']' && chunk.open_char != '[') ||
                               (c == '}' && chunk.open_char != '{') ||
                               (c == '>' && chunk.open_char != '<') {
                                return ParseResult::Corrupt(ndx);
                            }
                            else {
                                chunk.consumed += 1;
                                return ParseResult::Ok(chunk);
                            }
                        }
                        assert!("([{<".contains(c));
                        match Chunk::parse(&s[ndx..]) {
                            ParseResult::Ok(new_chunk) => {
                                ndx += new_chunk.consumed;
                                chunk.consumed += new_chunk.consumed;
                                chunk.subchunks.push(new_chunk);
                            },
                            ParseResult::Incomplete(score) => {
                                return ParseResult::Incomplete(5*score + match chunk.open_char {
                                    '(' => 1,
                                    '[' => 2,
                                    '{' => 3,
                                    '<' => 4,
                                    _ => panic!("unexpected opening character"),
                                });
                            },
                            ParseResult::Empty => {
                                return ParseResult::Empty;
                            },
                            ParseResult::Corrupt(corrupt_ndx) => {
                                return ParseResult::Corrupt(corrupt_ndx + ndx);
                            }
                        }
                    }
                },
            }
        }
    }
}

/**
Find the score of the corrupted lines.
# Examples
```
assert_eq!(26397, aoc2021::day10::run_part1("test_inputs/day10.txt"));
```
 */
pub fn run_part1(file: &str) -> i32 {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    buf_reader.lines()
        .map(|wrapped_line| {
            let line = wrapped_line.unwrap();
            match Chunk::parse(&line) {
                ParseResult::Corrupt(ndx) => {
                    match line.chars().nth(ndx).unwrap() {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => {panic!("unexpected illegal closing character");},
                    }
                },
                _ => 0
            }
        }).sum()
}

/**
Find the median score of the incomplete lines.
# Examples
```
assert_eq!(288957, aoc2021::day10::run_part2("test_inputs/day10.txt"));
```
 */

pub fn run_part2(file: &str) -> i128 {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    let mut scores: Vec<i128> = buf_reader.lines()
        .map(|wrapped_line| {
            let line = wrapped_line.unwrap();
            match Chunk::parse(&line) {
                ParseResult::Incomplete(score) => Some(score),
                _ => None,
            }
        }).filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .collect();
    scores.sort_by(|a,b| a.cmp(b));
    
    *scores.get(scores.len()/2 as usize).unwrap()
}
