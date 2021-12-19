use std::fs::File;
use std::io::Read;

use cpu::CPU;

fn main() {
    let mut f = File::open("day7.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    println!("Max output: {}", get_max(&content, 0, 5));
    println!("With feedback: {}", get_max(&content, 5, 10));
}

fn get_max(content: &str, x: u8, y: u8) -> i64 {
    let mut res = 0;
    for a in x..y {
        for b in x..y {
            if b == a {
                continue;
            };
            for c in x..y {
                if c == a {
                    continue;
                };
                if c == b {
                    continue;
                };
                for d in x..y {
                    if d == a {
                        continue;
                    };
                    if d == b {
                        continue;
                    };
                    if d == c {
                        continue;
                    };
                    for e in x..y {
                        if e == a {
                            continue;
                        };
                        if e == b {
                            continue;
                        };
                        if e == c {
                            continue;
                        };
                        if e == d {
                            continue;
                        };
                        let o = setup(content, vec![a, b, c, d, e]);
                        if o > res {
                            res = o;
                        }
                    }
                }
            }
        }
    }
    return res;
}

fn amplifier(pgm: &str, phase: u8) -> CPU {
    let orig: Vec<i64> = pgm.split(",").map(|s| s.trim().parse().unwrap()).collect();
    let mut cpu = CPU::new(&orig);
    cpu.feed(phase as i64);
    return cpu;
}

fn setup(pgm: &str, phase: Vec<u8>) -> i64 {
    let mut amp_a = amplifier(pgm, phase[0]);
    let mut amp_b = amplifier(pgm, phase[1]);
    let mut amp_c = amplifier(pgm, phase[2]);
    let mut amp_d = amplifier(pgm, phase[3]);
    let mut amp_e = amplifier(pgm, phase[4]);
    let mut seed = 0;
    loop {
        amp_a.feed(seed);
        amp_a.run();
        amp_b.feed(amp_a.starve().unwrap());
        amp_b.run();
        amp_c.feed(amp_b.starve().unwrap());
        amp_c.run();
        amp_d.feed(amp_c.starve().unwrap());
        amp_d.run();
        amp_e.feed(amp_d.starve().unwrap());
        amp_e.run();
        seed = amp_e.starve().unwrap();
        if amp_e.done() {
            break;
        }
    }
    return seed;
}

#[test]
fn test1() {
    assert_eq!(
        43210,
        setup(
            "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
            vec![4, 3, 2, 1, 0]
        )
    );
    assert_eq!(
        43210,
        get_max("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 0, 5)
    );
    assert_eq!(
        54321,
        get_max(
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
            0,
            5
        )
    );
    assert_eq!(65210, get_max("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", 0, 5));
}

#[test]
fn test2() {
    assert!(139629729 < std::i64::MAX);
    assert_eq!(
        139629729,
        get_max(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
            5,
            10
        )
    );
    assert_eq!(18216, get_max("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10", 5, 10))
}
