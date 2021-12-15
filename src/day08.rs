use std::collections::HashMap;
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
    buf_reader
        .lines()
        .map(|s| {
            s.unwrap()
                .split('|')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .filter(|s| [2, 3, 4, 7].contains(&s.len()))
                .count()
        })
        .sum()
}

#[derive(PartialEq, Eq, Hash)]
pub struct Display {
    segments: u8,
}

impl Display {
    /**
    Creates a new 7-segment display from a string.
     */
    pub fn new(s: &str) -> Display {
        let mut segments = 0;
        for c in s.chars() {
            match c {
                'a' => {
                    segments += 1;
                }
                'b' => {
                    segments += 2;
                }
                'c' => {
                    segments += 4;
                }
                'd' => {
                    segments += 8;
                }
                'e' => {
                    segments += 16;
                }
                'f' => {
                    segments += 32;
                }
                'g' => {
                    segments += 64;
                }
                _ => {
                    panic!("unexpected character '{}'", c);
                }
            }
        }

        Display { segments }
    }

    /**
    Creates a new 7-segment display from a u8.
     */
    pub fn from_u8(segments: u8) -> Display {
        if segments & 128 >= 128 {
            panic!("must only use 7 least significant digits");
        }
        Display { segments }
    }

    pub fn get_lit_segments_as_str(&self) -> String {
        let mut s = String::new();
        let chars = "abcdefg";
        let vals: &[u8] = &[1, 2, 4, 8, 16, 32, 64];
        for (c, val) in chars.chars().zip(vals.iter()) {
            if self.segments & val > 0 {
                s.push(c);
            }
        }

        s
    }

    pub fn get_lit_segments_as_u8(&self) -> u8 {
        self.segments
    }
}

/**
Analyze the signal to discover the mappings of each display to a digit.
 */
pub fn analyze_signal<'a>(displays: &'a [Display]) -> HashMap<&'a Display, u8> {
    if displays.len() != 10 {
        panic!("Expected 10 displays");
    }

    let mut digit_to_display_map: HashMap<u8, &Display> = HashMap::new();
    let mut five_segments: Vec<&Display> = Vec::new();
    let mut six_segments: Vec<&Display> = Vec::new();
    for display in displays {
        match display.get_lit_segments_as_str().len() {
            2 => {
                digit_to_display_map.insert(1, display);
            }
            3 => {
                digit_to_display_map.insert(7, display);
            }
            4 => {
                digit_to_display_map.insert(4, display);
            }
            5 => {
                five_segments.push(&display);
            }
            6 => {
                six_segments.push(&display);
            }
            7 => {
                digit_to_display_map.insert(8, display);
            }
            _ => {
                panic!("Illegal Display");
            }
        }
    }

    // map the scrambed segments to what they could be.
    let mut scrambled_to_potential: HashMap<char, Vec<char>> = HashMap::new();
    // The segments in the digit 1 could be c or f.
    for c in digit_to_display_map
        .get(&1)
        .unwrap()
        .get_lit_segments_as_str()
        .chars()
    {
        scrambled_to_potential.insert(c, vec!['c', 'f']);
    }
    // Compare 1 and 7:
    // The segment in 7 not in 1 must be 'a'.
    let segment = digit_to_display_map
        .get(&1)
        .unwrap()
        .get_lit_segments_as_u8()
        ^ digit_to_display_map
            .get(&7)
            .unwrap()
            .get_lit_segments_as_u8();
    let segment = Display::from_u8(segment).get_lit_segments_as_str();
    assert_eq!(1, segment.len());
    scrambled_to_potential.insert(segment.chars().nth(0).unwrap(), vec!['a']);
    // Compare 1 and 4:
    // The segments in 4 not in 1 are either b or d.
    let segment = digit_to_display_map
        .get(&1)
        .unwrap()
        .get_lit_segments_as_u8()
        ^ digit_to_display_map
            .get(&4)
            .unwrap()
            .get_lit_segments_as_u8();
    let segment = Display::from_u8(segment).get_lit_segments_as_str();
    assert_eq!(2, segment.len());
    for c in segment.chars() {
        scrambled_to_potential.insert(c, vec!['b', 'd']);
    }
    // The segments we didn't find are either e or g.
    for c in "abcdefg".chars() {
        if scrambled_to_potential.get(&c) == None {
            scrambled_to_potential.insert(c, vec!['e', 'g']);
        }
    }
    // Look at the 6-segment displays.
    // The one that doesn't have all segments in 1 is 6.
    // The present segment in 6 is f; the missing segment in 6 is c.
    let mut found_six = false;
    for display in six_segments.clone() {
        let segment = display.get_lit_segments_as_u8()
            & digit_to_display_map
                .get(&1)
                .unwrap()
                .get_lit_segments_as_u8();
        let segment = Display::from_u8(segment).get_lit_segments_as_str();
        if segment.len() == 1 {
            found_six = true;
            digit_to_display_map.insert(6, display);
            scrambled_to_potential.insert(segment.chars().nth(0).unwrap(), vec!['f']);
            for c in digit_to_display_map
                .get(&1)
                .unwrap()
                .get_lit_segments_as_str()
                .chars()
            {
                if c != segment.chars().nth(0).unwrap() {
                    scrambled_to_potential.insert(c, vec!['c']);
                }
            }
            break;
        }
    }
    assert!(found_six);
    six_segments.retain(|d| d != digit_to_display_map.get(&6).unwrap());
    assert_eq!(2, six_segments.len());
    // The remaining 6-segment digits are 0 and 9.
    // Find the non-common segments.
    let uncommon_segments =
        six_segments[0].get_lit_segments_as_u8() ^ six_segments[1].get_lit_segments_as_u8();
    let uncommon_segments = Display::from_u8(uncommon_segments).get_lit_segments_as_str();
    for c in uncommon_segments.chars() {
        match scrambled_to_potential.get(&c) {
            None => {panic!("Expected segment");},
            Some(v) => {
                // If this segment maps to b or d, it maps to d because b is in both 0 and 9.
                if v == &vec!['b', 'd'] {
                    scrambled_to_potential.insert(c, vec!['d']);
                    // The 6-display segment containing this segment is 9.
                    if six_segments[0].get_lit_segments_as_str().contains(c) {
                        digit_to_display_map.insert(9, six_segments[0]);
                    } else {
                        digit_to_display_map.insert(9, six_segments[1]);
                    }
                }
                // If this segment maps to e or g, it maps to e because g is in both 0 and 9.
                if scrambled_to_potential.get(&c).unwrap() == &vec!['e', 'g'] {
                    scrambled_to_potential.insert(c, vec!['e']);
                    // The 6-display segment containing this segment is 0.
                    if six_segments[0].get_lit_segments_as_str().contains(c) {
                        digit_to_display_map.insert(0, six_segments[0]);
                    } else {
                        digit_to_display_map.insert(0, six_segments[1]);
                    }
                }
            }
        };
    }
    // We now know which segments map to d and e.
    // The segment which maps to b or d maps to b.
    // The segment which maps to e or g maps to g.
    for c in "abcdefg".chars() {
        if scrambled_to_potential.get(&c).unwrap() == &vec!['b', 'd'] {
            scrambled_to_potential.insert(c, vec!['b']);
        } else if scrambled_to_potential.get(&c).unwrap() == &vec!['e', 'g'] {
            scrambled_to_potential.insert(c, vec!['g']);
        }
    }
    // Use the descrambled segments to discover the 5-segment digits.
    for display in five_segments {
        let mut proper_segments = String::new();
        for c in display.get_lit_segments_as_str().chars() {
            proper_segments.push(scrambled_to_potential.get(&c).unwrap()[0]);
        }
        let proper_display = Display::new(&proper_segments);
        match proper_display.get_lit_segments_as_str().as_str() {
            "acdeg" => {
                digit_to_display_map.insert(2, display);
            }
            "acdfg" => {
                digit_to_display_map.insert(3, display);
            }
            "abdfg" => {
                digit_to_display_map.insert(5, display);
            }
            _ => {
                panic!("Unexpected display");
            }
        };
    }

    assert_eq!(10, digit_to_display_map.len());
    // reverse the keys and values before returning this
    let mut display_to_digit_map = HashMap::new();
    for (key, val) in digit_to_display_map.iter() {
        display_to_digit_map.insert(*val, *key);
    }
    display_to_digit_map
}

/**
Use the solved display mappings to discover the intended output value.

# Examples
```
use std::collections::HashMap;
use aoc2021::day08::{Display,get_output};

let displays = vec![
    Display::new("a"),
    Display::new("b"),
    Display::new("c"),
    Display::new("d")    
];
let map = HashMap::from([
    (&displays[0], 1),
    (&displays[1], 2),
    (&displays[2], 3),
    (&displays[3], 4)
]);    
let output = vec![
    Display::new("d"),
    Display::new("c"),
    Display::new("a"),
    Display::new("b")
];
assert_eq!(4312, get_output(&output, &map));
```
 */
pub fn get_output(output: &[Display], display_map: &HashMap<&Display, u8>) -> u32 {
    assert_eq!(4, output.len());
    let bases: [u32; 4] = [1000, 100, 10, 1];
    output.iter()
        .map(|d| *display_map.get(&d).unwrap() as u32)
        .zip(bases)
        .map(|(a,b)| a * b)
        .sum()
}

/**
Run part 2 of the Day 8 exercise.

# Examples
```
let expected = 61229;
assert_eq!(expected, aoc2021::day08::run_part2("test_inputs/day08.txt"));
```
 */
pub fn run_part2(file: &str) -> u32 {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    
    buf_reader.lines()
        .map(|s| {
            let v: Vec<Display> = s.unwrap()
                .split(' ')
                .filter(|st| st != &"|")
                .map(|s| Display::new(s))
                .collect();
            let display_map = analyze_signal(&v[..10]);
            get_output(&v[10..], &display_map)
        }).sum()
}
