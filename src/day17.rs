use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/**
Load the x and y coordinates of the target into a 2-tuple of 2-tuple ints.

```
assert_eq!(((20,30),(-10,-5)), aoc2021::day17::load_target_coord("test_inputs/day17.txt"));
```
 */
pub fn load_target_coord(file: &str) -> ((i32,i32),(i32,i32)) {
    let file = File::open(file).expect("could not open file");
    let mut sbuf = String::new();
    match BufReader::new(file).read_line(&mut sbuf) {
        Err(e) => {panic!("Error reading input: {:?}", e);},
        _ => (),
    };
    let sbuf = sbuf.trim();
    // trim leading "target area: "
    let s: &str = &sbuf[13..];
    let v: Vec<(i32,i32)> = s.split(", ")
        .map(|ss| {
            let vss: &Vec<&str> = &ss[2..].split('.').collect();
            (vss.iter().nth(0).unwrap().parse::<i32>().unwrap(), 
             vss.iter().nth_back(0).unwrap().parse::<i32>().unwrap())
        }).collect();
    (v[0],v[1])
}

/**
The maximum number of steps allowed s.t. the probe will be at the lowest possible
y coordinate in the target area.

# Examples
```
let (_, y_target) = aoc2021::day17::load_target_coord("test_inputs/day17.txt");
assert_eq!(20, aoc2021::day17::max_steps(y_target.0));
```
 */
pub fn max_steps(y_low: i32) -> i32 {
    -2 * y_low
}

/**
What initial velocity is needed to reach the target after the specified number of steps?
 */
pub fn init_vel(steps: i32, target: i32, ) -> f64 {
    target as f64 / steps as f64 + (steps - 1) as f64 / 2.0
}

/**
Return all starting trajectories that will end with the probe in the target
range after the specified number of steps.
 */
pub fn trajectories(steps: i32, x_range: &(i32,i32), y_range: &(i32, i32)) -> HashSet<(i32,i32)> {
    if steps == 0 {
        panic!("Expected positive number of steps");
    }
    // Find initial x velocities to reach the beginning and end of the target range
    let mut x_v_init_range: Vec<f64> = [x_range.0, x_range.1].iter()
        .map(|x| init_vel(steps, *x))
        .collect();
    // Note that the function above only works if the number of steps is less than
    // or equal to the initial velocity
    if steps as f64 > x_v_init_range[1]{
        x_v_init_range = [x_range.0, x_range.1].iter()
            .map(|x| ((1.0 + 8.0 * *x as f64).sqrt() - 1.0) / 2.0)
            .collect();
    }
    let x_v_init_range = [x_v_init_range[0].ceil() as i32, x_v_init_range[1].floor() as i32];
    let y_v_init_range: Vec<f64> = [y_range.0, y_range.1].iter()
        .map(|y| init_vel(steps, *y))
        .collect();
    let y_v_init_range = [y_v_init_range[0].ceil() as i32, y_v_init_range[1].floor() as i32];
    let mut all_trajectories = HashSet::new();
    for x_v in x_v_init_range[0] .. x_v_init_range[1]+1 {
        for y_v in y_v_init_range[0] .. y_v_init_range[1]+1 {
            all_trajectories.insert((x_v,y_v));

        }
    }
    all_trajectories
}

/**
Run part 1 of Day 17's exercise.

# Examples

```
assert_eq!(45, aoc2021::day17::run_part1("test_inputs/day17.txt"));
```
 */
pub fn run_part1(file: &str) -> i32 {
    let (_, y_target) = load_target_coord(file);
    y_target.0 * (y_target.0 + 1) / 2
}

/**
Run part 2 of Day 17's exercise.

# Examples

```
assert_eq!(112, aoc2021::day17::run_part2("test_inputs/day17.txt"));
```
 */
pub fn run_part2(file: &str) -> usize {
    let (x_target, y_target) = load_target_coord(file);
    let mut all_trajectories: HashSet<(i32,i32)> = HashSet::new();
    for steps in 1 .. max_steps(y_target.0) + 1 {
        all_trajectories.extend(trajectories(steps, &x_target, &y_target).iter());
    }
    all_trajectories.len()

}
