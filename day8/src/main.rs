#[derive(Clone)]
struct Layer {
    data: Vec<Vec<u8>>,
}

impl Layer {
    pub fn new(content: &str, width: usize, height: usize) -> Layer {
        assert_eq!(width * height, content.len());
        let mut data = Vec::new();
        let mut line = Vec::new();
        for c in content.chars() {
            line.push(c.to_digit(10).unwrap() as u8);
            if line.len() == width {
                assert_eq!(width, line.len());
                data.push(line);
                line = Vec::new();
            }
        }
        assert_eq!(height, data.len());
        Layer { data: data }
    }
    pub fn new_empty(width: usize, height: usize) -> Layer {
        let mut data = Vec::new();
        for _ in 0..height {
            let mut line = Vec::new();
            for _ in 0..width {
                line.push(2);
            }
            data.push(line);
        }
        assert_eq!(height, data.len());
        Layer { data: data }
    }

    pub fn count(&self, v: u8) -> u16 {
        let mut res = 0;
        for line in &self.data {
            for char in line {
                if *char == v {
                    res += 1;
                }
            }
        }
        return res;
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        return self.data[y][x];
    }

    pub fn blend(&mut self, other: &Layer) {
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                let char = self.data[y][x];
                if char != 2 {
                    continue;
                }
                if other.get(x, y) != 2 {
                    self.data[y][x] = other.get(x, y);
                }
            }
        }
    }

    pub fn print(&self) {
        for line in &self.data {
            let mut s_line = String::new();
            for char in line {
                match char {
                    0 => s_line.push_str("  "),
                    1 => s_line.push_str("XX"),
                    _ => panic!(),
                }
            }
            println!("{}", &s_line);
        }
    }
}

struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

impl Image {
    pub fn new(content: &str, width: usize, height: usize) -> Image {
        let mut layers = Vec::new();
        let mut idx = 0;
        loop {
            if content.len() <= idx {
                break;
            }
            let layer = Layer::new(&content[idx..idx + (width * height)], width, height);
            layers.push(layer);
            idx += width * height;
        }
        return Image {
            width: width,
            height: height,
            layers: layers,
        };
    }

    pub fn part1(&self) -> u16 {
        let mut idx = 0;
        let mut min: u16 = (self.width * self.height + 1) as u16;
        for (i, layer) in self.layers.iter().enumerate() {
            let count = layer.count(0);
            if count < min {
                min = count;
                idx = i;
            }
        }
        println!(
            "Layer {} it is, {}, {}",
            idx,
            self.layers[idx].count(1),
            self.layers[idx].count(2)
        );
        return self.layers[idx].count(1) * self.layers[idx].count(2);
    }

    pub fn part2(&self) -> Layer {
        let mut res = Layer::new_empty(self.width, self.height);
        for layer in &self.layers {
            res.blend(&layer);
        }
        return res;
    }
}

fn main() {
    let content = include_str!("../../day8.txt").trim();
    let img = Image::new(content, 25, 6);

    println!("Part 1: {}", img.part1());

    img.part2().print();
}

#[test]
fn test() {
    let img = Image::new("0222112222120000", 2, 2);
    let res = img.part2();
    assert_eq!(0, res.get(0, 0));
    assert_eq!(1, res.get(1, 0));
    assert_eq!(1, res.get(0, 1));
    assert_eq!(0, res.get(1, 1));
}
