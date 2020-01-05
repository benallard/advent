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
            match self.opcode() {
                1 => {
                    let res = self.get(1) + self.get(2);
                    self.put(3, res);
                    self.ip += 4;
                },
                2 => {
                    let res = self.get(1) * self.get(2);
                    self.put(3, res);
                    self.ip += 4;
                },
                3 => {
                    let val = self.input();
                    self.put(1, val);
                    self.ip += 2;
                },
                4 => {
                    self.output(self.get(1));
                    self.ip += 2;
                },
                5 => {
                    let val = self.get(1);
                    if val != 0 {
                        self.ip = self.get(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                },
                6 => {
                    let val = self.get(1);
                    if val == 0 {
                        self.ip = self.get(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                },
                7 => {
                    if self.get(1) < self.get(2){
                        self.put(3, 1);
                    } else {
                        self.put(3, 0);
                    }
                    self.ip += 4;
                },
                8 => {
                    if self.get(1) == self.get(2){
                        self.put(3, 1);
                    } else {
                        self.put(3, 0);
                    }
                    self.ip += 4;
                },
                99 => return,
                _ => panic!(self.read(self.ip)),
            }
        }
    }

    fn opcode(&self)-> u16{
        return (self.read(self.ip) % 100) as u16;
    }

    fn mode(&self, offset: usize) -> u8 {
        let val = self.read(self.ip) / 100;
        return (val / 10i32.pow((offset - 1) as u32) % 10) as u8;
    }

    fn get(&self, offset: usize) -> i32{
        let value = self.read(self.ip + offset);
        return match self.mode(offset){
            0=> self.read(value as usize),
            1=> value,
            _ => panic!(self.read(self.ip)),
        };
    }

    fn put(&mut self, offset: usize, value: i32){
        self.write(self.read(self.ip + offset) as usize, value);
    }
    
    fn input(&mut self) -> i32{
        //println!("Input is {}", self.inputs[0]);
        return self.inputs.remove(0);
    }

    fn output(&mut self, value: i32){
        //println!("output: {}", value);
        self.output = value
    }

    fn read(&self, addr: usize) -> i32{
        //println!("Reading {} @{}", self.mem[addr], addr);
        return self.mem[addr];
    }

    fn write(&mut self, addr: usize, value: i32){
        //println!("Writing {} to {}", value, addr);
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

