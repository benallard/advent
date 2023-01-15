use std::collections::btree_map::BTreeMap;
use std::fmt::Debug;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let lines = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().parse::<Line>().unwrap())
        .collect::<Vec<_>>();

    let mut count = BTreeMap::new();
    for line in &lines {
        if line.is_diag() {
            println!("Skipping line");
            continue;
        }
        for point in line.points() {
            //dbg!(&point);
            *count.entry(point).or_insert(0) += 1;
        }
    }
    let part1 = count.iter().filter(|(_, v)| **v > 1).count();
    println!("part1: {}", part1);

    let mut count = BTreeMap::new();
    for line in &lines {
        for point in line.points() {
            //dbg!(&point);
            *count.entry(point).or_insert(0) += 1;
        }
    }
    let part2 = count.iter().filter(|(_, v)| **v > 1).count();
    println!("part2: {}", part2);
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = String;
    fn from_str(content: &str) -> Result<Self, <Self as FromStr>::Err> {
        let coord = content
            .split(",")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        Ok(Point {
            x: coord[0],
            y: coord[1],
        })
    }
}

struct Line {
    a: Point,
    b: Point,
}

impl FromStr for Line {
    type Err = String;
    fn from_str(content: &str) -> Result<Self, <Self as FromStr>::Err> {
        let points = content.split(" -> ").collect::<Vec<_>>();
        Ok(Line {
            a: points[0].parse().unwrap(),
            b: points[1].parse().unwrap(),
        })
    }
}

impl Line {
    fn is_diag(&self) -> bool {
        !(self.a.x == self.b.x || self.a.y == self.b.y)
    }

    fn len(&self) -> usize {
        let x_len = self.a.x.abs_diff(self.b.x);
        if x_len != 0 {
            return x_len as usize;
        }
        self.a.y.abs_diff(self.b.y) as usize
    }

    fn points(&self) -> Vec<Point> {
        let mut res = vec![];
        let x_offs: i32;
        if self.a.x == self.b.x {
            x_offs = 0;
        } else if self.a.x > self.b.x {
            x_offs = -1;
        } else {
            x_offs = 1;
        }

        let y_offs: i32;
        if self.a.y == self.b.y {
            y_offs = 0;
        } else if self.a.y > self.b.y {
            y_offs = -1;
        } else {
            y_offs = 1;
        }
        for i in 0..=self.len() {
            res.push(Point {
                x: (self.a.x as i32 + i as i32 * x_offs) as u32,
                y: (self.a.y as i32 + i as i32 * y_offs) as u32,
            });
        }
        res
    }
}

#[test]
fn test_diag() {
    let line: Line = "0,9 -> 5,9".parse().unwrap();
    assert_eq!(false, line.is_diag());
    assert_eq!(6, line.points().len());
    assert!("6,4 -> 2,0".parse::<Line>().unwrap().is_diag());
    assert_eq!(7, "9,4 -> 3,4".parse::<Line>().unwrap().points().len());
    assert_eq!(7, "4,9 -> 4,3".parse::<Line>().unwrap().points().len());
}
