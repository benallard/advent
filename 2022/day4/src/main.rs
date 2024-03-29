use std::{io::BufRead, str::FromStr};

fn main() {
    let amount = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|line| {
            line.unwrap()
                .trim()
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .filter(|secs| {
            let elf1 = Section::from_str(&secs[0]).unwrap();
            let elf2 = Section::from_str(&secs[1]).unwrap();
            // part 1
            //elf1.is_subset(&elf2) || elf2.is_subset(&elf1)
            // part 2
            !elf1.is_disjoint(&elf2)
        })
        .count();
    println!("count: {}", amount);
}

struct Section {
    from: u32,
    to: u32,
}

impl FromStr for Section {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let parts: Vec<&str> = s.split("-").collect();
        if parts.len() != 2 {
            return Err("Invalid string format".to_string());
        }

        let from = parts[0].parse::<u32>().unwrap();
        let to = parts[1].parse::<u32>().unwrap();

        Ok(Section { from, to })
    }
}

#[allow(dead_code)]
impl Section {
    fn is_subset(&self, other: &Section) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn is_disjoint(&self, other: &Section) -> bool {
        self.to < other.from || self.from > other.to
    }
}
