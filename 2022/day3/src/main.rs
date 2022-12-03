use std::collections::HashSet;
use std::io::BufRead;
use std::iter::FromIterator;

fn main() {
    let sum = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|line| line.unwrap().trim().into())
        .collect::<Vec<String>>()
        //.map(|line| share(line.unwrap().trim()))
        .chunks(3)
        .map(|lines| {
            let dwarf1:HashSet<_> = HashSet::from_iter(lines[0].chars().into_iter());
            let dwarf2 = HashSet::from_iter(lines[1].chars().into_iter());
            let dwarf3 = HashSet::from_iter(lines[2].chars().into_iter());
            let common12 = dwarf1.intersection(&dwarf2).map(|c| *c).collect::<HashSet<_>>();
            let mut common = common12.intersection(&dwarf3);
            *common.next().unwrap()
        })
        .map(|item| prio(item))
        .fold(0, |a, b| a + b);
    println!("sum: {}", sum)
}

fn share(content: &str) -> char{
    let part1:HashSet<char> = HashSet::from_iter(content[..content.len() / 2].chars().into_iter());
    let part2:HashSet<char> = HashSet::from_iter(content[content.len() / 2..].chars().into_iter());
    let mut common = part1.intersection(&part2);
    *common.next().unwrap()
}

fn prio(item: char) -> u32{
    dbg!(item);
    match item {
        'a' ..= 'z' => item.to_digit(36).unwrap() - 9,
        'A' ..= 'Z' => item.to_digit(36).unwrap() - 9 + 26,
        _ => panic!()
    }
    
}

#[test]
fn testpart1(){
    assert_eq!('p', share("vJrwpWtwJgWrhcsFMMfFFhFp"));
    assert_eq!(16, prio('p'));
    assert_eq!('L', share("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
    assert_eq!(38, prio('L'));

    assert_eq!('P', share("PmmdzqPrVvPwwTWBwg"));
    assert_eq!(42, prio('P'));

    assert_eq!('v', share("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
    assert_eq!(22, prio('v'));

    assert_eq!('t', share("ttgJtRGJQctTZtZT"));
    assert_eq!(20, prio('t'));

    assert_eq!('s', share("CrZsJsPPZsGzwwsLwLmpwMDw"));
    assert_eq!(19, prio('s'));
}