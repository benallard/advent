// Euclid's gcd
fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

const I_TH: i32 = 200;

#[derive(Clone, PartialEq, Eq)]
enum MapSpot {
    Empty,
    Hidden,
    Asteroid,
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    data: Vec<Vec<MapSpot>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut data = Vec::with_capacity(height);
        for _ in 0..height {
            data.push(vec![MapSpot::Empty; width]);
        }
        Map {
            width,
            height,
            data,
        }
    }

    pub fn add_asteroid(&mut self, x: usize, y: usize) {
        self.data[y][x] = MapSpot::Asteroid;
    }

    pub fn count_asteroids(&self) -> u32 {
        let mut res = 0;
        for line in &self.data {
            for spot in line {
                if *spot == MapSpot::Asteroid {
                    res += 1;
                }
            }
        }
        return res;
    }

    pub fn is_asteroid(&self, x: usize, y: usize) -> bool {
        self.data[y][x] == MapSpot::Asteroid
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn print(&self){
        for line in &self.data{
            let mut s = String::new();
            for spot in line{
                match spot {
                    MapSpot::Asteroid => s.push_str( "# "),
                    MapSpot::Hidden => s.push_str("X "),
                    MapSpot::Empty => s.push_str("  "),
                }
            }
            println!("{}", s);
        }
    }

    pub fn count_reachable(&mut self, start_x: usize, start_y: usize) -> u32 {
        // go over the asteroids, and mark the hidden spots
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.is_asteroid(x, y) {
                    self.mark_hidden(start_x, start_y, x, y);
                }
            }
        }
        return self.count_asteroids();
    }

    pub fn cleanup(&mut self, start_x:usize, start_y: usize) -> usize {
        let lastCoord: Coord;
        for _ in 0..I_TH+1 {
        }
        return lastCoord.0 * 100 + lastCoord.1;
    }

    fn mark_hidden(&mut self, start_x: usize, start_y: usize, x: usize, y: usize) {
        println!("Analysing {}, {}", x, y);
        if x == start_x && y == start_y {
            self.data[y][x] = MapSpot::Hidden;
            return;
        }
        let mut delta_x : i32 = x as i32 - start_x as i32;
        let mut delta_y : i32  = y as i32 - start_y as i32;
        println!("dx, dy: {}, {}", delta_x, delta_y);
        let gcd = gcd(delta_x.abs(), delta_y.abs());
        delta_x = delta_x / gcd;
        delta_y = delta_y / gcd;
        println!("new dx, dy: {}, {}", delta_x, delta_y);
        let mut i = 1;
        loop {
            let dest_x: i32 = x as i32 + (i * delta_x);
            if (dest_x as usize >= self.width) || (dest_x < 0) {
                println!("X out: {}", dest_x);
                break;
            }
            let dest_y: i32 = y as i32 + (i * delta_y);
            if (dest_y as usize >= self.height) || (dest_y < 0) {
                println!("Y out: {}", dest_y);
                break;
            }
            println!("Marking {}, {} as hidden", dest_x, dest_y);
            self.data[dest_y as usize][dest_x as usize] = MapSpot::Hidden;
            i += 1;
        }
    }
}

fn parse_map(content: &str) -> Map {
    let lines: Vec<&str> = content.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let mut map = Map::new(width, height);
    for (y, line) in lines.iter().enumerate() {
        assert_eq!(width, line.len());
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.add_asteroid(x, y);
            }
        }
    }
    return map;
}

fn count_reachable(map: &Map, start_x: usize, start_y: usize) -> u32 {
    println!("Trying for {}, {}", start_x, start_y);
    let mut map = map.clone();
    let res = map.count_reachable(start_x, start_y);
    map.print();
    println!("=> {}", res);
    res
}

type Coord = (usize, usize);

fn part1(map: &Map) -> u32 {
    println!("Map is: ");
    map.print();
    let mut res: Coord = (999, 999);
    let mut max = 0;
    for y in 0..map.height() {
        for x in 0..map.width() {
            if !map.is_asteroid(x, y) {
                continue;
            }
            let count = count_reachable(&map, x, y);
            if count <= max {
                continue;
            }
            max = count;
            res = (x, y);
        }
    }
    println!("Best spot: {:?}", res);
    return max;
}

fn part2(map: &Map) -> u32{
0
}

fn main() {
    let mut content  = String::new();
    std::io::BufReader::new(std::io::stdin()).read_to_string(&mut content).unwrap();
    let map = parse_map(content);
    println!("Max: {}", part1(&map));
    println!("200th: {}", part2(&map));
}

#[test]
fn test1() {
    let map = parse_map(
        ".#..#
.....
#####
....#
...##",
    );
    assert_eq!(8, part1(map));
}


#[test]
fn test2() {
    let map = parse_map(
        "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
    );
    assert_eq!(33, part1(map));
}
#[test]
fn test3() {
    let map = parse_map(
        "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.",
    );
    assert_eq!(35, part1(map));
}
#[test]
fn test4() {
    let map = parse_map(
        ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
    );
    assert_eq!(41, part1(map));
}
#[test]
fn test5() {
    let map = parse_map(
        ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
    );
    assert_eq!(210, part1(map));
}
