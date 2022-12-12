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
                        26
                    }
                    _ => c.to_digit(36).unwrap() as u8 - 10,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    //  Our distances to start, initialize with the biggest possible value
    let mut dist = vec![vec![field[0].len() * field.len(); field[0].len()]; field.len()];
    dist[start.y][start.x] = 0;

    // the points to process
    let mut todo = vec![];
    todo.push(start);
    while let Some(point) = todo.pop() {
        let res = dist[point.y][point.x] + 1;
        let local = field[point.y][point.x];
        // left
        if point.x > 0 {
            let dest = Point{x: point.x-1, y: point.y};
            if dist[dest.y][dest.x] > res {
                if field[dest.y][dest.x] <= local + 1 {
                    dist[dest.y][dest.x] = res;
                    todo.push(dest);
                }
            }
        }
        // up
        if point.y > 0 {
            let dest = Point{x: point.x, y: point.y-1};
            if dist[dest.y][dest.x] > res {
                if field[dest.y][dest.x] <= local + 1 {
                    dist[dest.y][dest.x] = res;
                    todo.push(dest);
                }
            }
        }
        // right
        if point.x < field[point.y].len()-1 {
            let dest = Point{x: point.x+1, y: point.y};
            if dist[dest.y][dest.x] > res {
                if field[dest.y][dest.x] <= local + 1 {
                    dist[dest.y][dest.x] = res;
                    todo.push(dest);
                }
            }
        }
        // down
        if point.y < field.len()-1 {
            let dest = Point{x: point.x, y: point.y+1};
            if dist[dest.y][dest.x] > res {
                if field[dest.y][dest.x] <= local + 1 {
                    dist[dest.y][dest.x] = res;
                    todo.push(dest);
                }
            }
        }
    }
    println!("part1: {}", dist[end.y][end.x]);
}

struct Point {
    x: usize,
    y: usize,
}
