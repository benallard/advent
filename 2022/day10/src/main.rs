use std::{io::BufRead};

fn main() {
    let mut x:i32 = 1;
    let res: i32 = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().into())
        .flat_map(|l: String| {
            let t: Vec<_> = l.split(" ").collect();
            let mut res = vec![];
            res.push(x);
            if t[0] == "addx"{
                x += t[1].parse::<i32>().unwrap();
                res.push(x);
            }
            res.into_iter()
        })
        .collect::<Vec<_>>()
        .chunks(40).enumerate()
        .map(|(i, c)| {
            //println!("{}, {}", 20+i*40, c[18]);
            c[18] * (20+i*40) as i32
        })
        .sum();
    println!("Sum: {}", res);
}
