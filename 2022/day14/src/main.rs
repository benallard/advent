use std::{io::BufRead, str::FromStr};

fn main() {
    let mut cave = Cave {
        state: vec![vec![Tile::Air; 1000]; 1000],
    };
    std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().into())
        .for_each(|l: String| {
            l.split(" -> ")
                .map(|p| p.parse().unwrap())
                .collect::<Vec<_>>()
                .windows(2)
                .for_each(|w| cave.add_wall(&w[0], &w[1]))
        })
}

struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(",").map(|c| c.parse().unwrap()).collect();
        Ok(Point{
            x: parts[0],
            y: parts[1]
        })
    }
}

#[derive(Clone)]
enum Tile{
    Air,
    Rock,
    Sand,
}

struct Cave {
    state: Vec<Vec<Tile>>,
}

impl Cave {
    fn add_wall(&mut self, a: &Point, b: &Point) {
        if a.x != b.x && a.y != b.y{
            panic!("diag!")
        } else
        if a.x == b.x{
            (a.y..=b.y).into_iter().for_each(|y| self.state[y][a.x] = Tile::Rock);
        } else
        if a.y == b.y {
            (a.x..=b.x).into_iter().for_each(|x| self.state[a.y][x] = Tile::Rock);
        }
    }

    fn add_sand(&mut self) -> Option<Point> {
        todo!()
    }
}
