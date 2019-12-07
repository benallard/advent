use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn run(){
    let f = File::open("day1.txt").unwrap();
    let f = BufReader::new(f);

    let mut res = 0;
    for line in f.lines(){
        res += total_fuel(line.unwrap().trim().parse().unwrap());
    }

    println!("Required: {}", res);
}

fn total_fuel(mass: i32) -> i32{
   let mut res = fuel(mass);
   let mut extra = fuel(res);
   loop{
       if extra <= 0 {
           break;
       }
       res += extra;
       extra = fuel(extra);
   }
   return res;
}

fn fuel (mass: i32) -> i32{
    return mass / 3 - 2;
}

#[test]
fn examples(){
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100756), 33583)
}

#[test]
fn examplesi2(){
    assert_eq!(total_fuel(14), 2);
    assert_eq!(total_fuel(1969), 966);
    assert_eq!(total_fuel(100756), 50346)
}
