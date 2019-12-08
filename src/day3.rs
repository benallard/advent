use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn run() {
  let f = File::open("day3.txt").unwrap();
  let mut f = BufReader::new(f);
  let mut path1 = String::new();
  f.read_line(&mut path1).unwrap();
  let path1 = path1.split(",").collect();

  let mut path2 = String::new();
  f.read_line(&mut path2).unwrap();
  let path2 = path2.split(",").collect();
  
  let res = nearest_intersection(path1, path2);
  println!("Shortest distance: {}", res);
}

fn nearest_intersection(path1: Vec<&str>, path2: Vec<&str>) -> i32{
  let points1 = gather_points(path1);
  let points2 = gather_points(path2);
  let mut min = std::i32::MAX;
  for (point, length) in &points1 {
      if !points2.contains_key(point){
          continue;
      }
      let distance = length + points2[point];
      if distance < min {
          min = distance;
      }
  }
  return min;
}

fn gather_points(path: Vec<&str>) -> HashMap<Point, i32>{
   let mut points = HashMap::new();
   let mut current = Point{x: 0, y: 0};
   let mut length = 0;
   for part in &path {
       let amount = part[1..].trim().parse().expect(part);
       match &part[0..1]{
           "U" => up(&mut current, amount, &mut length, &mut points),    
           "D" => down(&mut current, amount, &mut length, &mut points),
           "L" => left(&mut current, amount, &mut length, &mut points),    
           "R" => right(&mut current, amount, &mut length, &mut points),
           _ => return points,
       }
   }
   return points;
}

fn up(current: &mut Point, amount: i32, length: &mut i32, array: &mut HashMap<Point, i32>){
    for _i in 1..amount+1 {
        current.y += 1;
        *length += 1;
        array.entry(Point{x: current.x, y: current.y}).or_insert(*length);
    }
}
fn down(current: &mut Point, amount: i32, length: &mut i32, array: &mut HashMap<Point, i32>){
    for _i in 1..amount+1 {
        current.y -= 1;
        *length += 1;
        array.entry(Point{x: current.x, y: current.y}).or_insert(*length);
    }
}
fn left(current: &mut Point, amount: i32, length: &mut i32, array: &mut HashMap<Point, i32>){
    for _i in 1..amount+1 {
        current.x -= 1;
        *length += 1;
        array.entry(Point{x: current.x, y: current.y}).or_insert(*length);
    }
}
fn right(current: &mut Point, amount: i32, length: &mut i32, array: &mut HashMap<Point, i32>){
    for _i in 1..amount+1 {
        current.x += 1;
        *length += 1;
        array.entry(Point{x: current.x, y: current.y}).or_insert(*length);
    }
}

#[test]
fn examples(){
    assert_eq!(nearest_intersection(vec!["R8", "U5", "L5", "D3"], vec!["U7", "R6", "D4", "L4"]), 30);
    assert_eq!(nearest_intersection(vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"], vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"]), 610);
    assert_eq!(nearest_intersection(vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51"], vec!["U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"]), 410);
}
