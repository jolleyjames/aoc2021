use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/**
Read dot coordinates and fold instructions from file.

# Examples
```
let (dots, folds) = aoc2021::day13::load_dots_and_folds("test_inputs/day13.txt");
assert_eq!(18, dots.len());
assert!(dots.contains(&(6,10)));
assert!(dots.contains(&(9,0)));
assert_eq!(folds, vec![('y',7),('x',5)]);
```
 */
pub fn load_dots_and_folds(file: &str) -> (HashSet<(u32, u32)>, Vec<(char, u32)>) {
    let mut dots = HashSet::new();
    let mut folds = Vec::new();
    let mut read_dots = true;
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let line_text = line.unwrap();
        if read_dots {
            if line_text == "" {
                read_dots = false;
            } else {
                let coord: Vec<u32> = line_text
                    .split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect();
                dots.insert((coord[0], coord[1]));
            }
        } else {
            let fold: Vec<&str> = line_text.split(' ').nth(2).unwrap().split('=').collect();
            folds.push((
                fold[0].chars().nth(0).unwrap(),
                fold[1].parse::<u32>().unwrap(),
            ));
        }
    }

    (dots, folds)
}

/**
Perform a fold on the paper containing the specified dots.

# Examples
```
let (dots, folds) = aoc2021::day13::load_dots_and_folds("test_inputs/day13.txt");
let dots = aoc2021::day13::fold(&dots, &folds[0]);
assert_eq!(17, dots.len());
assert!(dots.contains(&(0,0)));
assert!(dots.contains(&(2,0)));
assert!(dots.contains(&(3,0)));
assert!(dots.contains(&(6,0)));
assert!(dots.contains(&(9,0)));
assert!(dots.contains(&(0,1)));
assert!(dots.contains(&(4,1)));
assert!(dots.contains(&(6,2)));
assert!(dots.contains(&(10,2)));
assert!(dots.contains(&(0,3)));
assert!(dots.contains(&(4,3)));
assert!(dots.contains(&(1,4)));
assert!(dots.contains(&(3,4)));
assert!(dots.contains(&(6,4)));
assert!(dots.contains(&(8,4)));
assert!(dots.contains(&(9,4)));
assert!(dots.contains(&(10,4)));
let dots = aoc2021::day13::fold(&dots, &folds[1]);
assert_eq!(16, dots.len());
for x in 0..5 {
    for y in [0,4] {
        assert!(dots.contains(&(x,y)));
    }
}
for x in [0,4] {
    for y in 1..4 {
        assert!(dots.contains(&(x,y)));
    }
}
```
 */
pub fn fold(dots: &HashSet<(u32, u32)>, fold: &(char, u32)) -> HashSet<(u32, u32)> {
    let mut new_dots = HashSet::new();
    for coord in dots.iter() {
        if match fold.0 {
            'x' => coord.0 > fold.1,
            'y' => coord.1 > fold.1,
            _ => {
                panic!("Illegal axis {}", fold.0);
            },
        } {
            new_dots.insert(match fold.0 {
                'x' => (2 * fold.1 - coord.0, coord.1),
                'y' => (coord.0, 2 * fold.1 - coord.1),
                _ => {
                    panic!("Illegal axis {}", fold.0);
                }
            });                
        } else {
            new_dots.insert(*coord);
        }
    }
    new_dots
}

/**
Run part 1 of the Day 13 exercise.

# Examples
```
assert_eq!(17, aoc2021::day13::run_part1("test_inputs/day13.txt"));
```
 */
pub fn run_part1(file: &str) -> usize {
    let (dots, folds) = load_dots_and_folds(file);
    fold(&dots, &folds[0]).len()
}

/**
Run part 2 of the Day 13 exercise.
 */
pub fn run_part2(file: &str) -> Vec<String> {
    let (mut dots, folds) = load_dots_and_folds(file);
    for this_fold in folds {
        let new_dots = fold(&dots, &this_fold);
        dots.clear();
        dots.extend(new_dots.iter());
    }
    let mut max_x = 0;
    let mut max_y = 0;
    for coord in &dots {
        if coord.0 > max_x {
            max_x = coord.0;
        }
        if coord.1 > max_y {
            max_y = coord.1;
        }
    }
    let mut print: Vec<Vec<char>> = Vec::new();
    for _ in 0 .. max_y+1 {
        print.push(vec![' '; max_x as usize+1]);
    }
    for coord in &dots {
        print[coord.1 as usize][coord.0 as usize] = '*';
    }
    print.iter().map(|v| v.iter().collect::<String>()).collect()
}