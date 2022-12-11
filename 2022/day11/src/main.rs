use std::io::BufRead;

use core::str::FromStr;

fn main() {
    let mut monkeys: Vec<_> = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .chunks(7)
        .map(|c| c.join("").parse::<Monkey>().unwrap())
        .collect();
}

struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test: Test,
}

impl FromStr for Monkey {
    type Err = String;
    fn from_str(_: &str) -> Result<Self, <Self as FromStr>::Err> {
        todo!()
    }
}

struct Operation {}

struct Test {
    div: u8,
    positive: usize,
    negative: usize,
}
