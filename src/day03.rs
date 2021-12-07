use ndarray::{arr1, Array1};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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
    let v: Vec<i32> = s
        .chars()
        .map(|c| match c {
            '1' => 1,
            '0' => -1,
            _ => 0,
        })
        .collect();

    arr1(&v)
}

/**
 Load a file of binary numbers into a Vec of ndarray::Arrays.
 '0's become -1, '1's become 1.
*/
pub fn file_to_varray(file: &str) -> Vec<Array1<i32>> {
    let file = File::open(file).expect("could not open file");

    BufReader::new(file)
        .lines()
        .map(|s| str_as_ndarray(&s.expect("could not read file")))
        .collect()
}

/**
Sum a slice of ndarray::Arrays.

# Examples
```
let a1 = ndarray::arr1(&[1,2,3]);
let a2 = ndarray::arr1(&[2,4,6]);
let a3 = ndarray::arr1(&[3,6,9]);
let a4 = ndarray::arr1(&[4,8,12]);
let v = vec![a1, a2, a3, a4];
let expected = ndarray::arr1(&[10,20,30]);
assert_eq!(expected, aoc2021::day03::sum_arrays(&v));
```
*/
pub fn sum_arrays(arrays: &[Array1<i32>]) -> Array1<i32> {
    let init = Array1::zeros(arrays[0].raw_dim());

    arrays.iter().fold(init, |a, b| a + b)
}

/**
From a file of binary numbers with the same number of bits, find
the difference of the number of times a '1' appears and the number
of times a '0' appears, for each bit.
*/
pub fn file_sum_as_ndarray(file: &str) -> Array1<i32> {
    sum_arrays(&file_to_varray(file))
}

/**
Transform an array back into a binary String.

# Examples
```
let a: ndarray::Array1<i32> = ndarray::arr1(&[1, -1, 2, -2, -3, 33, 0]);
let expected = "101001X";
assert_eq!(expected, aoc2021::day03::array_to_str(&a));
```
 */
pub fn array_to_str(array: &Array1<i32>) -> String {
    array
        .iter()
        .map(|n| {
            if n > &0 {
                '1'
            } else if n < &0 {
                '0'
            } else {
                'X'
            }
        })
        .collect::<String>()
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
    array_to_str(&file_sum_as_ndarray(&file))
}

pub enum Gas {
    Oxygen,
    CO2
}

/**
Find the rating.

#Examples
```
use aoc2021::day03::{file_to_varray, rating, Gas};

let v = file_to_varray("test_inputs/day03.txt");
let expected = "10111";
assert_eq!(expected, rating(&v, &Gas::Oxygen));
let expected = "01010";
assert_eq!(expected, rating(&v, &Gas::CO2));
```
 */
pub fn rating(varray: &Vec<Array1<i32>>, gas: &Gas) -> String {
    let mut mut_varray = varray.clone();
    let mut bit: usize = 0;
    let array_len: usize = varray[0].len();

    while mut_varray.len() > 1 {
        let sum = sum_arrays(&mut_varray);
        let keep: i32 = if sum[bit % array_len] >= 0 {
            match gas {
                Gas::Oxygen => 1,
                Gas::CO2 => -1,
            }
        } else {
            match gas {
                Gas::Oxygen => -1,
                Gas::CO2 => 1,
            }
        };
        mut_varray.retain(|a| a[bit % array_len] == keep);

        bit += 1;
    }
    
    array_to_str(&mut_varray[0])
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
    gamma
        .chars()
        .map(|c| match c {
            '1' => '0',
            '0' => '1',
            _ => 'X',
        })
        .collect::<String>()
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

/**
Run part 2 of the Day 3 exercise.

# Examples
```
let expected = 230;
assert_eq!(expected, aoc2021::day03::run_part2("test_inputs/day03.txt"));
```
*/
pub fn run_part2(file: &str) -> i32 {
    let v = file_to_varray(file);
    let oxygen = rating(&v, &Gas::Oxygen);
    let co2 = rating(&v, &Gas::CO2);
    let oxygen = i32::from_str_radix(&oxygen, 2);
    let co2 = i32::from_str_radix(&co2, 2);

    oxygen.unwrap() * co2.unwrap()
}
