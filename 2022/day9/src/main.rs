use std::{collections::HashSet, io::BufRead};

fn main() {
    let mut seen = HashSet::new();
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    seen.insert(tail);

    std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().into())
        .for_each(|l: String| {
            let tokens: Vec<_> = l.split(" ").collect();
            let dir = tokens[0];
            let length = tokens[1].parse().unwrap();
            for _ in 0..length {
                head.move_1(dir);
                //dbg!((head.x, head.y));
                tail.move_toward(&head);
                //dbg!((tail.x, tail.y));
                seen.insert(tail);
            }
        });

    println!("visited: {}", seen.len());
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_1(&mut self, dir: &str) {
        match dir {
            "R" => self.x += 1,
            "L" => self.x -= 1,
            "U" => self.y += 1,
            "D" => self.y -= 1,
            _ => panic!(),
        }
    }

    fn move_toward(&mut self, other: &Self) {
        if self.x == other.x {
            // only move y
            self.y += match other.y - self.y {
                2.. => 1,
                -1..=1 => 0,
                _ => -1,
            };
        } else if self.y == other.y {
            // only move x
            self.x += match other.x - self.x {
                2.. => 1,
                -1..=1 => 0,
                _ => -1,
            };
        } else {
            // move both
            let xdiff = other.x - self.x;
            let ydiff = other.y - self.y;
            if xdiff.abs() > 1 || ydiff.abs() > 1 {
                self.x += match xdiff {
                    1.. => 1,
                    0 => 0,
                    _ => -1,
                };
                self.y += match ydiff {
                    1.. => 1,
                    0 => 0,
                    _ => -1,
                };
            }
        }
    }
}
