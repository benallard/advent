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
    for line in lines {
        if line.is_diag() {
            println!("Skipping line");
            continue;
        }
        for point in line.points() {
            dbg!(&point);
            *count.entry(point).or_insert(0) += 1;
        }
    }
    let part1 = count.iter().filter(|(_, v)| **v > 1).count();
    println!("part1: {}", part1);
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

    fn points(&self) -> Vec<Point> {
        let mut res = vec![];
        let mut x_range = self.a.x..=self.b.x;
        if x_range.is_empty(){
            x_range = self.b.x..=self.a.x;
        }
        
        for x in x_range {
            let mut y_range = self.a.y..=self.b.y;
        if y_range.is_empty(){
            y_range = self.b.y..=self.a.y;
        }
            for y in y_range {
                res.push(Point { x, y });
            }
        }
        res
    }
}

#[test]
fn test_diag() {
    let line: Line = "0,9 -> 5,9".parse().unwrap();
    assert_eq!(false, line.is_diag());
    assert_eq!(6, line.points().len());
    assert!( "6,4 -> 2,0".parse::<Line>().unwrap().is_diag());
    assert_eq!(7, "9,4 -> 3,4".parse::<Line>().unwrap().points().len());
    assert_eq!(7, "4,9 -> 4,3".parse::<Line>().unwrap().points().len());
}
