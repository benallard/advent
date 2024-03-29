use std::io::BufRead;

fn main() {
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };
    let field: Vec<Vec<u8>> = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().to_owned())
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = Point { x, y };
                        0
                    }
                    'E' => {
                        end = Point { x, y };
                        25
                    }
                    _ => c.to_digit(36).unwrap() as u8 - 10,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    {
        //  Our distances to start, initialize with the biggest possible value
        let mut dist = vec![vec![field[0].len() * field.len(); field[0].len()]; field.len()];
        dist[start.y][start.x] = 0;

        // the points to process
        let mut todo = std::collections::VecDeque::new();
        todo.push_back(start);
        while let Some(point) = todo.pop_front() {
            // new dist
            let res = dist[point.y][point.x] + 1;
            // local height
            let local = field[point.y][point.x];
            // left
            if point.x > 0 {
                let dest = Point {
                    x: point.x - 1,
                    y: point.y,
                };
                // shorter dist
                if dist[dest.y][dest.x] > res {
                    // accessible
                    if field[dest.y][dest.x] <= local + 1 {
                        // won: update dist
                        dist[dest.y][dest.x] = res;
                        // process point.
                        todo.push_back(dest);
                    }
                }
            }
            // up
            if point.y > 0 {
                let dest = Point {
                    x: point.x,
                    y: point.y - 1,
                };
                if dist[dest.y][dest.x] > res {
                    if field[dest.y][dest.x] <= local + 1 {
                        dist[dest.y][dest.x] = res;
                        todo.push_back(dest);
                    }
                }
            }
            // right
            if point.x < field[point.y].len() - 1 {
                let dest = Point {
                    x: point.x + 1,
                    y: point.y,
                };
                if dist[dest.y][dest.x] > res {
                    if field[dest.y][dest.x] <= local + 1 {
                        dist[dest.y][dest.x] = res;
                        todo.push_back(dest);
                    }
                }
            }
            // down
            if point.y < field.len() - 1 {
                let dest = Point {
                    x: point.x,
                    y: point.y + 1,
                };
                if dist[dest.y][dest.x] > res {
                    if field[dest.y][dest.x] <= local + 1 {
                        dist[dest.y][dest.x] = res;
                        todo.push_back(dest);
                    }
                }
            }
        }
        /*
            dist.iter().enumerate().for_each(|(y,l)| {
                l.iter().enumerate().for_each(|(x, d)| print!(" {:#4}-{:#2}", d, field[y][x]));
                println!("");
            });
        */
        println!("part1: {}", dist[end.y][end.x]);
    }
    // do it again, start on top, stop at first cell with elevation 0
    {
        let mut min = 0;
        //  Our distances to start, initialize with the biggest possible value
        let mut dist = vec![vec![field[0].len() * field.len(); field[0].len()]; field.len()];
        dist[end.y][end.x] = 0;

        // the points to process
        let mut todo = std::collections::VecDeque::new();
        todo.push_back(end);
        while let Some(point) = todo.pop_front() {
            // new dist
            let res = dist[point.y][point.x] + 1;
            min = res - 1;
            // local height
            let local = field[point.y][point.x];
            // stop when we reach the bottom.
            if local == 0 {
                break;
            }
            // left
            if point.x > 0 {
                let dest = Point {
                    x: point.x - 1,
                    y: point.y,
                };
                // shorter dist
                if dist[dest.y][dest.x] > res {
                    // accessible
                    if field[dest.y][dest.x] >= local - 1 {
                        // won: update dist
                        dist[dest.y][dest.x] = res;
                        // process point.
                        todo.push_back(dest);
                    }
                }
            }
            // up
            if point.y > 0 {
                let dest = Point {
                    x: point.x,
                    y: point.y - 1,
                };
                if dist[dest.y][dest.x] > res {
                    if field[dest.y][dest.x] >= local - 1 {
                        dist[dest.y][dest.x] = res;
                        todo.push_back(dest);
                    }
                }
            }
            // right
            if point.x < field[point.y].len() - 1 {
                let dest = Point {
                    x: point.x + 1,
                    y: point.y,
                };
                if dist[dest.y][dest.x] > res {
                    if field[dest.y][dest.x] >= local - 1 {
                        dist[dest.y][dest.x] = res;
                        todo.push_back(dest);
                    }
                }
            }
            // down
            if point.y < field.len() - 1 {
                let dest = Point {
                    x: point.x,
                    y: point.y + 1,
                };
                if dist[dest.y][dest.x] > res {
                    if field[dest.y][dest.x] >= local - 1 {
                        dist[dest.y][dest.x] = res;
                        todo.push_back(dest);
                    }
                }
            }
        }
        /*
                dist.iter().enumerate().for_each(|(y, l)| {
                    l.iter()
                        .enumerate()
                        .for_each(|(x, d)| print!(" {:#4}-{:#2}", d, field[y][x]));
                    println!("");
                });
        */
        println!("part2: {}", min);
    }
}

#[derive(PartialEq)]
struct Point {
    x: usize,
    y: usize,
}
