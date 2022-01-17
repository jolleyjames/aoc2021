use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
use itertools::Itertools;

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

fn next_roll(rolls: &mut Vec<u8>, is_winner: bool) {
    if is_winner {
        *rolls.iter_mut().nth_back(0).unwrap() += 1;
    } else {
        rolls.push(3);
    }
    while rolls.len() > 0 && rolls.iter().nth_back(0).unwrap() >= &10 {
        rolls.remove(rolls.len()-1);
        if rolls.len() > 0 {
            *rolls.iter_mut().nth_back(0).unwrap() += 1;
        }
    }

}

/**
Run part 2 of Day 21's exercise.

# Examples
```
assert_eq!(444356092776315, aoc2021::day21::run_part2("test_inputs/day21.txt"));
```
 */
pub fn run_part2(file: &str) -> u64 {
    let (a_start, b_start) = load_start_pos(file);
    // find all totals from 3 rolls of the 3-sided die
    let mut totals: Vec<u8> = Vec::new();
    for a in 1..4 {
        for b in 1..4 {
            for c in 1..4 {
                totals.push((a+b+c) as u8);
            }
        }
    }
    totals.sort();
    let totals: HashMap<u8, u64> = totals.iter()
        .group_by(|t| *t)
        .into_iter()
        .map(|t| (*t.0, t.1.collect::<Vec<&u8>>().len() as u64))
        .collect();
    // combinations of 3-rolls that produce a winner
    let mut winners_0: Vec<Vec<u8>> = Vec::new();
    let mut winners_1: Vec<Vec<u8>> = Vec::new();
    let mut rolls: Vec<u8> = vec![3];
    while rolls.len() > 0 {
        //DEBUG
        println!("- - - - -");
        println!("rolls: {:?}", rolls);
        //END DEBUG
        let mut a_score = 0;
        let mut b_score = 0;
        let mut a_pos = a_start;
        let mut b_pos = b_start;
        for i in 0..rolls.len() {
            if i%2 == 0 {
                a_pos = (a_pos+ rolls[i] as i32)%10;
                a_score += if a_pos == 0 {
                    10
                } else {
                    a_pos
                };
            } else {
                b_pos = (b_pos+ rolls[i] as i32)%10;
                b_score += if b_pos == 0 {
                    10
                } else {
                    b_pos
                };
            }
        }
        // winner?
        if 21 <= if rolls.len()%2 == 1 {
            a_score
        } else {
            b_score
        } {
            if rolls.len()%2 == 1 {
                winners_1.push(Vec::from(&rolls[..]));
            } else {
                winners_0.push(Vec::from(&rolls[..]));
            }
            next_roll(&mut rolls, true);
        } else {
            next_roll(&mut rolls, false);
        }
        //DEBUG
        println!("a_score: {}, a_pos: {}", a_score, a_pos);
        println!("a_score: {}, a_pos: {}", b_score, b_pos);
        //END DEBUG
    }
    let winners_0: u64 = winners_0.iter()
        .map(|v| v.iter().map(|tot| totals.get(tot).unwrap()).product::<u64>())
        .sum();
    let winners_1: u64 = winners_1.iter()
        .map(|v| v.iter().map(|tot| totals.get(tot).unwrap()).product::<u64>())
        .sum();
    
    winners_0.max(winners_1)
}
