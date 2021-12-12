use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


/**
Represents a group of lanternfish that spawn on the same day.
 */
#[derive(Copy, Clone, Debug)]
pub struct FishTimer {
    count: u128,
    days_until_spawn: i8,
}

impl FishTimer {
    pub fn new(count: u128, days_until_spawn: i8) -> FishTimer {
        FishTimer{ count, days_until_spawn }
    }

    pub fn add_fish(&mut self, new: u128) {
        self.count += new;
    }

    pub fn next_day(&mut self) -> Option<FishTimer> {
        self.days_until_spawn -= 1;
        if self.days_until_spawn == -1 {
            self.days_until_spawn = 6;
            Some(FishTimer::new(self.count, 8))
        } else {
            None
        }
    }

    pub fn merge(&mut self, other: &FishTimer) {
        if self.days_until_spawn != other.days_until_spawn {
            panic!("incompatible FishTimers");
        }
        self.count += other.count
    }

    pub fn get_count(&self) -> u128 {
        self.count
    }

    pub fn get_days_until_spawn(&self) -> i8 {
        self.days_until_spawn
    }
}

pub fn next_day(v_fish: &mut Vec<FishTimer>) {
    let mut new_fish = Vec::new();
    for fish in v_fish.iter_mut() {
        let opt_fish = fish.next_day();
        if opt_fish.is_some() {
            new_fish.push(opt_fish.unwrap());
        }
    }
    v_fish.append(&mut new_fish);
    let mut six_fish = Vec::new();
    let mut ndx: usize = 0;
    while ndx < v_fish.len() {
        if v_fish[ndx].get_days_until_spawn() == 6 {
            six_fish.push(v_fish[ndx]);
            v_fish.remove(ndx);
        } else {
            ndx += 1;
        }
    }
    while six_fish.len() > 1 {
        let mut fish_one = six_fish.remove(0);
        let fish_two = six_fish.remove(0);
        fish_one.merge(&fish_two);
        six_fish.push(fish_one);        
    }
    v_fish.append(&mut six_fish);
}

/**
Run day 6 problem.

# Examples
```
let result = aoc2021::day06::run("test_inputs/day06.txt", 18);
assert_eq!(26, result);
let result = aoc2021::day06::run("test_inputs/day06.txt", 80);
assert_eq!(5934, result);
```
 */
pub fn run(file: &str, days: u32) -> u128 {
    let file = File::open(file).expect("could not open file");
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    let result = buf_reader.read_line(&mut line);
    if result.is_err() {
        panic!("{}", result.err().unwrap());
    }
    let line = line.trim();
    let mut counts: HashMap<i8, u128> = HashMap::new();
    let v_days: Vec<i8> = line.split(',').map(|s| s.parse::<i8>().unwrap()).collect();
    for day in v_days {
        if counts.contains_key(&day) {
            counts.insert(day, *counts.get(&day).unwrap()+1);
        } else {
            counts.insert(day, 1);
        }
    }
    let mut v_fish = Vec::new();
    for (day, count) in counts.iter() {
        v_fish.push(FishTimer::new(*count, *day));
    }
    for _d in 0..days {
        next_day(&mut v_fish);
    }

    v_fish.iter().map(|f| f.get_count()).sum()
}
