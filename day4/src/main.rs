fn main(){
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
    let mut count = [0; 10];
    let mut double = false;
    let mut prev = cipher(value, 0);
    count[prev as usize] = 1;
    for factor in 1..6{
        let val = cipher(value, factor);
        count[val as usize] += 1;
        if prev == val {
            double = true;
        }
        if prev < val {
            return false;
        }
        prev = val;
    }
    if !double {
        return false;
    }
    for value in &count{
        if *value == 2{
            return true;
        }
    }
    return false;
}

fn cipher(value: i32, idx: u32) -> i32 {
    let value = value / 10_i32.pow(idx);
    return value - value / 10 * 10;

}

#[test]
fn intern(){
    assert_eq!(cipher(123456, 0), 6);
    assert_eq!(cipher(123456, 1), 5);
    assert_eq!(cipher(123456, 2), 4);
    assert_eq!(cipher(123456, 3), 3);
    assert_eq!(cipher(123456, 4), 2);
    assert_eq!(cipher(123456, 5), 1);
}
#[test]
fn examples(){
    assert_eq!(valid(111111), false);
    assert_eq!(valid(223450), false);
    assert_eq!(valid(123789), false);
}
#[test]
fn examples2(){
    assert_eq!(valid(112233), true);
    assert_eq!(valid(123444), false);
    assert_eq!(valid(111122), true);
}
