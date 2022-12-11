use std::collections::VecDeque;
use std::io::BufRead;

use core::str::FromStr;

fn main() {
    let mut monkeys: Vec<_> = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .chunks(7)
        .map(|c| c.join("\n").parse::<Monkey>().unwrap())
        .collect();

    for _round in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                let value;
                let dest;
                {
                    let new = monkeys[i].operation.process(item);
                    let new = new / 3;
                    dest = monkeys[i].test.get_dest(new);
                    value = new;
                }
                monkeys[dest].items.push_back(value);
            }
        }
        monkeys
            .iter()
            .enumerate()
            .for_each(|(i, m)| println!("{}: {:?}", i, m.items))
    }
}

struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    test: Test,
}

impl FromStr for Monkey {
    type Err = String;
    fn from_str(content: &str) -> Result<Self, <Self as FromStr>::Err> {
        let lines: Vec<_> = content.split("\n").collect();
        if lines.len() < 6 {
            return Err(format!("Wrong Monkey length: {}", lines.len()));
        }
        let items = lines[1]
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();
        let operation: Operation = lines[2].split(": ").nth(1).unwrap().parse().unwrap();
        let test: Test = lines[3..6].join("\n").parse().unwrap();
        Ok(Monkey {
            items,
            operation,
            test,
        })
    }
}

enum Op {
    Add,
    Mul,
}

enum Operand {
    Constant(u32),
    Value,
}

struct Operation {
    op: Op,
    op1: Operand,
    op2: Operand,
}

impl FromStr for Operation {
    type Err = String;
    fn from_str(content: &str) -> Result<Self, <Self as FromStr>::Err> {
        let tokens: Vec<_> = content.split(" ").collect();
        if tokens[0] != "new" || tokens[1] != "=" {
            return Err("Wrong start".to_string());
        }
        let op1 = match tokens[2] {
            "old" => crate::Operand::Value,
            _ => crate::Operand::Constant(tokens[2].parse().unwrap()),
        };
        let op = match tokens[3] {
            "+" => crate::Op::Add,
            "*" => crate::Op::Mul,
            _ => panic!(),
        };
        let op2 = match tokens[4] {
            "old" => crate::Operand::Value,
            _ => crate::Operand::Constant(tokens[4].parse().unwrap()),
        };
        Ok(Operation { op, op1, op2 })
    }
}

impl Operation {
    fn process(&self, value: u32) -> u32 {
        let op1 = match self.op1 {
            Operand::Value => value,
            Operand::Constant(v) => v,
        };
        let op2 = match self.op2 {
            Operand::Value => value,
            Operand::Constant(v) => v,
        };
        match self.op {
            Op::Add => op1 + op2,
            Op::Mul => op1 * op2,
        }
    }
}

struct Test {
    div: u8,
    positive: usize,
    negative: usize,
}

impl FromStr for Test {
    type Err = String;
    fn from_str(content: &str) -> Result<Self, <Self as FromStr>::Err> {
        let lines: Vec<_> = content.split("\n").collect();
        if lines.len() != 3 {
            return Err("wrong Test length".to_string());
        }
        let div;
        {
            let tokens: Vec<_> = lines[0].split(" ").collect();
            if tokens[3] != "divisible" {
                return Err("Not divisible: ".to_owned() + tokens[2]);
            }
            div = tokens[5].parse().unwrap();
        }
        let positive = lines[1]
            .split(" ")
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let negative = lines[2]
            .split(" ")
            .collect::<Vec<_>>()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        Ok(Test {
            div,
            positive,
            negative,
        })
    }
}

impl Test {
    fn get_dest(&self, value: u32) -> usize {
        if value % (self.div as u32) == 0 {
            self.positive
        } else {
            self.negative
        }
    }
}

#[test]
fn testParse() {
    "new = old * 19".parse::<Operation>().unwrap();
}
