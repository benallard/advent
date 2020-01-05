use std::iter::Iterator;
use std::collections::HashMap;

fn main(){
    let orbits = parse_orbits(include_str!("../../day6.txt").lines().collect());
    println!("Orbit amounts: {}", count_orbits(&orbits));
    println!("Transfers: {}", min_transfers(&orbits, "YOU", "SAN"));
}

fn parse_orbits(lines: Vec<&str>) -> HashMap<&str, &str>{
    let mut map = HashMap::new();
    for line in lines{
        let objects: Vec<&str> = line.split(")").collect();
        map.insert(objects[1], objects[0]);
    }
    return map;
}

fn dist<'a>(map: &'a HashMap<&'a str, &'a str>, object: &'a str) -> Vec<&'a str>{
    let mut res = Vec::new();
    let mut obj = object;
    while map.contains_key(obj){
        res.push(obj.clone());
        obj = map[obj];
    }
    return res;
}

fn count_orbits(map: &HashMap<&str, &str>) -> usize{
    let mut res = 0;
    for obj in map.keys(){
        res += dist(map, obj).len();
    }
    return res;
}

fn min_transfers(map: &HashMap<&str, &str>, start: &str, end: &str) -> usize{
    let mut path1 = dist(map, start);
    path1.reverse();
    let mut path2 = dist(map, end);
    path2.reverse();
    let mut common = 0;
    for (i, part) in path2.iter().enumerate(){
        if &path1[i] != part{
            break;
        }
        common += 1;
    }

    return path1.len() - common + path2.len() - common - 2;
}

#[test]
fn test1(){
    let input = "COM)B B)C D)E E)F B)G G)H D)I E)J C)D J)K K)L".replace(" ", "\n");
    let orbits = parse_orbits(input.lines().collect());
    assert_eq!(count_orbits(&orbits), 42);
}
#[test]
fn test2(){
    let input = "COM)B B)C D)E E)F B)G K)YOU G)H D)I E)J C)D J)K K)L I)SAN".replace(" ", "\n");
    let orbits = parse_orbits(input.lines().collect());
    println!("{:?}", orbits);
    assert_eq!(min_transfers(&orbits, "YOU", "SAN"), 4);
}
