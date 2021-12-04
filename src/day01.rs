use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;

/**
Counts the number of times the next number in the sequence increases.

# Examples

```
let v: Vec<i32> = vec![1, 10, 2, 20, 20];
let answer = aoc2021::day01::count_increases(&v, 1);

assert_eq!(2, answer);
```
*/
pub fn count_increases(v: &Vec<i32>, window: usize) -> usize {
    if v.len() < window + 1{
        panic!("vec must be larger than window size");
    }
    let next = &v[window..];
    let prev = &v[..v.len() - window];
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
Run Day 1's puzzle.

# Examples
```
let result = aoc2021::day01::run("test_inputs/day01.txt", 1);
assert_eq!(7, result);
```
*/
pub fn run(file: &str, window: usize) -> usize {
    count_increases(&load_ints(file).unwrap(), window)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_increases_result() {
        let v = vec![1, 10, 2, 20, 20];
        let answer = count_increases(&v, 1);
        assert_eq!(2, answer);
    }

    #[test]
    #[should_panic]
    fn count_increases_empty_vec() {
        let v = vec![];
        let _ = count_increases(&v, 1);
    }

    #[test]
    #[should_panic]
    fn count_increases_one_item_vec() {
        let v = vec![42];
        let _ = count_increases(&v, 1);
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
    fn run_ut() {
        let result = run("test_inputs/day01.txt", 1);
        assert_eq!(7, result);
        let result = run("test_inputs/day01.txt", 3);
        assert_eq!(5, result);
    }
}
