use std::str::FromStr;

fn main(){
    let mut map= Map{sensors:vec![]};


}

fn man_dist(x1: isize, y1: isize, x2: isize, y2:isize) -> usize{
    x2.abs_diff(x1) + y2.abs_diff(y1)
}

struct Sensor{
    x: isize,
    y: isize,
    nearest_beacon: usize,
}

impl FromStr for Sensor{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        dbg!(parts[2][2..parts[2].len() - 1].to_owned());
        let x: isize = parts[2][2..parts[2].len() - 1].parse().unwrap();
        let y: isize = parts[3][2..parts[3].len() - 1].parse().unwrap();
        let xb : isize = parts[8][2..parts[8].len() - 1].parse().unwrap();
        let yb : isize = parts[9][2..].parse().unwrap();
        dbg!(xb, yb);
        Ok(Self{
            x,
            y,
            nearest_beacon: man_dist(x, y, xb, yb)
        })  
    }
}

#[test]
fn test_parse_sensor(){
    assert_eq!(7, man_dist(2, 18, -2, 15));
    let sensor : Sensor = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15".parse().unwrap();
    assert_eq!(2, sensor.x);
    assert_eq!(18, sensor.y);
    assert_eq!(7, sensor.nearest_beacon);
}

struct Map{
    sensors: Vec<Sensor>,
}