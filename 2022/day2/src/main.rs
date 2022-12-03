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
            calc_score_2(hands[0], hands[1])
        })
        .fold(0, |a, b| a + b);
    println!("score: {}", score);
}

fn calc_score_1(dwarf: char, me: char) -> i32{
    calc_score(dwarf, match me {
        'X' => 'A',
        'Y' => 'B',
        'Z' => 'C',
        _ => panic!()
    })
}

fn calc_score(dwarf: char, me: char) -> i32{
    let outcome = match dwarf{
        // Rock
        'A' => match me {
            'A' => 3, // Rock
            'B' => 6, // Paper
            'C' => 0, // Scissors
            _ => panic!()
        }
        // Paper
        'B' => match me {
            'A' => 0, // Rock
            'B' => 3, // Paper
            'C' => 6, // Scissors
            _ => panic!()
        }
        // Scissors
        'C' => match me {
            'A' => 6, // Rock
            'B' => 0, // Paper
            'C' => 3, // Scissors
            _ => panic!()
        }
        _ => panic!()
    };
    outcome + match me {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => panic!()
    }
}

fn calc_score_2(dwarf: char, result: char) -> i32{
    let me = match dwarf{
        'A' => match result{
            'X' => 'C',
            'Y' => 'A',
            'Z' => 'B',
            _ => panic!()
        }
        'B' => match result{
            'X' => 'A',
            'Y' => 'B',
            'Z' => 'C',
            _ => panic!()
        }
        'C' => match result{
            'X' => 'B',
            'Y' => 'C',
            'Z' => 'A',
            _ => panic!()
        }
        _ => panic!()
    };
    calc_score(dwarf, me)
}

#[test]
fn part1(){
    assert_eq!(8, calc_score_1('A', 'Y'));
    assert_eq!(1, calc_score_1('B', 'X'));
    assert_eq!(6, calc_score_1('C', 'Z'));
}

#[test]
fn part2(){
    assert_eq!(4, calc_score_2('A', 'Y'));
    assert_eq!(1, calc_score_2('B', 'X'));
    assert_eq!(7, calc_score_2('C', 'Z'));
}