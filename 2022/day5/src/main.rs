use std::io::BufRead;

fn main(){
    let lines:Vec<_> = std::io::BufReader::new(std::io::stdin())
        .lines().map(|l| l.unwrap()).collect();
    let mut ship = Ship::new(lines[0].len() / 3);
    for line in lines {
        if line.trim().len() == 0 {
            break;
        }
        for (idx, char) in line.chars().collect::<Vec<char>>().chunks(4).enumerate(){
            if char[0] == '[' {
                ship.add_crate(char[1], idx);
            }
        }
    }

    ship.draw();




}

struct Ship {
    stacks : Vec<Vec<char>>,
}

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
        dbg!(value, idx);
        self.stacks[idx].insert(0, value);
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