use std::io::Read;
use std::fs::File;

use cpu::CPU;

fn main() {
  let mut f = File::open("day7.txt").unwrap();
  let mut content = String::new();
  f.read_to_string(&mut content).unwrap();

  println!("Max output: {}", get_max(&content));
}

fn get_max(content: &str) -> i32{
    let mut res = 0;
    for a in 0..5{
        for b in 0..5 {
            if b == a {continue};
            for c in 0..5 {
                if c == a {continue};
                if c == b {continue};
                for d in 0..5 {
                    if d == a {continue};
                    if d == b {continue};
                    if d == c {continue};
                    for e in 0..5 {
                        if e == a {continue};
                        if e == b {continue};
                        if e == c {continue};
                        if e == d {continue};
                        let o = setup(content, vec![a,b,c,d,e]);
                        if o > res{
                            res = o;
                        }
                    }
                }
            }
        }
    }
    return res;
}

fn amplifier(pgm: &str, phase: u8) -> CPU{
  let orig:Vec<i32> = pgm.split(",").map(|s| s.trim().parse().unwrap()).collect();
  let mut cpu = CPU::new(&orig);
  cpu.feed(phase as i32);
  return cpu
}

fn setup(pgm: &str, phase: Vec<u8>) -> i32{
  let mut amp_a = amplifier(pgm, phase[0]);
  amp_a.feed(0);
  amp_a.run();
  let mut amp_b = amplifier(pgm, phase[1]);
  amp_b.feed(amp_a.starve());
  amp_b.run();
  let mut amp_c = amplifier(pgm, phase[2]);
  amp_c.feed(amp_b.starve());
  amp_c.run();
  let mut amp_d = amplifier(pgm, phase[3]);
  amp_d.feed(amp_c.starve());
  amp_d.run();
  let mut amp_e = amplifier(pgm, phase[4]);
  amp_e.feed(amp_d.starve());
  amp_e.run();
  return amp_e.starve();
}

#[test]
fn test1(){
    assert_eq!(43210, setup("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", vec![4,3,2,1,0]));
    assert_eq!(43210, get_max("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"));
    assert_eq!(54321, get_max("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"));
    assert_eq!(65210, get_max("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"));

}
