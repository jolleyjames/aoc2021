use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;

/**
Counts the number of times the next number in the sequence increases.

# Examples

```
let v: Vec<i32> = vec![1, 10, 2, 20, 20];
let answer = aoc2021::day01::count_increases(&v);

assert_eq!(2, answer);
```
*/
pub fn count_increases(v: &Vec<i32>) -> usize {
    if v.len() < 2 {
        panic!("vec must contain at least 2 values");
    }
    let next = &v[1..];
    let prev = &v[..v.len() - 1];
    next.iter()
        .zip(prev.iter())
        .map(|(a, b)| a - b)
        .filter(|x| x > &0)
        .count()
}

/**
Reads integers from a text file into a vector.

# Examples
```
let v = vec![199,200,208,210,200,207,240,269,260,263];
assert_eq!(aoc2021::day01::load_ints("test_inputs/day01.txt").unwrap(), v);
```
*/
pub fn load_ints(file: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(file)?;
    let buf_reader = BufReader::new(file);
    let v = buf_reader.lines()
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .collect();
    Ok(v)
}

/**
Run part 1 of puzzle.

# Examples
```
let result = aoc2021::day01::run_part1("test_inputs/day01.txt");
assert_eq!(7, result);
```
*/
pub fn run_part1(file: &str) -> usize {
    count_increases(&load_ints(file).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_increases_result() {
        let v = vec![1, 10, 2, 20, 20];
        let answer = count_increases(&v);

        assert_eq!(2, answer);
    }

    #[test]
    #[should_panic]
    fn count_increases_empty_vec() {
        let v = vec![];
        let _ = count_increases(&v);
    }

    #[test]
    #[should_panic]
    fn count_increases_one_item_vec() {
        let v = vec![42];
        let _ = count_increases(&v);
    }

    #[test]
    fn load_ints_result() {
        let expected = vec![199,200,208,210,200,207,240,269,260,263];
        assert_eq!(load_ints("test_inputs/day01.txt").unwrap(), expected);
    }

    #[test]
    #[should_panic]
    fn load_ints_filepanic() {
        let _ = load_ints("this is not a file").unwrap();
    }

    #[test]
    #[should_panic]
    fn load_ints_parsepanic() {
        let _ = load_ints("test_inputs/day01_bad.txt").unwrap();
    }

    #[test]
    fn run_part1_ut() {
        let result = run_part1("test_inputs/day01.txt");
        assert_eq!(7, result);
    }
}
