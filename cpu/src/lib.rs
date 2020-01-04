#[derive(Default)]
pub struct CPU{
    mem: Vec<i32>,
    ip: usize,
    inputs: Vec<i32>,
    output: i32,
}

impl CPU {
    pub fn new(mem: &[i32]) -> CPU{
        CPU{
            mem: mem.to_vec(),
            ip: 0,
            ..Default::default()
        }
    }

    pub fn feed(&mut self, input: i32){
        self.inputs.push(input);
    }

    pub fn starve(&self) -> i32{
        return self.output;
    }

    pub fn run(&mut self){
        loop {
            match self.read(self.ip) % 100 {
                1 => {
                    let res = self.arg(1) + self.arg(2);
                    self.write(self.read(self.ip + 3) as usize, res);
                    self.ip += 4;
                },
                2 => {
                    let res = self.arg(1) * self.arg(2);
                    self.write(self.read(self.ip + 3) as usize, res);
                    self.ip += 4;
                },
                3 => {
                    let val = self.input();
                    self.write(self.arg(1) as usize, val);
                    self.ip += 2;
                },
                4 => {
                    self.output(self.arg(1));
                    self.ip += 2;
                },
                99 => return,
                _ => panic!(self.read(self.ip)),
            }
        }
    }

    fn arg(&self, offset: usize) -> i32{
        return self.read(self.read(self.ip + offset) as usize);
    }
    
    fn input(&mut self) -> i32{
        return self.inputs.remove(0);
    }

    fn output(&mut self, value: i32){
        println!("output: {}", value);
        self.output = value
    }


    fn read(&self, addr: usize) -> i32{
        return self.mem[addr];
    }

    fn write(&mut self, addr: usize, value: i32){
        self.mem[addr] = value;
    }
}

pub fn process(program: &mut Vec<i32>) -> i32{
    let mut cpu = CPU::new(program);
    cpu.run();
    let res = cpu.starve();
    *program = cpu.mem;
    return res;
}

