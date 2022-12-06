use std::io::BufRead;

fn main() {
    let mut aim = 0;
    let mut pos = 0;
    let mut depth = 0;
    std::io::BufReader::new(std::io::stdin())
        .lines()
        .for_each(|line| {
            let line = line.unwrap();
            let orders: Vec<&str> = line.split(' ').collect();
            let val = orders[1].parse::<u64>().unwrap();
            match orders[0] {
                "forward" => {
                    pos += val;
                    depth += aim * val;
                }
                "up" => {
                    aim -= val;
                }
                "down" => {
                    aim += val;
                }
                _ => panic!("Unknown order: {}", orders[0]),
            }
        });
    println!("{}", pos * depth)
}
