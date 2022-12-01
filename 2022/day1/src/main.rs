use std::io::BufRead;

fn main() {
    let mut current = 0;

    let max = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|line| line.unwrap().trim().parse::<u32>())
        .map(|value| {
            match value {
                Ok(t) => {
                    current = current + t;
                    None
                }
                Err(_) => {
                    let val = current;
                    current = 0;
                    Some(val)
                }
            }
        })
        .filter(|val| val.is_some())
        .map(|val| val.unwrap())
        .max().unwrap();
    println!("{}", max);
}