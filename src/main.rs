use std::io;

fn main() {
    println!("Advent of code");

    let mut day = String::new();
    io::stdin().read_line(&mut day)
        .expect("No line to read");

    println!("Running day {}", day);
}
