fn main(){

}

fn captcha(_val: &str) -> i32 {
    0
}

#[test]
fn examples() {
    assert_eq!(captcha("1122"), 3);
    assert_eq!(captcha("1111"), 4);
    assert_eq!(captcha("1234"), 0);
    assert_eq!(captcha("91212129"), 9)
}