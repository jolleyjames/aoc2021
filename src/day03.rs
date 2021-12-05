use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use ndarray::{arr1, Array1};

pub fn my_first_array() {
    let v1: Array1<i32> = arr1(&[1,2,3,4,5]);
    let v2: Array1<i32> = arr1(&[10,20,30,40,50]);
    
    let v3: Array1<i32> = &v1 + &v2;
    println!("{} + {} = {}", &v1, &v2, &v3);

    let v_sum: Array1<i32> = [v1, v2, v3].iter()
        .fold(arr1(&[0,0,0,0,0]), |a, b| a+b);
    println!("v_sum == {}", &v_sum);

    let z: Array1<i32> = Array1::zeros((5,));
    println!("z == {}", z);
    
    let a = str_as_ndarray("0010101");
    println!("str_as_ndarray(\"0010101\") == {}", &a);




}

/**
Convert &str to an ndarray::Array that can be summed.
'0's become -1, '1's become 1.

# Examples
```
let s = "11010";
let expected: ndarray::Array1<i32> = ndarray::arr1(&[1, 1, -1, 1, -1]);
assert_eq!(expected, aoc2021::day03::str_as_ndarray(&s));
```
*/
pub fn str_as_ndarray(s: &str) -> Array1<i32> {
    let v: Vec<i32> = s.chars().map(|c| match c {
        '1' => 1,
        '0' => -1,
        _ => 0,
    }).collect();

    arr1(&v)
}

/**
From a file of binary numbers with the same number of bits, find
the difference of the number of times a '1' appears and the number
of times a '0' appears, for each bit.
*/
pub fn file_sum_as_ndarray(file: &str) -> Array1<i32> {
    let file = File::open(file).expect("could not open file");
    let mut iter = BufReader::new(file).lines();
    let msg = "could not read file";
    let init: Array1<i32> = str_as_ndarray(&iter.next().unwrap().expect(msg));

    iter.map(|s| str_as_ndarray(&s.expect(msg)))
        .fold(init, |a, b| a+b)
}

/**
Find the gamma rate from a file of binary numbers.

# Examples
```
let gamma = aoc2021::day03::file_as_gamma("test_inputs/day03.txt");
assert_eq!(&gamma, "10110");
```
*/
pub fn file_as_gamma(file: &str) -> String {
    file_sum_as_ndarray(&file).iter()
        .map(|n| if n > &0 {
            '1'
        } else if n < &0 {
            '0'
        } else {
            'X'
        }).collect::<String>()
}

/**
Convert a gamma rate value into an epsilon rate value by 
swapping '1's and '0's.

# Example
```
let gamma = "10110";
assert_eq!("01001", aoc2021::day03::gamma_as_epsilon(&gamma));
```
*/
pub fn gamma_as_epsilon(gamma: &str) -> String {
    gamma.chars().map(|c| match c {
        '1' => '0',
        '0' => '1',
        _ => 'X',
    }).collect::<String>()
}

/**
Run part 1 of the Day 3 exercise.

# Examples
```
let expected = 198;
assert_eq!(expected, aoc2021::day03::run_part1("test_inputs/day03.txt"));
```
*/
pub fn run_part1(file: &str) -> i32 {
    let gamma = file_as_gamma(file);
    let epsilon = gamma_as_epsilon(&gamma);
    let gamma = i32::from_str_radix(&gamma, 2);
    let epsilon = i32::from_str_radix(&epsilon, 2);

    gamma.unwrap() * epsilon.unwrap()
}
