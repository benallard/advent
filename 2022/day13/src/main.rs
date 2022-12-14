use std::{io::BufRead, str::FromStr};

fn main(){
    let part1 = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().into())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|c| PacketPair::new(&c[0], &c[1]))
        .filter(|pp| pp.is_sorted())
        .count();
    println!("count: {}", part1);
}

struct Packet{

}

impl FromStr for Packet{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct PacketPair{
    packet1: Packet,
    packet2: Packet,
}

impl PacketPair{
    fn new(l1: &str, l2: &str)-> PacketPair{
        todo!()
    }

    fn is_sorted(&self) -> bool{
        todo!()
    }
}