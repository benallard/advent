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
        .map(|(i, _)| i)
        .sum::<usize>();
    println!("count: {}", part1);
}

enum PacketValue {
    Value(u32), // terminal
    Deeper(Packet),
}

struct Packet {
    value: Vec<PacketValue>,
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // first and last char must be '[', ']'
        if !s.starts_with('[') {
            return Err("Wrong packet start".to_owned());
        }
        if !s.ends_with(']') {
            return Err("Wrong Packet end".to_owned());
        }
        let value = s[1..s.len() - 1]
            .split(",")
            .map(|v| match v.parse() {
                Ok(x) => PacketValue::Value(x),
                Err(_) => PacketValue::Deeper(v.parse::<Packet>().unwrap()),
            })
            .collect();
        Ok(Packet { value })
    }
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
        todo!()
    }
}
