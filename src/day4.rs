pub fn run(){
    let input = "125730-579381";
    let input: Vec<&str> = input.split("-").collect();
    let min: i32 = input[0].trim().parse().unwrap();
    let max: i32 = input[1].trim().parse().unwrap();

    let mut count = 0;
    for value in min..max+1 {
        if valid(value) {
            count += 1;
        }
    }
    println!("Result: {}", count)
}

fn valid(value: i32)-> bool {
    let mut double = false;
    let mut prev = cipher(value, 0);
    for factor in 1..6{
        let val = cipher(value, factor);
        if prev == val {
            double = true;
        }
        if prev < val {
            return false;
        }
        prev = val;
    }
    if double{
        println!("Found: {}", value);
    }
    return double;
}

fn cipher(value: i32, idx: u32) -> i32 {
    let value = value / 10_i32.pow(idx);
    return value - value / 10 * 10;

}

#[test]
fn examples(){
    assert_eq!(cipher(123456, 3), 3);
    assert_eq!(valid(111111), false);
    assert_eq!(valid(223450), false);
    assert_eq!(valid(123789), false);

}
