use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct NeighborCounter<'a> {
    coord: &'a (i32, i32),
    count: u8,
}

impl<'a> NeighborCounter<'a> {
    fn new(coord: &'a (i32, i32)) -> NeighborCounter {
        NeighborCounter { coord, count: 0 }
    }
}

impl<'a> Iterator for NeighborCounter<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= 4 {
            None
        } else {
            self.count += 1;
            match self.count {
                1 => Some((self.coord.0 - 1, self.coord.1)),
                2 => Some((self.coord.0, self.coord.1 - 1)),
                3 => Some((self.coord.0, self.coord.1 + 1)),
                4 => Some((self.coord.0 + 1, self.coord.1)),
                _ => None,
            }
        }
    }
}

/**
Load risks from the file, mapped by (row,col) coordinates.

# Examples
```
let risks = aoc2021::day15::load_risks("test_inputs/day15.txt");
assert_eq!(100, risks.len());
let expected_diag: Vec<u32> = vec![1,3,3,4,4,2,2,6,2,1];
let mut ndx: i32 = 0;
for value in expected_diag {
    assert_eq!(&value, risks.get(&(ndx,ndx)).unwrap());
    ndx += 1;
}
```
 */
pub fn load_risks(file: &str) -> HashMap<(i32, i32), u32> {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    let mut risks = HashMap::new();
    let mut row: i32 = 0;
    for line in buf_reader.lines() {
        let line_str = line.unwrap();
        for col in 0..line_str.len() {
            let risk = &line_str[col..col + 1].parse::<u32>().unwrap();
            risks.insert((row, col as i32), *risk);
        }
        row += 1;
    }
    risks
}

/**
Build a traversal graph from the risks.

# Examples
```
let risks = aoc2021::day15::load_risks("test_inputs/day15.txt");
let graph = aoc2021::day15::build_graph(&risks);
assert_eq!(360, graph.len());
assert_eq!(&9, graph.get(&(&(4,4),&(3,4))).unwrap());
assert_eq!(&3, graph.get(&(&(4,4),&(4,3))).unwrap());
assert_eq!(&1, graph.get(&(&(4,4),&(4,5))).unwrap());
assert_eq!(&1, graph.get(&(&(4,4),&(5,4))).unwrap());
```
 */
pub fn build_graph<'a>(
    risks: &'a HashMap<(i32, i32), u32>,
) -> HashMap<(&'a (i32, i32), &'a (i32, i32)), u32> {
    let mut graph = HashMap::new();
    for (coord, risk) in risks {
        for neighbor in NeighborCounter::new(coord) {
            match risks.get_key_value(&neighbor) {
                Some(n) => {
                    graph.insert((n.0, coord), *risk);
                }
                None => (),
            }
        }
    }

    graph
}

/**
Find shortest path from source to destination.

# Examples
```
use std::collections::HashSet;

let risks = aoc2021::day15::load_risks("test_inputs/day15.txt");
let graph = aoc2021::day15::build_graph(&risks);
let nodes: HashSet<&(i32,i32)> = risks.keys().collect();
assert_eq!(40, aoc2021::day15::shortest_path(&(0,0), &(9,9), &nodes, &graph));
```
 */
pub fn shortest_path(
    source: &(i32, i32),
    dest: &(i32, i32),
    nodes: &HashSet<&(i32, i32)>,
    graph: &HashMap<(&(i32, i32), &(i32, i32)), u32>,
) -> u32 {
    let mut unvisited = nodes.clone();
    let mut distances: HashMap<&(i32, i32), Option<u32>> =
        unvisited.iter().map(|coord| (*coord, None)).collect();
    distances.insert(source, Some(0));
    loop {
        // choose current node as the unvisited node with the least distance.
        let mut potential_currents: Vec<(&(i32, i32), u32)> = distances
            .iter()
            .filter(|(k, v)| unvisited.contains(*k) && v.is_some())
            .map(|(k, v)| 
                 (*k, v.unwrap() + (dest.0-k.0).abs() as u32 + (dest.1-k.1).abs() as u32))
            .collect();
        potential_currents.sort_by(|a, b| a.1.cmp(&b.1));
        
        let current = potential_currents[0].0;
        let current_distance = distances.get(potential_currents[0].0).unwrap().unwrap();
        for neighbor in NeighborCounter::new(current).filter(|n| unvisited.contains(n)) {
            match distances.get(&neighbor).unwrap() {
                None => {
                    let new_distance = current_distance + graph.get(&(current, &neighbor)).unwrap();
                    *distances.get_mut(&neighbor).unwrap() = Some(new_distance);
                }
                Some(d) => {
                    let potl_dist = current_distance + graph.get(&(current, &neighbor)).unwrap();
                    if &potl_dist < d {
                        *distances.get_mut(&neighbor).unwrap() = Some(potl_dist);
                    }
                }
            }
        }
        unvisited.remove(current);
        if !unvisited.contains(dest) {
            return distances.get(dest).unwrap().unwrap();
        }
    }
}

/**
Run the Day 15 exercise.

# Examples
```
assert_eq!(40, aoc2021::day15::run(1, "test_inputs/day15.txt"));
assert_eq!(315, aoc2021::day15::run(2, "test_inputs/day15.txt"));
```
 */
pub fn run(part: u8, file: &str) -> u32 {
    let mut risks = load_risks(file);
    if part == 2 {
        // expand risks right
        let length = risks.keys().map(|coord| coord.0).max().unwrap() + 1;
        for row in 0..length {
            for col in length..length * 5 {
                let last_risk = risks.get(&(row, col-length)).unwrap();
                let this_risk = if last_risk >= &9 {
                    1
                } else {
                    last_risk + 1
                };
                risks.insert((row, col), this_risk);
            }
        }
        // expand risks down
        for row in length..length * 5 {
            for col in 0..length * 5 {
                let last_risk = risks.get(&(row-length, col)).unwrap();
                let this_risk = if last_risk >= &9 {
                    1
                } else {
                    last_risk + 1
                };
                risks.insert((row, col), this_risk);
            }
        }
    }
    let graph = build_graph(&risks);
    let nodes: HashSet<&(i32, i32)> = risks.keys().collect();
    let max_ndx = nodes.iter().map(|coord| coord.0).max().unwrap();
    shortest_path(&(0, 0), &(max_ndx, max_ndx), &nodes, &graph)
}
