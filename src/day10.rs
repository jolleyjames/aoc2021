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
    Incomplete,
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
    assert_eq!(Chunk::parse("["), ParseResult::Incomplete);
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
    assert_eq!(Chunk::parse("(<[]("), ParseResult::Incomplete);
    assert_eq!(Chunk::parse("(<[](}>[])"), ParseResult::Corrupt(5));
    assert_eq!(Chunk::parse("(<[]})>[])"), ParseResult::Corrupt(4));
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
                        return ParseResult::Incomplete;
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
                            ParseResult::Incomplete => {
                                return ParseResult::Incomplete;
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
