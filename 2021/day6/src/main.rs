use std::collections::HashMap;

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut population = line
        .trim()
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    // Let's take on from here.
    let mut pop2 = HashMap::new();
    for fish in &population {
        *pop2.entry(*fish).or_insert(0) += 1;
    }

    for _day in 0..256 {
        if _day < 80 {
            for i in 0..population.len() {
                if population[i] == 0 {
                    population.push(8); // we are iterating over the old length, so put 8 at the end (not 9).
                    population[i] = 6;
                } else {
                    population[i] -= 1;
                }
            }
        }

        for age in 0..10 {
            if !pop2.contains_key(&age) {
                continue;
            }
            if age == 0 {
                pop2.insert(9, pop2[&0]);
                *pop2.entry(7).or_default() += pop2[&0];
            } else {
                pop2.insert(age - 1, pop2[&age]);
            }
            pop2.remove(&age);
        }
        if _day < 20 || _day == 79 {
            if _day < 5{
              dbg!(&pop2);
            }
            println!(
                "Day: {}, pop: {}, pop2: {}",
                _day,
                population.len(),
                pop2.iter().map(|(_, v)| v).sum::<u64>()
            )
        }
    }
    let part2 = pop2.iter().map(|(_, v)| v).sum::<u64>();
    println!("part2: {}", part2);
}
