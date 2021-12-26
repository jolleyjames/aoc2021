use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/**
Load polymer template and transformation rules from file.

# Examples
```
let (first, last, template, rules) = aoc2021::day14::load_polymers("test_inputs/day14.txt");
assert_eq!('N', first);
assert_eq!('B', last);
assert_eq!(3, template.len());
assert_eq!(&1, template.get(&String::from("NN")).unwrap());
assert_eq!(&1, template.get(&String::from("NC")).unwrap());
assert_eq!(&1, template.get(&String::from("CB")).unwrap());
assert_eq!(16, rules.len());
assert_eq!(&vec![String::from("CB"),String::from("BH")], rules.get("CH").unwrap());
assert_eq!(&vec![String::from("CC"),String::from("CN")], rules.get("CN").unwrap());
```
 */
pub fn load_polymers(
    file: &str,
) -> (
    char,
    char,
    HashMap<String, u128>,
    HashMap<String, Vec<String>>,
) {
    let file = File::open(file).expect("could not open file");
    let mut buf_reader = BufReader::new(file);
    // load polymer template
    let mut template_string = String::new();
    match buf_reader.read_line(&mut template_string) {
        Ok(_) => (),
        Err(e) => {
            panic!("could not read polymer template: {:?}", e);
        }
    }
    let template_string = template_string.trim();
    let first_char = template_string.chars().nth(0).unwrap();
    let last_char = template_string.chars().nth_back(0).unwrap();
    let mut template: HashMap<String, u128> = HashMap::new();
    for ndx in 1..template_string.len() {
        let c = template
            .entry(String::from(&template_string[ndx - 1..ndx + 1]))
            .or_insert(0);
        *c += 1;
    }
    // load insertion rules
    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut _buf = String::new();
    match buf_reader.read_line(&mut _buf) {
        // blank line
        Err(e) => {
            panic!("error reading expected blank line: {:?}", e);
        }
        _ => (),
    }
    for line in buf_reader.lines() {
        let line_str = line.unwrap();
        let key = String::from(&line_str[0..2]);
        let mut s1 = String::new();
        s1.push(line_str.chars().nth(0).unwrap());
        s1.push(line_str.chars().nth_back(0).unwrap());
        let mut s2 = String::new();
        s2.push(line_str.chars().nth_back(0).unwrap());
        s2.push(line_str.chars().nth(1).unwrap());
        rules.insert(key, vec![s1, s2]);
    }

    (first_char, last_char, template, rules)
}

/**
Update the polymer template using the given rules.

# Examples
```
let (_, _, mut template, rules) = aoc2021::day14::load_polymers("test_inputs/day14.txt");
for _ in 0..3 {
    let new_template = aoc2021::day14::step(&template, &rules);
    template.clear();
    template.extend(new_template.iter().map(|t| (String::from(t.0), *t.1)));
}
assert_eq!(24 as u128, template.iter().map(|e| e.1).sum());
assert_eq!(&4, template.get("NB").unwrap());
assert_eq!(&4, template.get("BB").unwrap());
assert_eq!(&3, template.get("BC").unwrap());
assert_eq!(&2, template.get("CN").unwrap());
assert_eq!(&1, template.get("NC").unwrap());
assert_eq!(&1, template.get("CC").unwrap());
assert_eq!(&2, template.get("BN").unwrap());
assert_eq!(&2, template.get("CH").unwrap());
assert_eq!(&3, template.get("HB").unwrap());
assert_eq!(&1, template.get("BH").unwrap());
assert_eq!(&1, template.get("HH").unwrap());
```
 */
pub fn step(
    template: &HashMap<String, u128>,
    rules: &HashMap<String, Vec<String>>,
) -> HashMap<String, u128> {
    let mut new_template: HashMap<String, u128> = template.clone();
    for (polymer, count) in template {
        // This polymer will be replaced.
        match new_template.entry(String::from(polymer)) {
            Entry::Occupied(mut e) => {
                *e.get_mut() -= count;
            }
            Entry::Vacant(_) => {
                panic!("missing expected polymer");
            }
        }
        // These polymers will replace it.
        for new_polymer in rules.get(polymer).unwrap() {
            let c = new_template.entry(String::from(new_polymer)).or_insert(0);
            *c += count;
        }
    }

    new_template
}

/**
Count the number of times each element appears in the template.
Requires the first and last characters in the template for an accurate count.

# Examples
```
let (first, last, mut template, rules) =
    aoc2021::day14::load_polymers("test_inputs/day14.txt");
for _ in 0..10 {
    let new_template = aoc2021::day14::step(&template, &rules);
    template.clear();
    template.extend(new_template.iter().map(|t| (String::from(t.0), *t.1)));
}
let result = aoc2021::day14::element_counts(&template, first, last);
assert_eq!(4, result.len());
assert_eq!(&1749, result.get(&'B').unwrap());
assert_eq!(&298, result.get(&'C').unwrap());
assert_eq!(&161, result.get(&'H').unwrap());
assert_eq!(&865, result.get(&'N').unwrap());
```
 */
pub fn element_counts(
    template: &HashMap<String, u128>,
    first: char,
    last: char,
) -> HashMap<char, u128> {
    let mut elements = HashMap::new();
    for c in [first, last] {
        *elements.entry(c).or_insert(0) += 1;
    }
    for (polymer, count) in template {
        for c in polymer.chars() {
            *elements.entry(c).or_insert(0) += count;
        }
    }
    for count in elements.values_mut() {
        *count /= 2;
    }

    elements
}

/**
Run Day 14's exercise.

# Examples
```
assert_eq!(1588, aoc2021::day14::run("test_inputs/day14.txt", 10));
assert_eq!(2188189693529, aoc2021::day14::run("test_inputs/day14.txt", 40));
```
 */
pub fn run(file: &str, steps: u32) -> u128 {
    let (first, last, mut template, rules) = load_polymers(file);
    for _ in 0..steps {
        let new_template = step(&template, &rules);
        template.clear();
        template.extend(new_template.iter().map(|t| (String::from(t.0), *t.1)));
    }
    let elements = element_counts(&template, first, last);
    let max = elements.values().max().unwrap();
    let min = elements.values().min().unwrap();

    max-min
}
