use std::io::BufRead;

fn main() {
    let field: Vec<Vec<u8>> = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().into())
        .map(|l: String| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect();

    let height = field.len();
    let width = field[0].len();

    // The border is visible
    let mut sum = 2 * width + (2 * (height - 2));
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if visible(&field, x, y) {
                //dbg!((x, y));
                sum += 1;
            }
        }
    }
    println!("visible: {}", sum);
    let mut max = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let score = scenic_score(&field, x, y);
            if score > max {
                //dbg!((x, y), score);
                max = score;
            }
        }
    }
    println!("scenic core: {}", max);
}

fn visible(field: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let self_height = field[y][x];
    // top
    let mut res = true;
    for yy in 0..y {
        if field[yy][x] >= self_height {
            res = false;
            //dbg!("top");
            break;
        }
    }
    if res {
        return res;
    }
    // left
    res = true;
    for xx in 0..x {
        if field[y][xx] >= self_height {
            res = false;
            //dbg!("left");
            break;
        }
    }
    if res {
        return res;
    }
    // bottom
    let mut res = true;
    for yy in (y + 1)..field.len() {
        if field[yy][x] >= self_height {
            res = false;
            //dbg!("bottom");
            break;
        }
    }
    if res {
        return res;
    }
    // right
    res = true;
    for xx in (x + 1)..field[y].len() {
        if field[y][xx] >= self_height {
            res = false;
            //dbg!("right");
            break;
        }
    }
    return res;
}

fn scenic_score(field: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let self_height = field[y][x];
    // top
    let mut res = 1;
    {
        let mut lres = 0;
        for yy in (0..y).rev() {
            lres += 1;
            if field[yy][x] >= self_height {
                break;
            }
        }
        //dbg!("top", lres);
        res *= lres;
    }
    // left
    {
        let mut lres = 0;
        for xx in (0..x).rev() {
            lres += 1;
            if field[y][xx] >= self_height {
                break;
            }
        }
        //dbg!("left", lres);
        res *= lres;
    }
    // bottom
    {
        let mut lres = 0;
        for yy in (y + 1)..field.len() {
            lres += 1;
            if field[yy][x] >= self_height {
                break;
            }
        }
        //dbg!("bottom", lres);
        res *= lres;
    }
    // right
    {
        let mut lres = 0;
        for xx in (x + 1)..field[y].len() {
            lres += 1;
            if field[y][xx] >= self_height {
                break;
            }
        }
        //dbg!("right", lres);
        res *= lres;
    }
    return res;
}
