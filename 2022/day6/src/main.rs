use std::io::Read;

fn main() {
    let mut content = String::new();
    std::io::BufReader::new(std::io::stdin()).read_to_string(&mut content);
    let pos = get_start(&content);
    println!("start: {}", pos);
}

fn get_start(stream: &str) -> usize {
    stream
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .position(|w| {
            w[0] != w[1]
                && w[0] != w[2]
                && w[0] != w[3]
                && w[1] != w[2]
                && w[1] != w[3]
                && w[2] != w[3]
        })
        .unwrap() + 4
}

#[test]
fn test_start(){
    assert_eq!(7, get_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(5, get_start("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(6, get_start("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(10, get_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(11, get_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}