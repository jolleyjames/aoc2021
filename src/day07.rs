use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/**
How much fuel is spent moving the crabs to the final position?

# Examples
```
let crabs = vec![1, 10, 100, 1000];
let final_pos = 20;
let expected = 19 + 10 + 80 + 980;
assert_eq!(expected, aoc2021::day07::fuel_used(&crabs, final_pos, |a:i32,b:i32| (a-b).abs()));
```
 */
pub fn fuel_used<F>(crabs_pos: &[i32], final_pos: i32, fuel_used: F ) -> i32 
    where F: Fn(i32, i32) -> i32 {
    crabs_pos.iter().map(|crab| fuel_used(*crab, final_pos)).sum()
}

/**
Return the minimum amount of fuel needed to align the crabs in
the same position, with the actual position.

# Examples
```
let crabs = vec![16,1,2,0,4,2,7,1,2,14];
assert_eq!((37,2), aoc2021::day07::align_using_min_fuel(&crabs, &|a:i32,b:i32| (a-b).abs()));
let crabs = vec![16,1,2,0,4,2,7,1,2,14];
assert_eq!((168,5), aoc2021::day07::align_using_min_fuel(&crabs, 
    &|a:i32,b:i32| (a-b).abs() * ((a-b).abs()+1) / 2));
```
 */
pub fn align_using_min_fuel<F>(crabs_pos: &[i32], f: &F) -> (i32, i32) 
    where F: Fn(i32, i32) -> i32 {
    let start = crabs_pos.iter().min().unwrap();
    let end = crabs_pos.iter().max().unwrap();
    let mut minfuel: Option<(i32,i32)> = Option::None;
    
    for pos in *start..*end+1 {
        let fuel = fuel_used(crabs_pos, pos, f);
        if minfuel.is_none() || minfuel.unwrap().0 > fuel {
            minfuel = Option::Some((fuel, pos));
        }
    }

    minfuel.unwrap()
}

/**
Run the Day 7 exercise.

# Examples
```
let expected = 37;
assert_eq!(expected, aoc2021::day07::run(1, "test_inputs/day07.txt"));
let expected = 168;
assert_eq!(expected, aoc2021::day07::run(2, "test_inputs/day07.txt"));
```
 */
pub fn run(part: i32, file: &str) -> i32 {
    let file = File::open(file).expect("could not open file");
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    match buf_reader.read_line(&mut line) {
        Err(e) => {panic!("{}", e)},
        _ => (),
    }
    
    let closure: fn(i32, i32) -> i32 = match part {
        1 => |a, b| (a-b).abs(),
        2 => |a, b| (a-b).abs() * ((a-b).abs()+1) / 2,
        _ => panic!("Unexpected part {}", part),
    };
    let crabs: Vec<i32> = line.trim().split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    
    align_using_min_fuel(&crabs, &closure).0
}
