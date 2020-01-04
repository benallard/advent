use cpu::{CPU, process};

#[test]
fn io(){
    let mut cpu = CPU::new(&[3,0,4,0,99]);
    cpu.feed(42);
    cpu.run();
    assert_eq!(cpu.starve(), 42);
}

#[test]
fn mode(){
    process(&mut vec![1002, 4, 3, 4, 33]);

}

#[test]
fn neg(){
    process(&mut vec![1101, 100, -1, 4, 0]);
}
