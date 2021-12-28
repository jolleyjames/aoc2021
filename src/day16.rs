use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/**
Convert a hexadecimal string to binary.

# Examples
```
assert_eq!("10100101", aoc2021::day16::hex_to_bin("A5"));
```
 */
pub fn hex_to_bin(s: &str) -> String {
    s.chars().map(|c| match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => {panic!("illegal hex char {}", c);},
    }).collect::<String>()
}

#[derive(Clone, Copy, Debug)]
pub enum PacketKind {
    Literal(u64),
    Operator(OperatorKind),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OperatorKind {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

#[derive(Debug)]
pub struct Packet {
    version: u8,
    type_id: u8,
    kind: PacketKind,
    subpackets: Vec<Packet>,
}

impl Packet {
    pub fn get_version(&self) -> u8 {
        self.version
    }

    pub fn get_type_id(&self) -> u8 {
        self.type_id
    }

    pub fn get_kind(&self) -> PacketKind {
        self.kind
    }

    pub fn get_subpackets<'a>(&'a self) -> &'a Vec<Packet> {
        &self.subpackets
    }

    /**
    Build a packet from a binary string. Returns a tuple containing the
    Packet and the number of chars consumed from the string.

    # Examples
    ```
    let s = aoc2021::day16::hex_to_bin("D2FE28");
    let (packet, consumed) = aoc2021::day16::Packet::from_binary_str(&s);
    assert_eq!(6, packet.get_version());
    assert_eq!(4, packet.get_type_id());
    match packet.get_kind() {
        aoc2021::day16::PacketKind::Literal(v) => {assert_eq!(2021, v);},
        _ => {panic!("expected Literal");},
    };
    assert!(packet.get_subpackets().is_empty());
    assert_eq!(21, consumed);

    let s = aoc2021::day16::hex_to_bin("38006F45291200");
    let (packet, consumed) = aoc2021::day16::Packet::from_binary_str(&s);
    assert_eq!(1, packet.get_version());
    assert_eq!(6, packet.get_type_id());
    match packet.get_kind() {
        aoc2021::day16::PacketKind::Operator(op) => {assert_eq!(aoc2021::day16::OperatorKind::Lt, op);},
        _ => {panic!("expected Operator");},
    };
    assert_eq!(2, packet.get_subpackets().len());
    assert_eq!(10, match packet.get_subpackets()[0].get_kind() {
        aoc2021::day16::PacketKind::Literal(v) => v,
        _ => {panic!("expected Literal");},
    });
    assert_eq!(20, match packet.get_subpackets()[1].get_kind() {
        aoc2021::day16::PacketKind::Literal(v) => v,
        _ => {panic!("expected Literal");},
    });
    assert_eq!(49, consumed);

    let s = aoc2021::day16::hex_to_bin("EE00D40C823060");
    let (packet, consumed) = aoc2021::day16::Packet::from_binary_str(&s);
    assert_eq!(7, packet.get_version());
    assert_eq!(3, packet.get_type_id());
    match packet.get_kind() {
        aoc2021::day16::PacketKind::Operator(op) => {assert_eq!(aoc2021::day16::OperatorKind::Max, op);},
        _ => {panic!("expected Operator");},
    };
    assert_eq!(3, packet.get_subpackets().len());
    assert_eq!(1, match packet.get_subpackets()[0].get_kind() {
        aoc2021::day16::PacketKind::Literal(v) => v,
        _ => {panic!("expected Literal");},
    });
    assert_eq!(2, match packet.get_subpackets()[1].get_kind() {
        aoc2021::day16::PacketKind::Literal(v) => v,
        _ => {panic!("expected Literal");},
    });
    assert_eq!(3, match packet.get_subpackets()[2].get_kind() {
        aoc2021::day16::PacketKind::Literal(v) => v,
        _ => {panic!("expected Literal");},
    });
    assert_eq!(51, consumed);
    ```
     */
    pub fn from_binary_str(s: &str) -> (Packet, usize) {
        let version = u8::from_str_radix(&s[0..3], 2).unwrap();
        let type_id = u8::from_str_radix(&s[3..6], 2).unwrap();
        let kind: PacketKind;
        let mut subpackets = Vec::new();
        let mut ndx: usize = 6;
        if type_id == 4 {
            let mut value: u64 = 0;
            loop {
                value *= 16;
                value += u64::from_str_radix(&s[ndx+1..ndx+5], 2).unwrap();
                ndx += 5;
                if &s[ndx-5..ndx-4] == "0" {
                    break;
                }
            }
            kind = PacketKind::Literal(value);
        } else {
            let length_type_id = &s[ndx..ndx+1];
            ndx += 1;
            if length_type_id == "0" {
                let subpackets_length = usize::from_str_radix(&s[ndx..ndx+15], 2).unwrap();
                ndx += 15;
                let end_ndx = ndx + subpackets_length;
                while ndx < end_ndx {
                    let (subpacket, consumed) = Packet::from_binary_str(&s[ndx..end_ndx]);
                    subpackets.push(subpacket);
                    ndx += consumed;
                }
            } else {
                let subpackets_count = usize::from_str_radix(&s[ndx..ndx+11], 2).unwrap();
                ndx += 11;
                for _ in 0..subpackets_count {
                    let (subpacket, consumed) = Packet::from_binary_str(&s[ndx..]);
                    subpackets.push(subpacket);
                    ndx += consumed;
                }
            }
            kind = PacketKind::Operator(match type_id {
                0 => OperatorKind::Sum,
                1 => OperatorKind::Product,
                2 => OperatorKind::Min,
                3 => OperatorKind::Max,
                5 => OperatorKind::Gt,
                6 => OperatorKind::Lt,
                7 => OperatorKind::Eq,
                _ => panic!("illegal type_id {}", type_id),
            });
        }
        (Packet{ version, type_id, kind, subpackets }, ndx)
    }
}

/**
Find all packets (including subpackets!) from the hexadecimal string and
return the sum of their versions.

# Examples
```
assert_eq!(16, aoc2021::day16::sum_versions_from_hex_str("8A004A801A8002F478"));
assert_eq!(12, aoc2021::day16::sum_versions_from_hex_str("620080001611562C8802118E34"));
assert_eq!(23, aoc2021::day16::sum_versions_from_hex_str("C0015000016115A2E0802F182340"));
assert_eq!(31, aoc2021::day16::sum_versions_from_hex_str("A0016C880162017C3686B18A3D4780"));
```
 */
pub fn sum_versions_from_hex_str(hex_str: &str) -> u64 {
    let (packet, _) = Packet::from_binary_str(&hex_to_bin(hex_str));

    sum_versions(&packet)
}

pub fn sum_versions(packet: &Packet) -> u64 {
    let subpacket_sum: u64 = packet.get_subpackets().iter()
        .map(|p| sum_versions(p)).sum();
    subpacket_sum + packet.get_version() as u64
}

/**
Run part 1 of Day 16's exercise.

# Examples
```
assert_eq!(31, aoc2021::day16::run_part1("test_inputs/day16_1.txt"));
```
 */
pub fn run_part1(file: &str) -> u64 {
    let file = File::open(file).expect("could not open file");
    let mut sbuf = String::new();
    match BufReader::new(file).read_line(&mut sbuf) {
        Err(e) => {panic!("Error reading input: {:?}", e);},
        _ => (),
    };
    sum_versions_from_hex_str(sbuf.trim())
}

/**
Evaluate the value of a packet.

# Examples
```
assert_eq!(3, aoc2021::day16::eval_packet_from_hex_str("C200B40A82"));
assert_eq!(54, aoc2021::day16::eval_packet_from_hex_str("04005AC33890"));
assert_eq!(7, aoc2021::day16::eval_packet_from_hex_str("880086C3E88112"));
assert_eq!(9, aoc2021::day16::eval_packet_from_hex_str("CE00C43D881120"));
assert_eq!(1, aoc2021::day16::eval_packet_from_hex_str("D8005AC2A8F0"));
assert_eq!(0, aoc2021::day16::eval_packet_from_hex_str("F600BC2D8F"));
assert_eq!(0, aoc2021::day16::eval_packet_from_hex_str("9C005AC2F8F0"));
assert_eq!(1, aoc2021::day16::eval_packet_from_hex_str("9C0141080250320F1802104A08"));
```
 */
pub fn eval_packet_from_hex_str(hex_str: &str) -> u64 {
    let (packet, _) = Packet::from_binary_str(&hex_to_bin(hex_str));

    eval_packet(&packet)
}

pub fn eval_packet(packet: &Packet) -> u64 {
    match packet.get_kind() {
        PacketKind::Literal(v) => v,
        PacketKind::Operator(op_kind) => if [OperatorKind::Gt, OperatorKind::Lt, OperatorKind::Eq].contains(&op_kind) {
            // binary operation
            let op = match op_kind {
                OperatorKind::Gt => u64::gt,
                OperatorKind::Lt => u64::lt,
                OperatorKind::Eq => u64::eq,
                _ => panic!("This line should not be reached"),
            };
            if op(&eval_packet(&packet.get_subpackets()[0]), &eval_packet(&packet.get_subpackets()[1])) {
                1
            } else {
                0
            }
        } else {
            // n-ary operation
            let iter = packet.get_subpackets().iter()
                .map(|p| eval_packet(p));
            match op_kind {
                OperatorKind::Sum => iter.sum(),
                OperatorKind::Product => iter.product(),
                OperatorKind::Min => iter.min().unwrap(),
                OperatorKind::Max => iter.max().unwrap(),
                _ => panic!("This line should not be reached"),
            }
        }
    }
}

/**
Run part 2 of Day 16's exercise.

# Examples
```
assert_eq!(1, aoc2021::day16::run_part2("test_inputs/day16_2.txt"));
```
 */
pub fn run_part2(file: &str) -> u64 {
    let file = File::open(file).expect("could not open file");
    let mut sbuf = String::new();
    match BufReader::new(file).read_line(&mut sbuf) {
        Err(e) => {panic!("Error reading input: {:?}", e);},
        _ => (),
    };
    eval_packet_from_hex_str(sbuf.trim())
}

