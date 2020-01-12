use cpu::{process, CPU};

// Part 1

#[test]
fn quine() {
    let pgm = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let mut cpu = CPU::new(&pgm);
    cpu.run();
    for v in pgm {
        assert_eq!(v, cpu.starve().unwrap());
    }
    assert!(cpu.done());
}

#[test]
fn big_number() {
    let res = process(&mut vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
    assert!(res.unwrap() > 0xffff);
}

#[test]
fn big_number2() {
    let res = process(&mut vec![104, 1125899906842624, 99]);
    assert_eq!(Some(1125899906842624), res);
}
