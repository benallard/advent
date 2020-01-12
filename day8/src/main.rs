struct Layer {
    data: Vec<Vec<u8>>,
}

impl Layer {
    pub fn new(content: &str, width: usize, height: usize) -> Layer {
        assert_eq!(width * height, content.len());
        let mut data = Vec::new();
        let mut line = Vec::new();
        for (i, c) in content.char_indices() {
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

    pub fn count(&self, v: u8) -> u16{
        let mut res = 0;
        for line in &self.data{
            for char in line{
                if *char == v{
                    res += 1;
                }
            }
        }
        return res;
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
        for (i, layer) in self.layers.iter().enumerate(){
            let count = layer.count(0);
            println!("{}: {}", i, count);
            if count < min{
                min = count;
                idx = i;
            }
        }
        println!("Layer {} it is, {}, {}", idx, self.layers[idx].count(1), self.layers[idx].count(2));
        return self.layers[idx].count(1) * self.layers[idx].count(2);
    }
}

fn main() {
    let content = include_str!("../../day8.txt").trim();
    let img = Image::new(content, 25, 6);

    println!("Part 1: {}", img.part1());
}

#[test]
fn test() {
    let img = Image::new("123456789012", 3, 2);
}
