use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Seafloor {
    max_x: usize,
    max_y: usize,
    east: HashSet<(usize,usize)>,
    south: HashSet<(usize,usize)>,
}

impl Seafloor {
    fn next_east(&self, coord: &(usize,usize)) -> (usize,usize) {
        assert!(coord.0 < self.max_y, "y coord out of range");
        assert!(coord.1 < self.max_x, "x coord out of range");
        (coord.0, (coord.1 + 1)%self.max_x)        
    }

    fn next_south(&self, coord: &(usize,usize)) -> (usize,usize) {
        assert!(coord.0 < self.max_y, "y coord out of range");
        assert!(coord.1 < self.max_x, "x coord out of range");
        ((coord.0 + 1)%self.max_y, coord.1)  
    }

    fn move_east(&mut self) {
        let movers: Vec<(usize,usize)> = self.east.iter().filter(|coord| {
            let next_coord = self.next_east(coord);
            !self.east.contains(&next_coord) && !self.south.contains(&next_coord)
        }).map(|coord| *coord).collect();
        for mover in movers {
            let next_coord = self.next_east(&mover);
            self.east.remove(&mover);
            self.east.insert(next_coord);
        }
    }

    fn move_south(&mut self) {
        let movers: Vec<(usize,usize)> = self.south.iter().filter(|coord| {
            let next_coord = self.next_south(coord);
            !self.east.contains(&next_coord) && !self.south.contains(&next_coord)
        }).map(|coord| *coord).collect();
        for mover in movers {
            let next_coord = self.next_south(&mover);
            self.south.remove(&mover);
            self.south.insert(next_coord);
        }
    }
}

fn load_seafloor(file: &str) -> Seafloor {
    let mut max_x: usize = 0;
    let mut east: HashSet<(usize,usize)> = HashSet::new();
    let mut south: HashSet<(usize,usize)> = HashSet::new();
    let mut y: usize = 0;
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let line_unwrap = line.unwrap();
        if y == 0 {
            max_x = line_unwrap.len();
        }
        for en_char in line_unwrap.chars().enumerate() {
            match en_char.1 {
                '>' => {east.insert((y, en_char.0));},
                'v' => {south.insert((y, en_char.0));},
                _ => (),
            };
        }
        y += 1;
    }
    Seafloor{ max_x, max_y: y, east, south }
}

/**
Run part 1 of the Day 25 exercise.

# Examples
```
assert_eq!(58, aoc2021::day25::run_part1("test_inputs/day25.txt"));
```
 */
pub fn run_part1(file: &str) -> u32 {
    let mut seafloor = load_seafloor(file);
    let mut count: u32 = 0;
    loop {
        let last_seafloor = seafloor.clone();
        seafloor.move_east();
        seafloor.move_south();
        count += 1;
        if last_seafloor == seafloor {
            break;
        }
    }
    count
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_seafloor() {
        let sf = load_seafloor("test_inputs/day25.txt");
        assert_eq!(10, sf.max_x);
        assert_eq!(9, sf.max_y);
        assert_eq!(26, sf.south.len());
        assert_eq!(23, sf.east.len());
    }
}