use std::{io::Read, str::FromStr};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let map: Map = buf.parse().unwrap();

    let part1 = map.low_points().iter().map(|v| (v + 1) as u32).sum::<u32>();
    println!("part1: {}", part1);
}

struct Map {
    points: Vec<Vec<u8>>,
}

impl FromStr for Map {
    type Err = String;
    fn from_str(content: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(Map {
            points: content
                .split("\n")
                .map(|l| {
                    l.chars()
                        .map(|c| c.to_digit(10))
                        .filter(|n| n.is_some())
                        .map(|n| n.unwrap() as u8)
                        .collect()
                })
                .filter(|v: &Vec<u8>| v.len() != 0)
                .collect(),
        })
    }
}

impl Map {
    fn low_points(&self) -> Vec<u8> {
        self.points
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .filter(|(x, _)| self.low_point(*x, y))
                    .map(|(_, v)| *v)
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn low_point(&self, x: usize, y: usize) -> bool {
        let val = self.points[y][x];
        if x != 0 && self.points[y][x - 1] < val {
            return false;
        }
        if y != 0 && self.points[y - 1][x] < val {
            return false;
        }
        if x != self.points[0].len() - 1 && self.points[y][x + 1] < val {
            return false;
        }
        if y != self.points.len() - 1 && self.points[y + 1][x] < val {
            return false;
        }
        return true;
    }
}
