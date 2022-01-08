use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Side {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct BracketData {
    pub level: usize,
    pub side: Side,
    pub corresponding_ndx: usize,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct NumberData {
    pub value: u64,
}

#[derive(Copy, Clone, Debug)]
pub enum EntryType {
    Bracket(BracketData),
    Number(NumberData),
    Comma,
}

#[derive(Copy, Clone, Debug)]
pub struct Entry {
    pub data: EntryType,
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

#[derive(Debug)]
pub struct PFNumber {
    pub entries: Vec<Entry>,
    pub head: Option<usize>,
    tail: Option<usize>,
    open_entries: Vec<usize>,
}

impl PFNumber {
    /**
    The index to use to store the next new entry.
     */
    fn use_index(&mut self) -> usize {
        match self.open_entries.pop() {
            Some(ndx) => ndx,
            None => self.entries.len(),
        }
    }

    /**
    Parse string into a representation of snailfish numbers.
    ```
     */
    pub fn parse(s: &str) -> PFNumber {
        let mut entries: Vec<Entry> = Vec::new();
        let mut level: usize = 0;
        let mut brac_stac: Vec<usize> = Vec::new();
        for c in s.chars() {
            let data = match c {
                '[' => {
                    brac_stac.push(entries.len());
                    level += 1;
                    EntryType::Bracket(BracketData {
                        level: level - 1,
                        side: Side::Left,
                        corresponding_ndx: 0,
                    })
                }
                ']' => {
                    let left_brac_ndx = brac_stac.pop().unwrap();
                    level -= 1;
                    let entries_len = entries.len();
                    match &mut entries[left_brac_ndx].data {
                        EntryType::Bracket(bd) => {
                            bd.corresponding_ndx = entries_len;
                        }
                        _ => {
                            panic!("Expected bracket");
                        }
                    };
                    EntryType::Bracket(BracketData {
                        level,
                        side: Side::Right,
                        corresponding_ndx: left_brac_ndx,
                    })
                }
                ',' => EntryType::Comma,
                _ => EntryType::Number(NumberData {
                    value: c.to_digit(10).unwrap() as u64,
                }),
            };
            match data {
                EntryType::Comma => (),
                _ => {
                    let entries_len = entries.len();
                    if entries_len != 0 {
                        entries[entries_len - 1].next = Option::Some(entries_len);
                    }
                    entries.push(Entry {
                        data,
                        prev: if entries.len() == 0 {
                            Option::None
                        } else {
                            Option::Some(entries.len() - 1)
                        },
                        next: Option::None,
                    });
                }
            }
        }

        let entries_len = entries.len();
        PFNumber {
            entries,
            head: Option::Some(0),
            tail: Option::Some(entries_len - 1),
            open_entries: Vec::new(),
        }
    }

    /**
    Reduce the PFNumber. Returns true if explodes or splits, false if it
    remains unchanged.

    # Examples
    ```
    use aoc2021::day18::PFNumber;
    let mut pfn = PFNumber::parse("[[1,2],[[3,4],5]]");
    assert!(!pfn.reduce());
    let mut pfn = PFNumber::parse("[[[[[9,8],1],2],3],4]");
    assert!(pfn.reduce());
    let expected = PFNumber::parse("[[[[0,9],2],3],4]");
    assert_eq!(expected, pfn);
    let mut pfn = PFNumber::parse("[7,[6,[5,[4,[3,2]]]]]");
    assert!(pfn.reduce());
    let expected = PFNumber::parse("[7,[6,[5,[7,0]]]]");
    assert_eq!(expected, pfn);
    let mut pfn = PFNumber::parse("[[6,[5,[4,[3,2]]]],1]");
    assert!(pfn.reduce());
    let expected = PFNumber::parse("[[6,[5,[7,0]]],3]");
    assert_eq!(expected, pfn);
    let mut pfn = PFNumber::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    assert!(pfn.reduce());
    let expected = PFNumber::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    assert_eq!(expected, pfn);
    assert!(pfn.reduce());
    let expected = PFNumber::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    assert_eq!(expected, pfn);

    let mut pfn = PFNumber::parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    assert!(pfn.reduce());
    println!("{}", pfn.to_string());
    let expected = PFNumber::parse("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
    assert_eq!(expected, pfn);
    assert!(pfn.reduce());
    println!("{}", pfn.to_string());
    assert!(pfn.reduce());
    println!("{}", pfn.to_string());
    assert!(pfn.reduce());
    println!("{}", pfn.to_string());
    let expected = PFNumber::parse("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]");
    assert_eq!(expected, pfn);
    assert!(pfn.reduce());
    println!("{}", pfn.to_string());
    let expected = PFNumber::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    assert_eq!(expected, pfn);
    assert!(!pfn.reduce());
    println!("{}", pfn.to_string());
    assert_eq!(expected, pfn);
    ```
     */
    pub fn reduce(&mut self) -> bool {
        let mut ndx = self.head;
        // look for explode
        while ndx.is_some() {
            let next_ndx = self.entries[ndx.unwrap()].next;
            let next_next_ndx = match next_ndx {
                Some(n) => self.entries[n].next,
                None => Option::None,
            };
            if next_ndx.is_some()
                && next_next_ndx.is_some()
                && match self.entries[ndx.unwrap()].data {
                    EntryType::Bracket(bd) => bd.level >= 4 && bd.side == Side::Left,
                    _ => false,
                }
                && match self.entries[next_ndx.unwrap()].data {
                    EntryType::Number(_) => true,
                    _ => false,
                }
                && match self.entries[next_next_ndx.unwrap()].data {
                    EntryType::Number(_) => true,
                    _ => false,
                }
            {
                // value left of pair is incremented by left's value.
                // find first number left of the left entry in the pair.
                let mut left_ndx = self.entries[ndx.unwrap()].prev;
                while left_ndx.is_some()
                    && match self.entries[left_ndx.unwrap()].data {
                        EntryType::Number(_) => false,
                        _ => true,
                    }
                {
                    left_ndx = self.entries[left_ndx.unwrap()].prev;
                }
                if left_ndx.is_some() {
                    let addend = match self.entries[next_ndx.unwrap()].data {
                        EntryType::Number(nd) => nd.value,
                        _ => {
                            panic!("expected left pair item to be a number");
                        }
                    };
                    match &mut self.entries[left_ndx.unwrap()].data {
                        EntryType::Number(nd) => {
                            nd.value += addend;
                        }
                        _ => {
                            panic!("expected first value left of pair to be a number");
                        }
                    };
                }
                // value right of pair is incremented by rights's value
                // find first number right of the right entry in the pair.
                let mut right_ndx = self.entries[next_next_ndx.unwrap()].next;
                while right_ndx.is_some()
                    && match self.entries[right_ndx.unwrap()].data {
                        EntryType::Number(_) => false,
                        _ => true,
                    }
                {
                    right_ndx = self.entries[right_ndx.unwrap()].next;
                }
                if right_ndx.is_some() {
                    let addend = match self.entries[next_next_ndx.unwrap()].data {
                        EntryType::Number(nd) => nd.value,
                        _ => {
                            panic!("expected right pair item to be a number");
                        }
                    };
                    match &mut self.entries[right_ndx.unwrap()].data {
                        EntryType::Number(nd) => {
                            nd.value += addend;
                        }
                        _ => {
                            panic!("expected first value right of pair to be a number");
                        }
                    };
                }
                self.entries[ndx.unwrap()].data = EntryType::Number(NumberData { value: 0 });
                // delete the next 3 entries
                for _ in 0..3 {
                    let delete_ndx = self.entries[ndx.unwrap()].next.unwrap();
                    self.entries[ndx.unwrap()].next = self.entries[delete_ndx].next;
                    self.open_entries.push(delete_ndx);
                }
                let next = self.entries[ndx.unwrap()].next.unwrap();
                self.entries[next].prev = ndx;
                self.fix_brackets();
                return true;
            }
            ndx = self.entries[ndx.unwrap()].next;
        }
        let mut ndx = self.head;
        // look for split
        while ndx.is_some() {
            if match self.entries[ndx.unwrap()].data {
                EntryType::Number(nd) => nd.value >= 10,
                _ => false,
            } {
                let value = match self.entries[ndx.unwrap()].data {
                    EntryType::Number(nd) => nd.value,
                    _ => {
                        panic!("unexpected non-number entry");
                    }
                };
                let new_left = value / 2;
                let new_right = value - new_left;
                // the bracket must be either before or after this number
                let prev_maybe_bracket = self.entries[ndx.unwrap()].prev;
                let next_maybe_bracket = self.entries[ndx.unwrap()].next;
                let bracket_ndx = match self.entries[prev_maybe_bracket.unwrap()].data {
                    EntryType::Bracket(_) => prev_maybe_bracket.unwrap(),
                    _ => next_maybe_bracket.unwrap(),
                };
                let new_level = match self.entries[bracket_ndx].data {
                    EntryType::Bracket(bd) => bd.level + 1,
                    _ => {
                        panic!("expected bracket");
                    }
                };
                // set left bracket
                self.entries[ndx.unwrap()].data = EntryType::Bracket(BracketData {
                    level: new_level,
                    side: Side::Left,
                    corresponding_ndx: 0,
                });
                let start_ndx = ndx.unwrap();
                let after_end_ndx = self.entries[start_ndx].next.unwrap();
                // add left item
                let left_item_ndx = self.use_index();
                self.entries[start_ndx].next = Option::Some(left_item_ndx);
                let left_entry = Entry {
                    data: EntryType::Number(NumberData { value: new_left }),
                    prev: Option::Some(start_ndx),
                    next: Option::None,
                };
                if left_item_ndx < self.entries.len() {
                    self.entries[left_item_ndx] = left_entry;
                } else if left_item_ndx == self.entries.len() {
                    self.entries.push(left_entry);
                } else {
                    panic!("new entry index too high");
                }
                // add right item
                let right_item_ndx = self.use_index();
                self.entries[left_item_ndx].next = Option::Some(right_item_ndx);
                let right_entry = Entry {
                    data: EntryType::Number(NumberData { value: new_right }),
                    prev: Option::Some(left_item_ndx),
                    next: Option::None,
                };
                if right_item_ndx < self.entries.len() {
                    self.entries[right_item_ndx] = right_entry;
                } else if right_item_ndx == self.entries.len() {
                    self.entries.push(right_entry);
                } else {
                    panic!("new entry index too high");
                }
                // add right bracket
                let right_brkt_ndx = self.use_index();
                self.entries[right_item_ndx].next = Option::Some(right_brkt_ndx);
                let right_brkt_entry = Entry {
                    data: EntryType::Bracket(BracketData {
                        level: new_level,
                        side: Side::Right,
                        corresponding_ndx: start_ndx,
                    }),
                    prev: Option::Some(right_item_ndx),
                    next: Option::Some(after_end_ndx),
                };
                self.entries[after_end_ndx].prev = Option::Some(right_brkt_ndx);
                if right_brkt_ndx < self.entries.len() {
                    self.entries[right_brkt_ndx] = right_brkt_entry;
                } else if right_brkt_ndx == self.entries.len() {
                    self.entries.push(right_brkt_entry);
                } else {
                    panic!("new entry index too high");
                }
                self.fix_brackets();
                return true;
            }
            ndx = self.entries[ndx.unwrap()].next;
        }
        false
    }

    fn fix_brackets(&mut self) {
        let mut next_left: usize = 0;
        let mut ndx = self.head;
        while ndx.is_some() {
            match &mut self.entries[ndx.unwrap()].data {
                EntryType::Bracket(nd) => match nd.side {
                    Side::Left => {
                        nd.level = next_left;
                        next_left += 1;
                    }
                    Side::Right => {
                        nd.level = next_left - 1;
                        next_left -= 1;
                    }
                },
                _ => (),
            };
            ndx = self.entries[ndx.unwrap()].next;
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        let mut ndx = self.head;
        while ndx.is_some() {
            let app: String = match self.entries[ndx.unwrap()].data {
                EntryType::Bracket(bd) => match bd.side {
                    Side::Left => format!("["),
                    Side::Right => format!("]"),
                },
                EntryType::Number(nd) => format!("{} ", nd.value),
                EntryType::Comma => ",".to_string(),
            };
            s.push_str(&app);
            ndx = self.entries[ndx.unwrap()].next;
        }
        s
    }

    /**
    Reduce this snailfish number until only the magnitude remains.

    # Examples
    ```
    use aoc2021::day18::PFNumber;
    assert_eq!(143, PFNumber::parse("[[1,2],[[3,4],5]]").magnitude_reduce());
    assert_eq!(1384, PFNumber::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude_reduce());
    assert_eq!(445, PFNumber::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude_reduce());
    assert_eq!(791, PFNumber::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude_reduce());
    assert_eq!(1137, PFNumber::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude_reduce());
    assert_eq!(3488, PFNumber::parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude_reduce());
    ```
     */
    pub fn magnitude_reduce(&mut self) -> u64 {
        while match self.entries[self.head.unwrap()].data {
            EntryType::Bracket(_) => true,
            _ => false,
        } {
            let mut ndx = self.head;
            loop {
                let next_ndx = self.entries[ndx.unwrap()].next;
                let next_next_ndx = match next_ndx {
                    Some(n) => self.entries[n].next,
                    None => Option::None,
                };
                if match self.entries[ndx.unwrap()].data {
                    EntryType::Bracket(bd) => bd.side == Side::Left,
                    _ => false,
                } && match self.entries[next_ndx.unwrap()].data {
                    EntryType::Number(_) => true,
                    _ => false,
                } && match self.entries[next_next_ndx.unwrap()].data {
                    EntryType::Number(_) => true,
                    _ => false,
                } {
                    let left = match self.entries[next_ndx.unwrap()].data {
                        EntryType::Number(nd) => nd.value,
                        _ => {
                            panic!("expected number")
                        }
                    };
                    let right = match self.entries[next_next_ndx.unwrap()].data {
                        EntryType::Number(nd) => nd.value,
                        _ => {
                            panic!("expected number")
                        }
                    };
                    let mag = 3 * left + 2 * right;
                    self.entries[ndx.unwrap()].data = EntryType::Number(NumberData { value: mag });
                    // delete the next 3 entries
                    for _ in 0..3 {
                        let delete_ndx = self.entries[ndx.unwrap()].next.unwrap();
                        self.entries[ndx.unwrap()].next = self.entries[delete_ndx].next;
                        self.open_entries.push(delete_ndx);
                    }
                    let next = self.entries[ndx.unwrap()].next;
                    if next.is_some() {
                        self.entries[next.unwrap()].prev = ndx;
                    }
                    self.fix_brackets();
                    break;
                }
                ndx = self.entries[ndx.unwrap()].next;
            }
        }

        match self.entries[self.head.unwrap()].data {
            EntryType::Number(nd) => nd.value,
            _ => {
                panic!("expected number");
            }
        }
    }

    /**
    Add a PFNumber to this PFNumber.

    # Examples
    ```
    use aoc2021::day18::PFNumber;
    let mut accumulator = PFNumber::parse("[1,2]");
    let addend = PFNumber::parse("[[3,4],5]");
    let expected = PFNumber::parse("[[1,2],[[3,4],5]]");
    accumulator.add_assign(&addend);
    assert_eq!(expected, accumulator);
    ```
     */
    pub fn add_assign(&mut self, other: &PFNumber) {
        // increase levels for all entries in self
        let mut ndx = self.head;
        while ndx.is_some() {
            match &mut self.entries[ndx.unwrap()].data {
                EntryType::Bracket(bd) => {
                    bd.level += 1;
                }
                _ => (),
            };
            ndx = self.entries[ndx.unwrap()].next;
        }
        // push all items from other onto self with incremented level
        let mut ndx = other.head;
        while ndx.is_some() {
            let mut data = other.entries[ndx.unwrap()].data;
            let prev = self.tail;
            let next = Option::None;
            match &mut data {
                EntryType::Bracket(bd) => {
                    bd.level += 1;
                }
                _ => (),
            };
            let entry = Entry {data, prev, next};
            let new_ndx = self.use_index();
            self.entries[self.tail.unwrap()].next = Option::Some(new_ndx);
            self.tail = Option::Some(new_ndx);
            if new_ndx < self.entries.len() {
                self.entries[new_ndx] = entry;
            } else {
                self.entries.push(entry);
            }
            ndx = other.entries[ndx.unwrap()].next;
        }
        // add leading, trailing brackets
        let entry = Entry {
            data: EntryType::Bracket(BracketData {
                level: 0,
                side: Side::Left,
                corresponding_ndx: 0,
            }),
            prev: Option::None,
            next: self.head,
        };
        let new_head = self.use_index();
        if new_head < self.entries.len() {
            self.entries[new_head] = entry;
        } else {
            self.entries.push(entry);
        }
        self.entries[self.head.unwrap()].prev = Option::Some(new_head);
        self.head = Option::Some(new_head);

        let entry = Entry {
            data: EntryType::Bracket(BracketData {
                level: 0,
                side: Side::Right,
                corresponding_ndx: 0,
            }),
            prev: self.tail,
            next: Option::None,
        };
        let new_tail = self.use_index();
        if new_tail < self.entries.len() {
            self.entries[new_tail] = entry;
        } else {
            self.entries.push(entry);
        }
        self.entries[self.tail.unwrap()].next = Option::Some(new_tail);
        self.tail = Option::Some(new_tail);

        match &mut self.entries[self.head.unwrap()].data {
            EntryType::Bracket(bd) => {
                bd.corresponding_ndx = self.tail.unwrap();
            }
            _ => {
                panic!("expected bracket at head");
            }
        };
        match &mut self.entries[self.tail.unwrap()].data {
            EntryType::Bracket(bd) => {
                bd.corresponding_ndx = self.head.unwrap();
            }
            _ => {
                panic!("expected bracket at head");
            }
        };
        self.tail = Option::Some(new_tail);
    }
}

impl PartialEq for PFNumber {
    fn eq(&self, other: &Self) -> bool {
        let mut self_ndx = self.head;
        let mut other_ndx = other.head;
        while self_ndx.is_some() && other_ndx.is_some() {
            let self_ndx_unwr = self_ndx.unwrap();
            let other_ndx_unwr = other_ndx.unwrap();
            match &self.entries[self_ndx_unwr].data {
                EntryType::Comma => {
                    match &other.entries[other_ndx_unwr].data {
                        EntryType::Comma => (),
                        _ => {
                            return false;
                        }
                    };
                }
                EntryType::Bracket(self_bd) => {
                    match &other.entries[other_ndx_unwr].data {
                        EntryType::Bracket(other_bd) => {
                            if self_bd.level != other_bd.level || self_bd.side != other_bd.side {
                                return false;
                            }
                        }
                        _ => {
                            return false;
                        }
                    };
                }
                EntryType::Number(self_nd) => {
                    match &other.entries[other_ndx_unwr].data {
                        EntryType::Number(other_nd) => {
                            if self_nd != other_nd {
                                return false;
                            }
                        }
                        _ => {
                            return false;
                        }
                    };
                }
            };
            self_ndx = self.entries[self_ndx_unwr].next;
            other_ndx = other.entries[other_ndx_unwr].next;
        }

        self_ndx.is_none() && other_ndx.is_none()
    }
}
impl Eq for PFNumber {}

/**
Run part 1 of the Day 18 exercise.

# Examples
```
assert_eq!(4140, aoc2021::day18::run_part1("test_inputs/day18.txt"));
```
 */
pub fn run_part1(file: &str) -> u64 {
    let file = File::open(file).expect("could not open file");
    let mut buf_reader = BufReader::new(file);
    let mut s: String = String::new();
    match buf_reader.read_line(&mut s) {
        Err(e) => panic!("{}", e),
        _ => (),
    };
    let s = s.trim();
    let mut pfn = PFNumber::parse(&s);
    let pf_numbers: Vec<PFNumber> = buf_reader
        .lines()
        .map(|line| PFNumber::parse(&line.unwrap()))
        .collect();
    for addend in pf_numbers.iter() {
        pfn.add_assign(addend);
        while pfn.reduce() { }
    }
    pfn.magnitude_reduce()
}

/**
Run part 2 of the Day 18 exercise.

# Examples
```
assert_eq!(3993, aoc2021::day18::run_part2("test_inputs/day18.txt"));
```
 */
pub fn run_part2(file: &str) -> u64 {
    let file = File::open(file).expect("could not open file");
    let buf_reader = BufReader::new(file);
    let pfn_strings: Vec<String> = buf_reader
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let mut combos = Vec::new();
    for a in 0..pfn_strings.len() {
        for b in 0..pfn_strings.len() {
            if a != b {
                combos.push((a,b));
            }
        }
    }
    combos.iter()
        .map(|t| {
            let mut pfn = PFNumber::parse(&pfn_strings[t.0]);
            pfn.add_assign(&PFNumber::parse(&pfn_strings[t.1]));
            while pfn.reduce() {}
            pfn.magnitude_reduce()
        })
        .max().unwrap()
}
