use std::io::BufRead;

fn main() {
    let mut x: i32 = 1;
    let mut vec = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().into())
        .flat_map(|l: String| {
            let t: Vec<_> = l.split(" ").collect();
            let mut res = vec![];
            res.push(x);
            if t[0] == "addx" {
                x += t[1].parse::<i32>().unwrap();
                res.push(x);
            }
            res.into_iter()
        })
        .collect::<Vec<_>>();
    // add the initial value
    vec.insert(0, 1);
    // drop the last one
    vec.pop();

    let res: i32 = vec
        .chunks(40)
        .enumerate()
        .map(|(i, c)| {
            //println!("{}, {}", 20+i*40, c[19]);
            c[19] * (20 + i * 40) as i32
        })
        .sum();
    println!("Sum: {}", res);

    vec.chunks(40)
        // part 2
        .for_each(|l| {
            l.iter().enumerate().for_each(|(j, c)| {
                let spot: i32 = (j + 1) as i32;
                //println!("cycle: {}, sprite: {}-{}", spot, *c, *c + 2);
                if (*c..c + 3).contains(&spot) {
                    print!("#");
                } else {
                    print!(".");
                }
            });
            println!("");
        });
}
