use std::io;

mod day1;

fn main() {
    println!("Advent of code");
    println!("Day ?");

    let mut day = String::new();
    io::stdin().read_line(&mut day)
        .expect("No line to read");

    let day : u32 = day.trim().parse()
        .expect("No number given");

    println!("Running day {}", day);

    match day {
        1 => day1::run(),
        _ => println!("Not implemented"),
    }
}
