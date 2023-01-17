use std::io::BufRead;
use std::str::FromStr;

use std::convert::TryInto;

fn main() {
    let entries = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().parse::<Entry>().unwrap())
        .collect::<Vec<_>>();

    let part1 = entries.into_iter().map(|e| e.count_output_1_4_7_8()).sum::<usize>();
    println!("part1: {}", part1);
}

#[derive(Debug)]
struct Digit {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: bool,
    f: bool,
    g: bool,
}

impl FromStr for Digit {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, <Self as FromStr>::Err> {
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g) =
            (false, false, false, false, false, false, false);
        value.chars().into_iter().for_each(|x| match x {
            'a' => a = true,
            'b' => b = true,
            'c' => c = true,
            'd' => d = true,
            'e' => e = true,
            'f' => f = true,
            'g' => g = true,
            other => panic!("{}", other),
        });
        Ok(Digit {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
        })
    }
}

impl Digit {
    fn count(&self) -> usize {
        let mut res = 0;
        if self.a {
            res += 1
        }
        if self.b {
            res += 1
        }
        if self.c {
            res += 1
        }
        if self.d {
            res += 1
        }
        if self.e {
            res += 1
        }
        if self.f {
            res += 1
        }
        if self.g {
            res += 1
        }
        res
    }

    fn is_1_4_7_or_8(&self) -> bool {
        match self.count() {
            2 | 3 | 4 | 7 => true,
            _=> false,
        }
    }
}

struct Entry {
    patterns: [Digit; 10],
    output: [Digit; 4],
}

impl FromStr for Entry {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, <Self as FromStr>::Err> {
        let (patterns, output) = value.split_once(" | ").unwrap();
        Ok(Entry {
            patterns: patterns
                .split_whitespace()
                .into_iter()
                .map(|d| d.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            output: output
                .split_whitespace()
                .into_iter()
                .map(|d| d.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        })
    }
}

impl Entry{
    fn count_output_1_4_7_8(&self) -> usize{
        self.output.iter().filter(|d| d.is_1_4_7_or_8()).count()
    }
}
