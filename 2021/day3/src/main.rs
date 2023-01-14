use std::io::BufRead;

fn main() {
    let numbers: Vec<_> = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|line| u64::from_str_radix(line.unwrap().trim(), 2).unwrap())
        .collect();
    let mut sum: Vec<u64> = vec![];
    let mut len = 0;
    for n in &numbers {
        let mut idx: u32 = 0;
        while n >= &2_u64.pow(idx) {
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
    }
    //println!("{}", sum.len());
    let gamma = sum
        .iter()
        .enumerate()
        .map(|(i, &n)| {
            //println!("{} {}", i, n);
            match n > len / 2 {
                true => 2_u64.pow(i as u32),
                false => 0,
            }
        })
        .sum::<u64>();
    let epsilon = (!gamma) & (2_u64.pow(sum.len() as u32) - 1);
    println!("{:b}, {:b}", gamma, epsilon);
    println!("part1: {}", gamma * epsilon);

    let oxygen = oxygen(&numbers);
    let co2 = co2(&numbers);
    println!("part2: {}", oxygen * co2);
}

fn oxygen(numbers: &Vec<u64>) -> u64 {
    let mut remainings = numbers.clone();
    let width = numbers
        .iter()
        .map(|x| (*x as f32).log2().ceil() as u32)
        .max()
        .unwrap();
    println!("width: {}", width);

    for idx in (0..width).rev() {
        let count = remainings
            .iter()
            .map(|x| (x >> idx) & 1)
            .filter(|x| *x == 1)
            .count();
        let cond = match count*2 >= remainings.len() {
            true => 1,
            false => 0,
        };
        remainings.retain(|x| (x >> idx) & 1 == cond);
        println!(
            "idx: {}, count: {}, cond: {}, remaining: {}",
            idx,
            count,
            cond,
            remainings.len()
        );
        if remainings.len() == 1 {
            break;
        }
    }
    println!("oxygen: {:b}", remainings.get(0).unwrap());
    *remainings.get(0).unwrap()
}


fn co2(numbers: &Vec<u64>) -> u64 {
    let mut remainings = numbers.clone();
    let width = numbers
        .iter()
        .map(|x| (*x as f32).log2().ceil() as u32)
        .max()
        .unwrap();
    println!("width: {}", width);

    for idx in (0..width).rev() {
        let count = remainings
            .iter()
            .map(|x| (x >> idx) & 1)
            .filter(|x| *x == 1)
            .count();
        let cond = match count*2 >= remainings.len() {
            true => 0,
            false => 1,
        };
        remainings.retain(|x| (x >> idx) & 1 == cond);
        println!(
            "idx: {}, count: {}, cond: {}, remaining: {}",
            idx,
            count,
            cond,
            remainings.len()
        );
        if remainings.len() == 1 {
            break;
        }
    }
    println!("co2: {:b}", remainings.get(0).unwrap());
    *remainings.get(0).unwrap()
}
