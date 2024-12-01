use std::io::BufRead;

fn main() {
    let locations_a: &mut Vec<u32> = &mut vec![];
    let locations_b: &mut Vec<u32> = &mut vec![];

    std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|line| line.unwrap().trim().into())
        .for_each(|line: String| {
            line.split_whitespace().enumerate().for_each(|(i, val)| {
                let location = val.parse().unwrap();
                if i == 0 {
                    locations_a.push(location);
                }
                if i == 1 {
                    locations_b.push(location);
                }
            })
        });
    locations_a.sort();
    locations_b.sort();
    let sum_value: u32 = locations_a.iter()
        .zip(locations_b.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    println!("Part1: {}", sum_value)
}
