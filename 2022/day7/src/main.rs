use std::cell::RefCell;
use std::io::BufRead;

fn main() {
    // special one, create manually
    let root = Dir::root();
    let mut current = &root;
    std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().into())
        .for_each(|l: String| {
            let tokens: Vec<_> = l.split(" ").collect();
            match tokens[0] {
                "$" => {
                    match tokens[1] {
                        "cd" => {
                            current = match tokens[2] {
                                "/" => &root,
                                "." => current,
                                ".." => &current.get_parent(),
                                _ => &current.get_dir(tokens[2]).unwrap(),
                            }
                        }
                        "ls" => (), // do nothing
                        _ => panic!(),
                    };
                }
                // Assume all answers are from `ls`
                "dir" => (&mut current).add_dir(tokens[1]),
                _ => {
                    let size = tokens[0].parse().unwrap();
                    current.add_file(tokens[1], size);
                }
            }
        });
    println!("size: {}", root.get_size());
}

struct Dir {
    name: String,
    parent: Option<RefCell<Box<Dir>>>,
    files: Vec<File>,
    dirs: Vec<Dir>,
}

impl Dir {
    fn root() -> Self {
        let res = Self {
            name: "/".to_string(),
            parent: None,
            files: vec![],
            dirs: vec![],
        };
        let parent = Some(RefCell::new(Box::new(res)));
        res.parent = parent;
        return res;
    }

    fn new(name: &str, parent: &mut Self) -> Self {
        Self {
            name: name.to_string(),
            parent: Some(RefCell::new(Box::new(*parent))),
            files: vec![],
            dirs: vec![],
        }
    }

    fn get_parent(&mut self) -> &Self {
        &self.parent.as_mut().unwrap().borrow()
    }

    fn get_dir(&self, name: &str) -> Option<&Self> {
        self.dirs.iter().find(|d| d.name == name)
    }

    fn add_dir(&mut self, name: &str) {
        let sub_dir = Dir::new(name, Box::new(self));
        self.dirs.push(sub_dir)
    }

    fn add_file(&mut self, name: &str, size: u32) {
        self.files.push(File::new(name, size))
    }
}

struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: &str, size: u32) -> File {
        File {
            name: name.to_string(),
            size,
        }
    }
}

trait Sizeable {
    fn get_size(&self) -> u64;
}

impl Sizeable for Dir {
    fn get_size(&self) -> u64 {
        self.dirs.iter().map(|d| d.get_size()).sum::<u64>()
            + self.files.iter().map(|f| f.get_size()).sum::<u64>()
    }
}

impl Sizeable for File {
    fn get_size(&self) -> u64 {
        self.size.into()
    }
}
