use std::io;

mod day1;
mod day2; 
mod day3;
mod day4;

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
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        _ => println!("Not implemented"),
    }
}
