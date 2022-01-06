use std::ops::AddAssign;

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
    head: Option<usize>,
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
    Push a new entry to the end of the PFNumber.
     */
    pub fn push_entry(&mut self, data: EntryType) {
        let ndx = self.use_index();
        let entry = Entry {
            data,
            prev: self.tail,
            next: Option::None,
        };
        match self.tail {
            Option::Some(tail_ndx) => {
                self.entries[tail_ndx].next = Option::Some(ndx);
            },
            _ => (),
        };
        if self.head == Option::None {
            self.head = Option::Some(ndx);
        }
        self.tail = Option::Some(ndx);
        if ndx >= self.entries.len() {
            self.entries.push(entry)
        } else {
            self.entries[ndx] = entry;
        }
    }


    /**
    Parse string into a representation of snailfish numbers.

    # Examples
    ```
    // TODO write a real test
    let pfn = aoc2021::day18::PFNumber::parse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    println!("{:#?}", pfn);
    assert!(false);
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
                        level: level-1,
                        side: Side::Left,
                        corresponding_ndx: 0,
                    })
                },
                ']' => {
                    let left_brac_ndx = brac_stac.pop().unwrap();
                    level -= 1;
                    let entries_len = entries.len();
                    match &mut entries[left_brac_ndx].data {
                        EntryType::Bracket(bd) => {bd.corresponding_ndx = entries_len;},
                        _ => {panic!("Expected bracket");},
                    };
                    EntryType::Bracket(BracketData {
                            level,
                            side: Side::Right,
                            corresponding_ndx: left_brac_ndx,
                    })
                },
                ',' => {                    
                    EntryType::Comma
                },
                _ => {
                    EntryType::Number(NumberData {
                            value: c.to_digit(10).unwrap() as u64,
                    })
                },
            };
            match data {
                EntryType::Comma => (),
                _ => {
                    let entries_len = entries.len();
                    if entries_len != 0 {
                        entries[entries_len-1].next = Option::Some(entries_len);
                    }
                    entries.push(Entry { 
                        data,
                        prev: if entries.len() == 0 {
                            Option::None
                        } else {
                            Option::Some(entries.len() - 1)
                        },
                        next: Option::None
                    });
                }
            }
        }

        let entries_len = entries.len();
        PFNumber { 
            entries, 
            head: Option::Some(0), 
            tail: Option::Some(entries_len-1), 
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
    //TODO test splits
    ```
     */
    pub fn reduce(&mut self) -> bool {
        let mut ndx = self.head;
        while ndx.is_some() {            
            let next_ndx = self.entries[ndx.unwrap()].next;
            let next_next_ndx = match next_ndx {
                Some(n) => self.entries[n].next,
                None => Option::None,
            };
            // explode?
            if next_ndx.is_some() && next_next_ndx.is_some() &&
               match self.entries[ndx.unwrap()].data {
                   EntryType::Bracket(bd) => bd.level >= 4 && bd.side == Side::Left,
                   _ => false,
               } &&
               match self.entries[next_ndx.unwrap()].data {
                EntryType::Number(_) => true,
                _ => false,
               } &&
               match self.entries[next_next_ndx.unwrap()].data {
                EntryType::Number(_) => true,
                _ => false,
               } {

                // value left of pair is incremented by left's value.
                // find first number left of the left entry in the pair.
                let mut left_ndx = self.entries[ndx.unwrap()].prev;
                while left_ndx.is_some() && match self.entries[left_ndx.unwrap()].data {
                    EntryType::Number(_) => false,
                    _ => true,
                } {
                    left_ndx = self.entries[left_ndx.unwrap()].prev;
                }
                if left_ndx.is_some() {
                    let addend = match self.entries[next_ndx.unwrap()].data {
                        EntryType::Number(nd) => nd.value,
                        _ => {panic!("expected left pair item to be a number");},
                    };
                    match &mut self.entries[left_ndx.unwrap()].data {
                        EntryType::Number(nd) => {nd.value += addend;},
                        _ => {panic!("expected first value left of pair to be a number");},
                    };
                }
                // value right of pair is incremented by rights's value
                // find first number right of the right entry in the pair.
                let mut right_ndx = self.entries[next_next_ndx.unwrap()].next;
                while right_ndx.is_some() && match self.entries[right_ndx.unwrap()].data {
                    EntryType::Number(_) => false,
                    _ => true,
                } {
                    right_ndx = self.entries[right_ndx.unwrap()].next;
                }
                if right_ndx.is_some() {
                    let addend = match self.entries[next_next_ndx.unwrap()].data {
                        EntryType::Number(nd) => nd.value,
                        _ => {panic!("expected right pair item to be a number");},
                    };
                    match &mut self.entries[right_ndx.unwrap()].data {
                        EntryType::Number(nd) => {nd.value += addend;},
                        _ => {panic!("expected first value right of pair to be a number");},
                    };
                }
                self.entries[ndx.unwrap()].data = EntryType::Number(NumberData {
                        value: 0,
                    });
                // delete the next 3 entries
                for _ in 0..3 {
                    let delete_ndx = self.entries[ndx.unwrap()].next.unwrap();
                    self.entries[ndx.unwrap()].next = self.entries[delete_ndx].next;
                    self.open_entries.push(delete_ndx);
                }
                return true;
            }
            // split?
            //TODO implement me

            // no explode? no split?
            ndx = self.entries[ndx.unwrap()].next;
        }
        false
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
                        _ => {return false;},
                    };
                },
                EntryType::Bracket(self_bd) => {
                    match &other.entries[other_ndx_unwr].data {
                        EntryType::Bracket(other_bd) => {
                            if self_bd.level != other_bd.level || self_bd.side != other_bd.side {
                                return false;
                            }
                        },
                        _ => {return false;},
                    };
                },
                EntryType::Number(self_nd) => {
                    match &other.entries[other_ndx_unwr].data {
                        EntryType::Number(other_nd) => {
                            if self_nd != other_nd {
                                return false;
                            }
                        },
                        _ => {return false;},
                    };
                },
            };
            self_ndx = self.entries[self_ndx_unwr].next;
            other_ndx = other.entries[other_ndx_unwr].next;
        }

        self_ndx.is_none() && other_ndx.is_none()
    }
}
impl Eq for PFNumber {}

impl AddAssign for PFNumber {
    /**
    Implement the += operator for PFNumber.

    # Examples
    ```
    use aoc2021::day18::PFNumber;
    let mut accumulator = PFNumber::parse("[1,2]");
    let addend = PFNumber::parse("[[3,4],5]");
    let expected = PFNumber::parse("[[1,2],[[3,4],5]]");
    accumulator += addend;
    assert_eq!(expected, accumulator);
    ```
     */
    fn add_assign(&mut self, other: Self) {
        // increase levels for all entries in self
        let mut ndx = self.head;
        while ndx.is_some() {
            match &mut self.entries[ndx.unwrap()].data {
                EntryType::Bracket(bd) => {
                    bd.level += 1;
                },
                _ => (),
            };
            ndx = self.entries[ndx.unwrap()].next;
        }
        // push all items from other onto self with incremented level
        let mut ndx = other.head;
        while ndx.is_some() {
            let mut entry = other.entries[ndx.unwrap()].data;
            match &mut entry {
                EntryType::Bracket(bd) => {bd.level += 1;},
                _ => (),
            };
            self.push_entry(entry);
            ndx = other.entries[ndx.unwrap()].next;
        }
        // add leading, trailing brackets
        let new_head = self.use_index();
        self.entries.push(Entry { 
            data: EntryType::Bracket(BracketData {
                level: 0,
                side: Side::Left,
                corresponding_ndx: 0,
            }),
            prev: Option::None,
            next: self.head,
        });
        self.entries[self.head.unwrap()].prev = Option::Some(new_head);
        self.head = Option::Some(new_head);
        let new_tail = self.use_index();
        match &mut self.entries[self.head.unwrap()].data {
            EntryType::Bracket(bd) => {bd.corresponding_ndx = new_tail;},
            _ => {panic!("expected bracket at head");},
        };
        self.entries.push(Entry { 
            data: EntryType::Bracket(BracketData {
                level: 0,
                side: Side::Right,
                corresponding_ndx: new_head,
            }),
            prev: self.tail,
            next: Option::None,
        });
        self.entries[self.tail.unwrap()].next = Option::Some(new_tail);
        self.tail = Option::Some(new_tail);
    }
}

