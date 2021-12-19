#[derive(Default)]
pub struct CPU {
    mem: Vec<i64>,
    ip: usize,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
    done: bool,
    relative_base: i64,
}

impl CPU {
    pub fn new(mem: &[i64]) -> CPU {
        CPU {
            mem: mem.to_vec(),
            ip: 0,
            ..Default::default()
        }
    }

    pub fn feed(&mut self, input: i64) {
        self.inputs.push(input);
    }

    pub fn starve(&mut self) -> Option<i64> {
        let res = self.outputs.get(0).copied();
        if res.is_some() {
            self.outputs.remove(0);
        }
        return res;
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn run(&mut self) {
        loop {
            match self.opcode() {
                1 => {
                    let res = self.get(1) + self.get(2);
                    self.put(3, res);
                    self.ip += 4;
                }
                2 => {
                    let res = self.get(1) * self.get(2);
                    self.put(3, res);
                    self.ip += 4;
                }
                3 => {
                    if self.inputs.is_empty() {
                        //println!("Input required");
                        break;
                    }
                    let val = self.input();
                    self.put(1, val);
                    self.ip += 2;
                }
                4 => {
                    self.output(self.get(1));
                    self.ip += 2;
                }
                5 => {
                    let val = self.get(1);
                    if val != 0 {
                        self.ip = self.get(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    let val = self.get(1);
                    if val == 0 {
                        self.ip = self.get(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    if self.get(1) < self.get(2) {
                        self.put(3, 1);
                    } else {
                        self.put(3, 0);
                    }
                    self.ip += 4;
                }
                8 => {
                    if self.get(1) == self.get(2) {
                        self.put(3, 1);
                    } else {
                        self.put(3, 0);
                    }
                    self.ip += 4;
                }
                9 => {
                    self.relative_base += self.get(1);
                    self.ip += 2;
                }
                99 => {
                    self.done = true;
                    break;
                }
                _ => panic!(self.read(self.ip)),
            }
        }
    }

    fn opcode(&self) -> u8 {
        return (self.read(self.ip) % 100) as u8;
    }

    fn mode(&self, offset: usize) -> u8 {
        let val = self.read(self.ip) / 100;
        return (val / 10_i64.pow((offset - 1) as u32) % 10) as u8;
    }

    fn get(&self, offset: usize) -> i64 {
        let value = self.read(self.ip + offset);
        return match self.mode(offset) {
            0 => self.read(value as usize),                        // position
            1 => value,                                            // value
            2 => self.read((self.relative_base + value) as usize), // relative
            _ => panic!(self.read(self.ip)),
        };
    }

    fn put(&mut self, offset: usize, value: i64) {
        let addr = self.read(self.ip + offset);
        match self.mode(offset) {
            0 => self.write(addr as usize, value),
            2 => self.write((self.relative_base + addr) as usize, value),
            _ => panic!(self.read(self.ip)),
        }
    }

    fn input(&mut self) -> i64 {
        //println!("Input is {}", self.inputs[0]);
        return self.inputs.remove(0);
    }

    fn output(&mut self, value: i64) {
        //println!("output: {}", value);
        self.outputs.push(value);
    }

    fn read(&self, addr: usize) -> i64 {
        //println!("Reading {} @{}", self.mem[addr], addr);
        return self.mem.get(addr).copied().unwrap_or(0);
    }

    fn write(&mut self, addr: usize, value: i64) {
        //println!("Writing {} to {}", value, addr);
        self.mem.resize(self.mem.len().max(addr + 1), 0);
        self.mem[addr] = value;
    }
}

pub fn process(program: &mut Vec<i64>) -> Option<i64> {
    let mut cpu = CPU::new(program);
    cpu.run();
    let res = cpu.starve();
    *program = cpu.mem;
    return res;
}
