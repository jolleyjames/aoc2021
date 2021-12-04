use std::env;
use aoc2021::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    run(&args[1], &args[2..]);
}
