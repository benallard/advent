use std::io::BufRead;

fn main() {
    let mut sum : Vec<u64> = vec![];
    let mut len = 0;
    std::io::BufReader::new(std::io::stdin())
    .lines()
    .map(|line| u64::from_str_radix(line.unwrap().trim(), 2).unwrap())
    .for_each(|n| {
        let mut idx: u32 = 0;
        while n >= 2_u64.pow(idx) {
            //println!("{}, {}", n, 2_u64.pow(idx));
            if sum.get(idx as usize) == None {
                //println!("extending");
                sum.push(0);
            }
            if (n & 2_u64.pow(idx)) != 0 {
                sum[idx as usize] += 1;
            }
            idx += 1;
        }
        len += 1;
    });
    //println!("{}", sum.len());
    let gamma = sum.iter().enumerate().map(|(i, &n)| {
        println!("{} {}", i, n);
        match n > len/2 {
        true => 2_u64.pow(i as u32),
        false => 0
    }}).sum::<u64>();
    let epsilon = (!gamma) & (2_u64.pow(sum.len() as u32)- 1);
    println!("{:b}, {:b}", gamma, epsilon);
    println!("{}", gamma * epsilon);
}