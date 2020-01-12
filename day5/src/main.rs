use std::fs::File;
use std::io::Read;

use cpu;

fn main() {
    let mut f = File::open("day5.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let orig: Vec<i32> = content
        .split(",")
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let mut cpu = cpu::CPU::new(&orig);
    cpu.feed(5);
    cpu.run();
    println!("Diagnostic code: {}", cpu.starve());
}
