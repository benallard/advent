use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Part1: {}", captcha1(&input));
    println!("Part2: {}", captcha2(&input));
}

fn captcha1(val: &str) -> u32 {
    //val.push(val.chars().nth(0));
    val.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .map(|w| if w[0] == w[1] { w[0] } else { 0 })
        .sum::<u32>()
}

fn captcha2(val: &str) -> u32 {
    let mut list1 = val.chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let list2 = list1.split_off(list1.len() / 2);
    let res: u32 = list1.iter().zip(list2.iter())
    .map(|(a, b)| if a == b {*a} else {0})
    .sum();
    res * 2
}

#[test]
fn examples() {
    assert_eq!(captcha1("1122"), 3);
    assert_eq!(captcha1("1111"), 3); // manually add 1 please
    assert_eq!(captcha1("1234"), 0);
    assert_eq!(captcha1("91212129"), 0); // same here, add 9 manually
}

#[test]
fn examples2() {
    assert_eq!(captcha2("1212"), 6);
    assert_eq!(captcha2("1221"), 0);
    assert_eq!(captcha2("123425"), 4);
    assert_eq!(captcha2("123123"), 12);
    assert_eq!(captcha2("12131415"), 4);
}
