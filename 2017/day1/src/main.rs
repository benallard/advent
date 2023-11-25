use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Part1: {}", captcha(&input))
}

fn captcha(val: &str) -> u32 {
    //val.push(val.chars().nth(0));
    val.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .map(|w| if w[0] == w[1] {w[0]} else {0})
        .sum::<u32>()
}

#[test]
fn examples() {
    assert_eq!(captcha("1122"), 3);
    assert_eq!(captcha("1111"), 4);
    assert_eq!(captcha("1234"), 0);
    assert_eq!(captcha("91212129"), 9)
}