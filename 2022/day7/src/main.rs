use std::io::BufRead;

fn main(){
    // special one, create manually
    let root = Dir::root();
    let mut current = &root;
    std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().into())
        .for_each(|l: String| {
            let tokens: Vec<_> = l.split(" ").collect();
            match tokens[0]{
                "$" => {
                    match tokens[1]{
                        "cd" => current = match tokens[2]{
                            "/" => &root,
                            "." => current,
                            ".." => &mut current.get_parent(),
                            _ => &mut current.get_dir(tokens[2]).unwrap(),
                        },
                        "ls" => (), // do nothing
                        _ => panic!(),
                    };
                },
                // Assume all answers are from `ls`
                "dir" => current.add_dir(tokens[1]),
                _ => {
                    let size = tokens[0].parse().unwrap();
                    current.add_file(tokens[1], size);
                }
            }
        })

}

struct Dir{
    name: String,
    parent: Option<Box<Dir>>,
    files: Vec<File>,
    dirs: Vec<Dir>,
}

impl Dir {
    fn root() -> Dir{
        let mut res = Dir{ 
            name: "/".to_string(),
            parent: None,
            files: vec!(),
            dirs: vec!(),
        };
        res.parent = Some(Box::new(res));
        return res;
    }

    fn new(name: &str, parent: &Dir) -> Dir{
        Dir {
            name: name.to_string(),
            parent: Some(Box::new(*parent)),
            files: vec!(),
            dirs: vec!(),
        }
    }

    fn get_parent(&self) -> &Dir{
        &self.parent.unwrap()
    }

    fn get_dir(&self, name: &str) -> Option<&Dir>{
        self.dirs.iter().find(|d| d.name == name)
    }

    fn add_dir(&mut self, name: &str){
        self.dirs.push(Dir::new(name, self))
    }

    fn add_file(&mut self, name: &str, size: u32){
        self.files.push(File::new(name, size))
    }
}

struct File{
    name: String,
    size: u32,
}

impl File{
    fn new(name: &str, size: u32) -> File{
        File{
            name: name.to_string(),
            size
        }
    }
}