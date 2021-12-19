use std::io::BufRead;

fn main() {
    let mut res = 0;
    let mut prev = u64::max_value();
    std::io::BufReader::new(std::io::stdin())
    .lines()
    .map(|line| line.unwrap().trim().parse().unwrap())
    .for_each(|n| {if n > prev { res += 1} prev = n;});
    println!("{}", res);

}
