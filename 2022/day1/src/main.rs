use std::io::BufRead;

fn main() {
    let dwarfs = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|line| line.unwrap().trim().parse::<u32>())
        .scan(0, |state, x| {
            match x {
                Ok(t) => {
                    *state += t;
                    Some(None)
                }
                Err(_) => {
                    let val = *state;
                    *state = 0;
                    Some(Some(val))
                }
            }
        })
        .filter(|val| val.is_some())
        .map(|val| val.unwrap());
    println!("max: {}", dwarfs.max().unwrap());
    //let dwarfes = dwarfs.collect().sort().

}