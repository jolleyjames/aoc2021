use std::collections::{HashMap,HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/**
Load caves from file into HashMap of cave names and neighbors.
 */
pub fn load_caves(file: &str) -> HashMap<String, Vec<String>> {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    let mut caves: HashMap<String, Vec<String>> = HashMap::new();
    for line in buf_reader.lines() {
        let path: Vec<String> = line.unwrap().split('-').map(|s| String::from(s)).collect();
        for ndx in 0..2 {
            match caves.get_mut(&path[ndx]) {
                Some(v) => {v.push(path[(ndx+1)%2].clone());},
                None => {caves.insert(path[ndx].clone(), vec![path[(ndx+1)%2].clone()]);},
            }
        }
    }
    caves
}

/**
Find all unique paths from start to end.
 */
pub fn find_paths(caves: &HashMap<String, Vec<String>>) -> HashSet<Vec<String>> {
    let mut incomplete_paths: HashSet<Vec<String>> = HashSet::new();
    let mut complete_paths: HashSet<Vec<String>> = HashSet::new();
    incomplete_paths.insert(vec![String::from("start")]);

    while incomplete_paths.len() > 0 {
        let this_path = incomplete_paths.iter().next().unwrap().clone();
        incomplete_paths.remove(&this_path);
        let last_cave: String = this_path.iter().next_back().unwrap().clone();
        caves.get(&last_cave).unwrap().iter()
            .for_each(|cave| {
                if &cave.to_lowercase() == cave && this_path.contains(cave) {
                    return;
                }
                let mut new_path = this_path.clone();
                new_path.push(cave.to_string());
                if !incomplete_paths.contains(&new_path) && !complete_paths.contains(&new_path) {
                    if cave == "end" {
                        complete_paths.insert(new_path);
                    } else {
                        incomplete_paths.insert(new_path);
                    }
                }
            });
    }

    complete_paths
}

/**
Run part 1 of Day 12's exercise.

# Examples
```
assert_eq!(10, aoc2021::day12::run_part1("test_inputs/day12_1.txt"));
assert_eq!(19, aoc2021::day12::run_part1("test_inputs/day12_2.txt"));
assert_eq!(226, aoc2021::day12::run_part1("test_inputs/day12_3.txt"));
```
 */
pub fn run_part1(file: &str) -> usize {
    find_paths(&load_caves(file)).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_caves() {
        let result = load_caves("test_inputs/day12_1.txt");
        assert_eq!(6, result.len());
        match result.get("start") {
            None => panic!("expected cave named \"start\""),
            Some(v) => {
                assert_eq!(2, v.len());
                assert!(v.contains(&String::from("A")));
                assert!(v.contains(&String::from("b")));
            },
        };
        match result.get("A") {
            None => panic!("expected cave named \"A\""),
            Some(v) => {
                assert_eq!(4, v.len());
                assert!(v.contains(&String::from("start")));
                assert!(v.contains(&String::from("b")));
                assert!(v.contains(&String::from("c")));
                assert!(v.contains(&String::from("end")));
            },
        };
        match result.get("b") {
            None => panic!("expected cave named \"b\""),
            Some(v) => {
                assert_eq!(4, v.len());
                assert!(v.contains(&String::from("start")));
                assert!(v.contains(&String::from("A")));
                assert!(v.contains(&String::from("d")));
                assert!(v.contains(&String::from("end")));
            },
        };
        match result.get("c") {
            None => panic!("expected cave named \"c\""),
            Some(v) => {
                assert_eq!(1, v.len());
                assert!(v.contains(&String::from("A")));
            },
        };
        match result.get("d") {
            None => panic!("expected cave named \"d\""),
            Some(v) => {
                assert_eq!(1, v.len());
                assert!(v.contains(&String::from("b")));
            },
        };
        match result.get("end") {
            None => panic!("expected cave named \"end\""),
            Some(v) => {
                assert_eq!(2, v.len());
                assert!(v.contains(&String::from("A")));
                assert!(v.contains(&String::from("b")));
            },
        };
    }

    #[test]
    fn test_find_paths() {
        let caves = load_caves("test_inputs/day12_1.txt");
        let paths = find_paths(&caves);
        assert_eq!(10, paths.len());
        let expected_paths =
"start,A,b,A,c,A,end
start,A,b,A,end
start,A,b,end
start,A,c,A,b,A,end
start,A,c,A,b,end
start,A,c,A,end
start,A,end
start,b,A,c,A,end
start,b,A,end
start,b,end";
        let expected_paths: Vec<Vec<String>> = expected_paths.lines()
            .map(|line| 
                 line.split(',').map(|s| String::from(s)).collect()
            ).collect();
        for path in expected_paths {
            assert!(paths.contains(&path));
        }
    }
}