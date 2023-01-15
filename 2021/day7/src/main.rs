fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let crabs = line
        .trim()
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    let min_fuel: u32 = (min..=max)
        .into_iter()
        .map(|pos| crabs.iter().map(|c| c.abs_diff(pos)).sum())
        .min()
        .unwrap();
    println!("part1: {}", min_fuel);
    let min_fuel: u32 = (min..=max)
        .into_iter()
        .map(|pos| crabs.iter().map(|c| costs(*c, pos)).sum())
        .min()
        .unwrap();
    println!("part2: {}", min_fuel);
}

fn costs(from: u32, to: u32) -> u32 {
    let dist = from.abs_diff(to);
    //(0..=dist).into_iter().sum()
    ((dist + 1) as f32 * (dist as f32 / 2.)).round() as u32
}

#[test]
fn test_fuel_pt_2() {
    assert_eq!(66, costs(16, 5));
    assert_eq!(10, costs(1, 5));
    assert_eq!(6, costs(2, 5));
    assert_eq!(15, costs(0, 5));
    assert_eq!(1, costs(4, 5));
    assert_eq!(3, costs(7, 5));
    assert_eq!(45, costs(14, 5));
}
