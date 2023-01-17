use std::{io::BufRead, str::FromStr};

fn main() {
    let lines = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().to_owned())
        .collect::<Vec<_>>();

    let numbers = lines[0]
        .split(",")
        .into_iter()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut boards = lines[1..]
        .chunks(6)
        .into_iter()
        .map(|c| c[1..].join("\n").parse::<Board>().unwrap())
        .collect::<Vec<_>>();

    //'outer:
    for number in numbers {
        for board in boards.iter_mut() {
            if board.done() {
                continue;
            }
            board.play(number);
            if board.done() {
                let sum = board.sum();
                println!("Score: {}", sum * number);
                //break 'outer;
            }
        }
    }
}

struct Board {
    board: [[Option<u32>; 5]; 5],
}

impl FromStr for Board {
    type Err = String;
    fn from_str(content: &str) -> Result<Self, <Self as FromStr>::Err> {
        let lines: Vec<_> = content.split("\n").collect();
        if lines.len() != 5 {
            return Err(format!("Wrong Board length: {}", lines.len()));
        }
        let mut board = [[None; 5]; 5];
        for (j, line) in lines.into_iter().enumerate() {
            for (i, n) in line.split_whitespace().enumerate() {
                board[j][i] = Some(n.parse().unwrap());
            }
        }
        Ok(Board { board })
    }
}

impl Board {
    fn play(&mut self, n: u32) {
        for j in 0..5 {
            for i in 0..5 {
                if self.board[j][i] == Some(n) {
                    self.board[j][i] = None;
                }
            }
        }
    }

    fn done(&self) -> bool {
        for line in self.board {
            if line.iter().filter(|o| o.is_none()).count() == 5 {
                return true;
            }
        }
        for i in 0..5 {
            if self
                .board
                .iter()
                .map(|l| l[i])
                .filter(|o| o.is_none())
                .count()
                == 5
            {
                return true;
            }
        }
        false
    }

    fn sum(&self) -> u32 {
        self.board
            .iter()
            .flatten()
            .filter(|n| n.is_some())
            .map(|o| o.unwrap())
            .sum()
    }
}
