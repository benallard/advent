use std::fs::File;
use std::io::Read;

use cpu;

fn main() {
    let mut f = File::open("day9.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let orig: Vec<i64> = content
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut cpu = cpu::CPU::new(&orig);
    cpu.feed(1);
    cpu.run();
    loop {
        let res = cpu.starve();
        if res.is_none() {
            break;
        }
        println!("Error?: {}", res.unwrap());
    }
}
