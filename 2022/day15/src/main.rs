use std::{collections::HashSet, io::BufRead, str::FromStr};

fn main() {
    let mut map = Map { sensors: vec![] };
    std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().to_owned())
        .for_each(|l| map.sensors.push(l.parse().unwrap()));

    let val = 10; // test
    let val = 2000000;

    let (left, right) = map.part1(val);
    println!(
        "part1: {}",
        right.abs_diff(left) + 1 - map.beacon_amount(val)
    );

    let val = 20; // test
    let val = 4000000;

    for y in 0..val {
        if let Some(b) = map.part_2(y) {
            println!("part2: {}", b.x * 4000000 + b.y);
            break;
        }
    }
}

fn man_dist(x1: isize, y1: isize, x2: isize, y2: isize) -> usize {
    x2.abs_diff(x1) + y2.abs_diff(y1)
}

#[derive(Hash, Eq, PartialEq)]
struct Beacon {
    x: isize,
    y: isize,
}

struct Sensor {
    x: isize,
    y: isize,
    nearest_beacon: Beacon,
}

impl Sensor {
    fn beacon_dist(&self) -> usize {
        man_dist(self.x, self.y, self.nearest_beacon.x, self.nearest_beacon.y)
    }

    fn in_range(&self, y: isize) -> bool {
        self.y.abs_diff(y) <= self.beacon_dist()
    }

    fn range(&self, y: isize) -> (isize, isize) {
        let x_dist: isize = (self.beacon_dist() - self.y.abs_diff(y)) as isize;
        let left = self.x - x_dist;
        let right = self.x + x_dist;
        (left, right)
    }
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        //dbg!(parts[2][2..parts[2].len() - 1].to_owned());
        let x: isize = parts[2][2..parts[2].len() - 1].parse().unwrap();
        let y: isize = parts[3][2..parts[3].len() - 1].parse().unwrap();
        let xb: isize = parts[8][2..parts[8].len() - 1].parse().unwrap();
        let yb: isize = parts[9][2..].parse().unwrap();
        //dbg!(xb, yb);
        Ok(Self {
            x,
            y,
            nearest_beacon: Beacon { x: xb, y: yb },
        })
    }
}

#[test]
fn test_parse_sensor() {
    assert_eq!(7, man_dist(2, 18, -2, 15));
    let sensor: Sensor = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
        .parse()
        .unwrap();
    assert_eq!(2, sensor.x);
    assert_eq!(18, sensor.y);
    assert_eq!(7, sensor.beacon_dist());
}

struct Map {
    sensors: Vec<Sensor>,
}

impl Map {
    /*
     * This is only working because this line is not the one of part 2
     * As at the end, the range is continuous.
     */
    fn part1(&self, y: isize) -> (isize, isize) {
        self.sensors
            .iter()
            .filter(|s| s.in_range(y))
            .map(|s| s.range(y))
            .reduce(|a, b| (isize::min(a.0, b.0), isize::max(a.1, b.1)))
            .unwrap()
    }

    fn beacon_amount(&self, y: isize) -> usize {
        self.sensors
            .iter()
            .map(|s| &s.nearest_beacon)
            .filter(|b| b.y == y)
            .collect::<HashSet<_>>()
            .iter()
            .count()
    }

    fn part_2(&self, y: isize) -> Option<Beacon> {
        let mut ranges = self
            .sensors
            .iter()
            .filter(|s| s.in_range(y))
            .map(|s| s.range(y))
            .collect::<Vec<_>>();
        // Sort by left boundary
        ranges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut right = ranges.first().unwrap().1;
        for (l, r) in &ranges[1..] {
            if l > &right {
                return Some(Beacon { x: *l - 1, y });
            }
            right = isize::max(right, *r);
        }
        None
    }
}
