use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Image {
    pixels: HashSet<(i32,i32)>,
    other_pixels: bool,
}

fn load_image(file: &str) -> (HashSet<u16>, Image) {
    let file = File::open(file).expect("could not open file");
    let mut buf_reader = BufReader::new(file);

    // read image algorithm
    let mut str_buf = String::new();
    match buf_reader.read_line(&mut str_buf) {
        Err(e) => panic!("could not read line: {}", e),
        _ => (),
    };
    let mut n: u16 = 0;
    let mut alg_set: HashSet<u16> = HashSet::new();
    for c in str_buf.chars() {
        if c == '#' {
            alg_set.insert(n);
        }
        n += 1;
    }

    // read blank line
    match buf_reader.read_line(&mut str_buf) {
        Err(e) => panic!("could not read line: {}", e),
        _ => (),
    };

    // read image
    let mut row: i32 = 0;
    let mut image: HashSet<(i32,i32)> = HashSet::new();
    for line_wrapped in buf_reader.lines() {
        for t in line_wrapped.unwrap().chars().enumerate() {
            if t.1 == '#' {
                image.insert((row, t.0 as i32));
            }
        }
        row += 1;
    }

    (alg_set, Image{pixels: image, other_pixels: false})
}

fn min_max(image: &HashSet<(i32,i32)>) -> (i32,i32,i32,i32) {
    let mut min_row = 0;
    let mut max_row = 0;
    let mut min_col = 0;
    let mut max_col = 0;
    let mut started = false;
    for pix in image {
        if !started {
            started = true;
            min_row = pix.0;
            max_row = pix.0;
            min_col = pix.1;
            max_col = pix.1;
        } else {
            if pix.0 < min_row {
                min_row = pix.0;
            }
            if pix.0 > max_row {
                max_row = pix.0;
            }
            if pix.1 < min_col {
                min_col = pix.1;
            }
            if pix.1 > max_col {
                max_col = pix.1;
            }
        }
    }
    (min_row,max_row,min_col,max_col)    
}

fn enhance(image: &mut Image, alg: &HashSet<u16>) -> Image {
    let mut new_image: HashSet<(i32,i32)> = HashSet::new();
    let (mut min_row, mut max_row, mut min_col, mut max_col) = min_max(&image.pixels);
    if image.other_pixels {
        for col in min_col-1 .. max_col+2 {
            image.pixels.insert((min_row-1, col));
            image.pixels.insert((max_row+1, col));
        }
        for row in min_row .. max_row+2 {
            image.pixels.insert((row, min_col-1));
            image.pixels.insert((row, max_col+1));
        }
        min_row -= 1;
        min_col -= 1;
        max_row += 1;
        max_col += 1;
    }
    for row in min_row-1 .. max_row+2 {
        for col in min_col-1 .. max_col+2 {
            let mut ndx: u16 = 0;
            for y in row-1 .. row+2 {
                for x in col-1 .. col+2 {
                    ndx = 2*ndx + if image.pixels.contains(&(y,x)) {
                        1
                    } else if image.other_pixels && 
                              (y < min_row || y > max_row || x < min_col || x > max_col) {
                        1
                    } else {
                        0
                    };
                }
            }
            if alg.contains(&ndx) {
                new_image.insert((row,col));
            }
        }
    }
    Image { 
        pixels: new_image,
        other_pixels: alg.contains(if image.other_pixels {
            &511
        } else {
            &0
        }),
    }
}

/**
Run Day 20's exercise.

# Examples
```
assert_eq!(35, aoc2021::day20::run("test_inputs/day20.txt", 2));
assert_eq!(3351, aoc2021::day20::run("test_inputs/day20.txt", 2));
```
 */
pub fn run(file: &str, enhance_count: usize) -> usize {
    let (alg, mut image) = load_image(file);
    for _ in 0..enhance_count {
        image = enhance(&mut image, &alg);
    }
    if image.other_pixels {
        panic!("Infinite pixels");
    }
    image.pixels.len()
}
