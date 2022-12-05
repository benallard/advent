use std::io::BufRead;

fn main(){
    let lines:Vec<_> = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let mut ship = Ship::new(lines[0].len() / 3);
    let mut end = 0;
    for line in &lines {
        end += 1;
        if line.trim().len() == 0 {
            break;
        }
        for (idx, char) in line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .enumerate(){
            if char[0] == '[' {
                ship.add_crate(char[1], idx);
            }
        }
    }

    ship.draw();

    for line in &lines[end..]{
        // 'move 1 from 2 to 1'
        let chunks: Vec<_> = line.split(" ").collect();
        if chunks[0] != "move"{
            continue;
        }
        let amount = chunks[1 as usize].parse().unwrap();
        let source = chunks[3 as usize].parse().unwrap();
        let dest = chunks[5 as usize].parse().unwrap();
        ship.move_crates_2(amount, source, dest);
        println!(": {}", line);
        ship.draw();
    }

    ship.draw();

    println!("tops: {}", ship.tops())
}

struct Ship {
    stacks : Vec<Vec<char>>,
}

#[allow(dead_code)]
impl Ship{
    pub fn new(stacksize: usize) -> Ship{
        let mut stacks = Vec::with_capacity(stacksize);
        for _ in 0..stacksize{
            stacks.push(Vec::new())
        }
        Ship{
            stacks: stacks,
        }
    }

    pub fn add_crate(&mut self, value: char, idx: usize){
        // Crates are initialized from the top
        self.stacks[idx].insert(0, value);
    }


    pub fn move_crates_2(&mut self, amount: usize, source: usize, dest:usize){
        let drain_start = self.stacks[source-1].len() - amount;
        let mut values : Vec<_> = self.stacks[source-1]
            .drain(drain_start..)
            .collect();
        self.stacks[dest - 1].append(&mut values);
    }

    pub fn move_crates_1(&mut self, amount: u8, source: usize, dest:usize){
        for _ in 0..amount{
            self.move_crate(source, dest);
        }
    }

    pub fn move_crate(&mut self, source: usize, dest: usize){
        let value = self.stacks[source-1].pop().unwrap();
        self.stacks[dest-1].push(value);
    }

    pub fn tops(&self) -> String{
        self.stacks.iter()
            .map(|s| s.last())
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect()
    }

    pub fn draw(&self){
        let highest = self.stacks.iter().map(|s| s.len()).max().unwrap();
        for height in (0..highest).rev() {
            for stack in &self.stacks {
                if stack.len() > height{
                    print!("[{}] ", stack[height]);
                } else {
                    print!("    ");
                }
            }
            println!("");
        }
        for idx in 0..self.stacks.len() {
            print!(" {}  ", idx + 1);
        }
        println!("");
    }
}