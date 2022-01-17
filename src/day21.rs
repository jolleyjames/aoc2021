use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn load_start_pos(file: &str) -> (i32,i32) {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    let pos: Vec<i32> = buf_reader.lines()
        .map(|s| s.unwrap().split(' ').nth_back(0).unwrap().parse::<i32>().unwrap())
        .collect();
    (pos[0],pos[1])
}

/**
Run part 1 of Day 21's exercise.

# Examples
```
assert_eq!(739785, aoc2021::day21::run_part1("test_inputs/day21.txt"));
```
 */
pub fn run_part1(file: &str) -> i32 {
    let (mut a_pos, mut b_pos) = load_start_pos(file);
    let mut rolls = 0;
    let mut a_score = 0;
    let mut b_score = 0;
    loop {
        let spaces = (9*rolls + 6) % 10;
        if rolls % 2 == 0 {
            a_pos = (a_pos + spaces) % 10;
            if a_pos == 0 {
                a_pos = 10;
            }
            a_score += a_pos;
            if a_score >= 1000 {
                return b_score * 3 * (rolls+1);
            }
        } else {
            b_pos = (b_pos + spaces) % 10;
            if b_pos == 0 {
                b_pos = 10;
            }
            b_score += b_pos;
            if b_score >= 1000 {
                return a_score * 3 * (rolls+1);
            }
        }
        rolls += 1;
    }


}

