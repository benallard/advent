use std::{io::BufRead, str::FromStr};

fn main() {
    let part1 = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().into())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|c| PacketPair::new(&c[0], &c[1]))
        .enumerate()
        .filter(|(_, pp)| pp.is_sorted())
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    println!("count: {}", part1);
}

#[derive(PartialEq, Debug)]
enum PacketValue {
    Value(u32), // terminal
    Deeper(Packet),
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PacketValue::Value(a), PacketValue::Value(b)) => a.partial_cmp(b),
            (PacketValue::Deeper(a), PacketValue::Deeper(b)) => a.partial_cmp(b),
            (PacketValue::Value(a), PacketValue::Deeper(_b)) => PacketValue::Deeper(Packet {
                value: vec![PacketValue::Value(*a)],
            })
            .partial_cmp(other),
            (PacketValue::Deeper(_a), PacketValue::Value(b)) => {
                self.partial_cmp(&PacketValue::Deeper(Packet {
                    value: vec![PacketValue::Value(*b)],
                }))
            }
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug)]
struct Packet {
    value: Vec<PacketValue>,
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //dbg!(s);
        // first and last char must be '[', ']'
        if !s.starts_with('[') {
            return Err("Wrong packet start".to_owned());
        }
        if !s.ends_with(']') {
            return Err("Wrong Packet end".to_owned());
        }
        let mut values = vec![];
        let mut start = 1;
        while start < s.len() {
            //dbg!(start);
            if s[start..].starts_with('[') {
                //dbg!("deep");
                let end = get_matching_closing_idx(&s[start..]) + start + 1;
                if start != end {
                    values.push(PacketValue::Deeper(
                        s[start..end].parse::<Packet>().unwrap(),
                    ));
                }
                start = end + 1;
            } else {
                let end = s[start..].find(',').unwrap_or(s.len() - start - 1) + start;
                //dbg!(end, s[start..end].to_owned());
                if start != end {
                    values.push(PacketValue::Value(s[start..end].parse().unwrap()));
                }
                start = end + 1;
            }
        }
        Ok(Packet { value: values })
    }
}

fn get_matching_closing_idx(s: &str) -> usize {
    let mut count = 0;
    for (idx, c) in s.chars().enumerate() {
        if c == '[' {
            count += 1;
        } else if c == ']' {
            count -= 1;
            if count == 0 {
                return idx;
            }
        }
    }
    return s.len();
}

#[test]
fn test_closing_idx() {
    assert_eq!(6, get_matching_closing_idx("[[123]]abcdfehd"))
}

struct PacketPair {
    packet1: Packet,
    packet2: Packet,
}

impl PacketPair {
    fn new(l1: &str, l2: &str) -> PacketPair {
        PacketPair {
            packet1: l1.parse().unwrap(),
            packet2: l2.parse().unwrap(),
        }
    }

    fn is_sorted(&self) -> bool {
        self.packet1 < self.packet2
    }
}

#[test]
fn example_part1() {
    let packet: Packet = "[1,1,3,1,1]".parse().unwrap();
    assert_eq!(5, packet.value.len());
    assert_eq!(PacketValue::Value(1), packet.value[0]);
    assert_eq!(PacketValue::Value(1), packet.value[1]);
    assert_eq!(PacketValue::Value(3), packet.value[2]);
    assert_eq!(PacketValue::Value(1), packet.value[3]);
    assert_eq!(PacketValue::Value(1), packet.value[4]);
    let packet: Packet = "[]".parse().unwrap();
    assert_eq!(0, packet.value.len());
    assert!(PacketPair::new("[1,1,3,1,1]", "[1,1,5,1,1]").is_sorted());
    assert!(PacketPair::new("[[1],[2,3,4]]", "[[1],4]").is_sorted());
    assert!(!PacketPair::new("[9]", "[[8,7,6]]").is_sorted());
    assert!(PacketPair::new("[[4,4],4,4]", "[[4,4],4,4,4]").is_sorted());
    assert!(!PacketPair::new("[7,7,7,7]", "[7,7,7]").is_sorted());
    assert!(PacketPair::new("[]", "[3]").is_sorted());
    assert!(!PacketPair::new("[[[]]]]", "[[]]").is_sorted());
    assert!(
        !PacketPair::new("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]").is_sorted()
    );
}
