use std::fs::File;
use std::io::Read;

use cpu;

fn main() {
    let mut f = File::open("day2.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let orig: Vec<i64> = content
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = orig.to_vec();
            program[1] = noun;
            program[2] = verb;
            cpu::process(&mut program);
            if program[0] == 19690720 {
                println!("Found! noun={}, verb={}", noun, verb);
                return;
            }
        }
    }

    println!("Not found");
}

// cpu was moved to the cpu module
