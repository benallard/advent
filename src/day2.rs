use std::io::Read;
use std::fs::File;

pub fn run() {
  let mut f = File::open("day2.txt").unwrap();
  let mut content = String::new();
  f.read_to_string(&mut content).unwrap();

  let program:Vec<i32> = content.split(",").map(|s| s.trim().parse().unwrap()).collect();

  process(&program);

  println!("Result: {}", program[0])
}

fn process(program:&Vec<i32>){
}
