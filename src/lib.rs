//! # Advent of Code 2021
//!
//! `aoc2021` contains implementations to solve the daily programming challenges
//! in the 2021 version of Advent of Code.
//! 
//! See [Advent of Code 2021](https://adventofcode.com/2021)

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

pub fn run(problem: &str, args: &[String]) {
    if problem == "1" {
        println!("{}", day01::run(&args[0], args[1].parse::<usize>().unwrap()));        
    } else if problem == "2" {     
        let part = args[0].parse::<u8>().unwrap();
        let result = day02::run(&args[1], part);
        println!("{:?} (product {})", result, result.horizontal * result.depth);
    } else if problem == "3" {
        let f = if args[0] == "1" {
            day03::run_part1
        } else if args[0] == "2" {
            day03::run_part2
        } else {
            panic!("part must be 1 or 2");
        };
        println!("{}", f(&args[1]));
    } else if problem == "4" {
        println!("{}", day04::run(args[0].parse::<i32>().unwrap(), &args[1]));
    } else if problem == "5" {
        println!("{}", day05::run(args[0].parse::<i32>().unwrap(), &args[1]));
    } else if problem == "6" {
        println!("{}", day06::run(&args[0], args[1].parse::<u32>().unwrap()));
    } else if problem == "7" {
        println!("{}", day07::run(args[0].parse::<i32>().unwrap(), &args[1]));
    } else if problem == "8" {
        match args[0].as_str() {
            "1" => {println!("{}", day08::run_part1(&args[1]));},
            "2" => {println!("{}", day08::run_part2(&args[1]));},
            _ => {panic!("Unexpected part {}", args[0])},
        };
    } else if problem == "9" {
        println!("{}", day09::run_part1(&args[0]));
    }
}
