use std::io::BufRead;

fn main() {
    let res = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect::<Vec<u64>>()
        // comment out the following lines to get the first part
        .windows(3)
        .map(|window| window[0] + window[1] + window[2])
        .collect::<Vec<u64>>()
        // end comment
        .windows(2)
        .fold(0, |acc, w| acc + (w[1] > w[0]) as u64);
    println!("{}", res);

}
