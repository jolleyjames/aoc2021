//! # Advent of Code 2021
//!
//! `aoc2021` contains implementations to solve the daily programming challenges
//! in the 2021 version of Advent of Code.
//! 
//! See [Advent of Code 2021](https://adventofcode.com/2021)

pub mod day01;

pub fn run(problem: &str, args: &[String]) {
    if problem == "1" {
        println!("{}", day01::run(&args[0], args[1].parse::<usize>().unwrap()));        
    }
}