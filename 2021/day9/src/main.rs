use std::{io::Read, str::FromStr};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let map: Map = buf.parse().unwrap();

    let part1 = map
        .low_points()
        .iter()
        //.inspect(|c| println!("{}, {}: {}", c.x, c.y, map.points[c.y][c.x]))
        .map(|c| (map.points[c.y][c.x] + 1) as u32)
        .sum::<u32>();
    println!("part1: {}", part1);
/* 
    let part2_vec = map
        .low_points()
        .iter()
        .map(|c| map.basin_size(c.x, c.y))
        .collect::<Vec<_>>();
    part2_vec.sort();
    let part2 = part2_vec
        .iter()
        .rev()
        .collect::<Vec<_>>()
        .chunks(3)
        .nth(0)
        .map(|c| c[0] * c[1] * c[2])
        .unwrap();
    println!("part2 {}", part2);
    */
}

struct Map {
    points: Vec<Vec<u8>>,
}

struct Coord {
    x: usize,
    y: usize,
}

impl FromStr for Map {
    type Err = String;
    fn from_str(content: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(Map {
            points: content
                .lines()
                //.inspect(|l| println!("line: {}", l))
                .map(|l| {
                    l.chars()
                        .map(|c| c.to_digit(10))
                        .filter(|n| n.is_some())
                        .map(|n| n.unwrap() as u8)
                        .collect()
                })
                .filter(|v: &Vec<u8>| v.len() != 0)
                //.inspect(|v| println!("len: {}", v.len()))
                .collect(),
        })
    }
}

impl Map {
    fn low_points(&self) -> Vec<Coord> {
        self.points
            .iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .filter(|(x, _)| self.low_point(*x, y))
                    .map(|(x, _)| Coord { x, y })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn low_point(&self, x: usize, y: usize) -> bool {
        let val = self.points[y][x];
        if val == 9 {
            return false;
        }
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

/*    fn basin_size(&self, x: usize, y: usize) -> usize {
        
    }
*/
}
