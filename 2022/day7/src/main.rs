use std::io::BufRead;

fn main(){
    let root = Dir::new("/", None);
    let mut current = &root;
    std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap().trim().into())
        .for_each(|l: String| {
            match l.chars().nth(0).unwrap(){
                '$' => {
                    let command: Vec<_> = l.split(" ").collect();
                    match command[1]{
                        "cd" => current = current.get_dir(command[2]).unwrap(),
                        //"ls" =>,
                        _ => panic!(),
                    }
                }
                _ => {
                    
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
    fn new(name: &str, parent: Option<Box<Dir>>) -> Dir{
        Dir {
            name: name.to_string(),
            parent: parent,
            files: vec!(),
            dirs: vec!(),
        }
    }

    fn get_dir(&self, name: &str) -> Option<&Dir>{
        self.dirs.iter().find(|d| d.name == name)
    }
}

struct File{
    name: String,
    size: i32,
    parent: Dir,
}
