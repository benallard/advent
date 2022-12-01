use std::io::BufRead;

fn main() {
    let mut max = 0;
    let mut current = 0;

    std::io::BufReader::new(std::io::stdin())
        .lines()
        .for_each(|line| {
            let value: Result<u32, _> = line.unwrap().trim().parse::<u32>();
            match value {
                Ok(t) => {
                    current = current + t;
                }
                Err(_) => {
                    if current > max {
                        max = current;
                    }
                    current = 0;
                }
            }
        });
    if current > max {
        max = current;
    }
    println!("{}", max);
}