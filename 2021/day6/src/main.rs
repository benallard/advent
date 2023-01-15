fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut population = line.trim().split(",").map(|n| n.parse::<u8>().unwrap()).collect::<Vec<_>>();

    for _day in 0..80{
        for i in 0..population.len(){
            if population[i] == 0{
                population.push(8);
                population[i] = 6;
            } else {
                population[i] -= 1;
            }
        }
        //println!("Day: {}: {:?}", _day,population);
    }
    println!("part1: {}", population.len());
}
