use std::io::BufRead;
use std::str::FromStr;

use std::convert::TryInto;

fn main() {
    let entries = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().parse::<Entry>().unwrap())
        .collect::<Vec<_>>();

    let part1 = entries
        .iter()
        .map(|e| e.count_output_1_4_7_8())
        .sum::<usize>();
    println!("part1: {}", part1);
    let part2 = entries
        .into_iter()
        .map(|e| e.decode())
        .inspect(|v| println!("value: {}", v))
        .sum::<u32>();
    println!("part2: {}", part2);
}

#[derive(Debug, PartialEq, Clone)]
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
            _ => false,
        }
    }

    fn and(&self, other: &Digit) -> Digit {
        Digit {
            a: self.a && other.a,
            b: self.b && other.b,
            c: self.c && other.c,
            d: self.d && other.d,
            e: self.e && other.e,
            f: self.f && other.f,
            g: self.g && other.g,
        }
    }

    /*
        0 0 | 0
        0 1 | 0
        1 0 | 1
        1 1 | 0
    */
    fn minus(&self, other: &Digit) -> Digit {
        Digit {
            a: self.a && !other.a,
            b: self.b && !other.b,
            c: self.c && !other.c,
            d: self.d && !other.d,
            e: self.e && !other.e,
            f: self.f && !other.f,
            g: self.g && !other.g,
        }
    }

    /*
        0 0 | 0
        0 1 | 1
        1 0 | 1
        1 1 | 1
    */
    fn plus(&self, other: &Digit) -> Digit {
        Digit {
            a: self.a || other.a,
            b: self.b || other.b,
            c: self.c || other.c,
            d: self.d || other.d,
            e: self.e || other.e,
            f: self.f || other.f,
            g: self.g || other.g,
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

impl Entry {
    fn count_output_1_4_7_8(&self) -> usize {
        self.output.iter().filter(|d| d.is_1_4_7_or_8()).count()
    }

    fn decode(&self) -> u32 {
        let one = self
            .patterns
            .iter()
            .filter(|d| d.count() == 2)
            .next()
            .unwrap();
        let four = self
            .patterns
            .iter()
            .filter(|d| d.count() == 4)
            .next()
            .unwrap();
        let seven = self
            .patterns
            .iter()
            .filter(|d| d.count() == 3)
            .next()
            .unwrap();
        let eight = self
            .patterns
            .iter()
            .filter(|d| d.count() == 7)
            .next()
            .unwrap();
        // 7 & 1 -> top
        let top = seven.and(one);
        // (5&2&3) & 4 -> middle
        let five_two_three = self
            .patterns
            .iter()
            .filter(|d| d.count() == 5)
            .cloned()
            .reduce(|accum, item| accum.and(&item))
            .unwrap();
        let middle = five_two_three.and(four);
        // 8 - middle -> 0
        let zero = self
            .patterns
            .iter()
            .filter(|d| d.count() == 6)
            .filter(|d| eight.minus(d) == middle)
            .next()
            .unwrap();
        // (5&2&3) - top - middle -> bottom
        let bottom = five_two_three.minus(&top).minus(&middle);
        // 3 = 1 + top + middle + bottom
        let three = &one.plus(&top).plus(&middle).plus(&bottom);
        // 9 = L6 - 3 = L1
        let nine = self
            .patterns
            .iter()
            .filter(|d| d.count() == 6)
            //.inspect(|d| println!("len: {}", d.minus(&three).count()))
            .filter(|d| d.minus(&three).count() == 1)
            .next()
            .unwrap();
        // 5 = L5 - top - middle - bottom - TL = L1
        let five = self
            .patterns
            .iter()
            .filter(|d| d.count() == 5)
            .filter(|d| d.minus(&seven).minus(&three).count() == 1)
            .next()
            .unwrap();
        // 2 = L5, !5, !3
        let two = self
            .patterns
            .iter()
            .filter(|d| d.count() == 5)
            .filter(|d| d != &five)
            .filter(|d| d != &three)
            .next()
            .unwrap();
        // 6 = L6, !9 !0
        let six = self
            .patterns
            .iter()
            .filter(|d| d.count() == 6)
            .filter(|d| d != &nine)
            .filter(|d| d != &zero)
            .next()
            .unwrap();

        let res = self
            .output
            .iter()
            .map(|d| match d {
                i if i == zero => 0,
                i if i == one => 1,
                i if i == two => 2,
                i if i == three => 3,
                i if i == four => 4,
                i if i == five => 5,
                i if i == six => 6,
                i if i == seven => 7,
                i if i == eight => 8,
                i if i == nine => 9,
                _ => panic!(),
            })
            .collect::<Vec<_>>();
        res[0] * 1000 + res[1] * 100 + res[2] * 10 + res[3]
    }
}
