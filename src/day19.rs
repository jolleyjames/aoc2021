use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Copy, Debug)]
enum Angle {
    Zero,
    HalfPi,
    Pi,
    ThreeHalfPi,
}

fn cos(theta: Angle) -> i32 {
    match theta {
        Angle::Zero => 1,
        Angle::Pi => -1,
        _ => 0,
    }
}

fn sin(theta: Angle) -> i32 {
    match theta {
        Angle::HalfPi => 1,
        Angle::ThreeHalfPi => -1,
        _ => 0,
    }
}

fn get_matrix(alpha: Angle, beta: Angle, gamma: Angle) -> [i32; 9] {
    [
        cos(beta) * cos(gamma),
        sin(alpha) * sin(beta) * cos(gamma) - cos(alpha) * sin(gamma),
        cos(alpha) * sin(beta) * cos(gamma) + sin(alpha) * sin(gamma),
        cos(beta) * sin(gamma),
        sin(alpha) * sin(beta) * sin(gamma) + cos(alpha) * cos(gamma),
        cos(alpha) * sin(beta) * sin(gamma) - sin(alpha) * cos(gamma),
        -sin(beta),
        sin(alpha) * cos(beta),
        cos(alpha) * cos(beta),
    ]
}

fn all_rot_matrices() -> HashSet<[i32; 9]> {
    let all_angles = [Angle::Zero, Angle::HalfPi, Angle::Pi, Angle::ThreeHalfPi];
    let mut all_rots = Vec::new();
    for alpha in all_angles {
        for beta in all_angles {
            for gamma in all_angles {
                all_rots.push([alpha, beta, gamma]);
            }
        }
    }
    all_rots
        .iter()
        .map(|rot| get_matrix(rot[0], rot[1], rot[2]))
        .collect::<HashSet<[i32; 9]>>()
}

/**
 * Apply the specified rotation to the specified vector.
 */
fn rotate(m: &[i32; 9], v: &[i32; 3]) -> [i32; 3] {
    [
        m[0] * v[0] + m[1] * v[1] + m[2] * v[2],
        m[3] * v[0] + m[4] * v[1] + m[5] * v[2],
        m[6] * v[0] + m[7] * v[1] + m[8] * v[2],
    ]
}

fn beacon_diff(a: &[i32; 3], b: &[i32; 3]) -> [i32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn beacon_sum(a: &[i32; 3], b: &[i32; 3]) -> [i32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/**
 * Returns a vec of pairs of indexes which represent the same beacon found by
 * different scanners.
 */
fn find_overlap(
    a: &Vec<[i32; 3]>,
    b: &Vec<[i32; 3]>,
    rots: &HashSet<[i32; 9]>,
) -> (Option<[i32; 3]>, Vec<[i32; 3]>) {
    for m in rots.iter() {
        let rot_b: Vec<[i32; 3]> = b.iter().map(|beacon| rotate(m, beacon)).collect();
        let mut diff_map: HashMap<[i32; 3], Vec<[usize; 2]>> = HashMap::new();
        for a_ndx in 0..a.len() {
            for b_ndx in 0..b.len() {
                let diff = beacon_diff(&a[a_ndx], &rot_b[b_ndx]);
                match &mut diff_map.get_mut(&diff) {
                    Some(v) => {
                        v.push([a_ndx, b_ndx]);
                    }
                    None => {
                        diff_map.insert(diff, vec![[a_ndx, b_ndx]]);
                    }
                };
            }
        }
        for item in diff_map.iter() {
            if item.1.len() >= 12 {
                return (
                    Option::Some(*item.0),
                    rot_b.iter().map(|b| beacon_sum(item.0, b)).collect(),
                );
            }
        }
    }
    (Option::None, Vec::new())
}

fn load_scanners(file: &str) -> Vec<Vec<[i32; 3]>> {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    let mut scanners = Vec::new();
    for line in buf_reader.lines() {
        let line_unwrap = line.unwrap();
        if line_unwrap.len() > 0 {
            if line_unwrap.starts_with("--") {
                scanners.push(Vec::new());
            } else {
                let v: Vec<i32> = line_unwrap
                    .split(',')
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                scanners
                    .iter_mut()
                    .nth_back(0)
                    .unwrap()
                    .push([v[0], v[1], v[2]]);
            }
        }
    }
    scanners
}

/**
Run part 1 of the Day 19 exercise.

# Examples
```
assert_eq!(79, aoc2021::day19::run_part1("test_inputs/day19.txt"));
```
 */
pub fn run_part1(file: &str) -> usize {
    let matrices = all_rot_matrices();
    let mut scanners = load_scanners(file);
    // merge the scanners into the first scanner
    while scanners.len() > 1 {
        for a_ndx in 0..scanners.len() - 1 {
            for b_ndx in a_ndx + 1..scanners.len() {
                let (_, overlaps) = find_overlap(&scanners[a_ndx], &scanners[b_ndx], &matrices);
                if overlaps.len() > 0 {
                    // merge a and b together
                    let mut new_scanner_a: HashSet<[i32; 3]> = HashSet::new();
                    for beacon in scanners[a_ndx].iter() {
                        new_scanner_a.insert(*beacon);
                    }
                    for beacon in overlaps.iter() {
                        new_scanner_a.insert(*beacon);
                    }
                    scanners[a_ndx] = new_scanner_a
                        .iter()
                        .map(|beacon| *beacon)
                        .collect::<Vec<[i32; 3]>>();
                    scanners.remove(b_ndx);
                    break;
                }
            }
        }
    }

    scanners[0].len()
}

/**
Run part 2 of the Day 19 exercise.

# Examples
```
assert_eq!(3621, aoc2021::day19::run_part2("test_inputs/day19.txt"));
```
 */
pub fn run_part2(file: &str) -> i32 {
    let matrices = all_rot_matrices();
    let mut scanners = load_scanners(file);
    let mut scanner_distances: HashMap<(usize, usize), [i32;3]> = HashMap::new();
    let mut aligned_scanners: Vec<usize> = vec![0];
    let mut as_ndx: usize = 0;
    while as_ndx < scanners.len() {
        let a_ndx = aligned_scanners[as_ndx];
        for b_ndx in 0..scanners.len() {
            if !aligned_scanners.contains(&b_ndx) {
                let (loc, overlaps) = find_overlap(&scanners[a_ndx], &scanners[b_ndx], &matrices);
                if loc.is_some() {
                    scanners[b_ndx] = overlaps;
                    aligned_scanners.push(b_ndx);
                    scanner_distances.insert((a_ndx,b_ndx), loc.unwrap());
                }
            }
        }
        as_ndx += 1;
    }
    let mut max_man_dist: i32 = 0;
    let scanner_distances: Vec<[i32;3]> = scanner_distances.iter()
        .map(|s| *s.1)
        .collect();
    for c_ndx in 0..scanner_distances.len()-1 {
        for d_ndx in c_ndx+1..scanner_distances.len() {
            let diff = beacon_diff(&scanner_distances[c_ndx], &scanner_distances[d_ndx]);
            let man_dist: i32 = diff.iter()
                .map(|n| n.abs())
                .sum();
            max_man_dist = max_man_dist.max(man_dist);
        }
    }
    max_man_dist
}
