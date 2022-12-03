use std::io::BufRead;

fn main() {
    let score = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|line| line.unwrap().trim().into())
        .map(|line: String| {
            let hands: Vec<_> = line
                .split(" ")
                .map(|c| c.chars().nth(0).unwrap())
                .collect();
            calc_score(hands[0], hands[1])
        })
        .fold(0, |a, b| a + b);
    println!("score: {}", score);
}

fn calc_score(dwarf: char, me: char) -> i32{
    let outcome = match dwarf{
        // Rock
        'A' => match me {
            'X' => 3, // Rock
            'Y' => 6, // Paper
            'Z' => 0, // Scissors
            _ => panic!()
        }
        // Paper
        'B' => match me {
            'X' => 0, // Rock
            'Y' => 3, // Paper
            'Z' => 6, // Scissors
            _ => panic!()
        }
        // Scissors
        'C' => match me {
            'X' => 6, // Rock
            'Y' => 0, // Paper
            'Z' => 3, // Scissors
            _ => panic!()
        }
        _ => panic!()
    };
    outcome + match me {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!()
    }
}

#[test]
fn examples(){
    assert_eq!(8, calc_score('A', 'Y'));
    assert_eq!(1, calc_score('B', 'X'));
    assert_eq!(6, calc_score('C', 'Z'));
}