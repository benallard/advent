use std::{io::BufRead, str::FromStr};

fn main() {
    let mut cave = Cave {
        state: vec![vec![Tile::Air; 1000]; 1000],
        max_depth: 0,
    };
    std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().into())
        .for_each(|l: String| {
            l.split(" -> ")
                .map(|p| p.parse().unwrap())
                .collect::<Vec<_>>()
                .windows(2)
                .for_each(|w| {
                    //dbg!(w[0], w[1]);
                    cave.add_wall(&w[0], &w[1])}
                )
        });
    cave.init_depth();
    dbg!(cave.max_depth);
    let sand_seed = "500,0".parse().unwrap();

    let mut count = 0;
    while let Some(_point) = cave.add_sand(&sand_seed) {
        count += 1;
    }
    println!("part1: {}", count);
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(",").map(|c| c.parse().unwrap()).collect();
        Ok(Point {
            x: parts[0],
            y: parts[1],
        })
    }
}

impl Point{
    fn to(&self, dest: &Point) -> Vec<Point>{
        if self.x == dest.x{
            if self.y < dest.y{
                return (self.y..=dest.y).map(|y| Point{x: self.x, y}).collect();
            } else {
                return (dest.y..self.y).map(|y| Point{x: self.x, y}).collect();
            }
        }
        if self.y == dest.y{
            if self.x < dest.x{
                return (self.x..=dest.x).map(|x| Point{x, y:self.y}).collect();
            } else {
                return (dest.x..=self.x).map(|x| Point{x, y:self.y}).collect();
            }
        }
        panic!()
    }
}

#[derive(Clone, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

struct Cave {
    state: Vec<Vec<Tile>>,
    max_depth: usize,
}

impl Cave {
    fn add_wall(&mut self, a: &Point, b: &Point) {
            a.to(b).iter().for_each(|p| {
                self.state[p.y][p.x] = Tile::Rock
            })
    }

    fn init_depth(&mut self) {
        self.max_depth = self.state.len()
            - self
                .state
                .iter()
                .rev()
                .position(|l| l.iter().filter(|t| **t != Tile::Air).next().is_some())
                .unwrap();
    }

    fn add_sand(&mut self, seed: &Point) -> Option<Point> {
        let mut pos = *seed;
        loop {
            if pos.y > self.max_depth {
                //dbg!("done".to_owned(), pos.x);
                return None;
            }
            if self.state[pos.y + 1][pos.x] == Tile::Air {
                pos = Point {
                    x: pos.x,
                    y: pos.y + 1,
                }
            } else if self.state[pos.y + 1][pos.x - 1] == Tile::Air {
                pos = Point {
                    x: pos.x - 1,
                    y: pos.y + 1,
                }
            } else if self.state[pos.y + 1][pos.x + 1] == Tile::Air {
                pos = Point {
                    x: pos.x + 1,
                    y: pos.y + 1,
                }
            } else {
                break;
            }
            //dbg!("nd".to_owned());
        }
        self.state[pos.y][pos.x] = Tile::Sand;
        Some(pos)
    }
}
