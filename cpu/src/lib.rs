struct CPU{
    mem: Vec<i32>,
    ip: usize
}

impl CPU {
    pub fn new(mem: &[i32]) -> CPU{
        CPU{
            mem: mem.to_vec(),
            ip: 0
        }
    }

    pub fn run(&mut self){
    loop {
        let res;
        match self.mem[self.ip] {
            1 => res = self.arg(1) + self.arg(2),
            2 => res = self.arg(1) * self.arg(2),
            99 => return,
            _ => panic!(self.mem[self.ip]),
        }
        let residx = self.mem[self.ip + 3] as usize;
        self.mem[residx] = res;
        self.ip += 4;
    }
    }

    fn arg(&mut self, offset: usize) -> i32{
        return self.mem[self.mem[self.ip + offset] as usize];
    }
}

pub fn process(program: &mut Vec<i32>){
    let mut cpu = CPU::new(program);
    cpu.run();
    *program = cpu.mem;
}

#[test]
fn examples(){
    let mut pgm = vec![1,0,0,0,99];
    process(&mut pgm);
    println!("{:?}", pgm);
    assert_eq!(pgm[0], 2);

    pgm = vec![2,3,0,3,99];
    process(&mut pgm);
    assert_eq!(pgm[3], 6);

    pgm = vec![2,4,4,5,99,0];
    process(&mut pgm);
    assert_eq!(pgm[5], 9801);

    pgm = vec![1,1,1,4,99,5,6,0,99];
    process(&mut pgm);
    assert_eq!(pgm[0], 30);
    assert_eq!(pgm[4], 2);
}
