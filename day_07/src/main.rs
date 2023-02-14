use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

fn read_file(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(filename).expect("Can't read file");
    let rows: Vec<String> = file.split("\n").map(|x| x.to_owned()).collect();
    rows
}

#[derive(Default)]
struct Filesystem {
    directories: HashMap<String, Rc<RefCell<Directory>>>,
}

impl Filesystem {
    fn new() -> Self {
        Filesystem::default()
    }

    fn add_child(&mut self, child: Rc<RefCell<Directory>>) {
        self.directories
            .entry(child.borrow().name.to_string())
            .or_insert(Rc::clone(&child));
    }
}

#[derive(Default)]
struct Directory {
    name: String,
    parent: Rc<RefCell<Self>>,
    files: HashMap<String, File>,
    children: HashMap<String, Rc<RefCell<Self>>>,
}

struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: String, size: u32) -> Self {
        File {
            name: name,
            size: size,
        }
    }
}

impl Directory {
    fn new(name: String, parent: Rc<RefCell<Self>>) -> Self {
        Directory {
            name: name,
            parent: parent,
            ..Default::default()
        }
    }

    fn add_child_dir(&mut self, child: Rc<RefCell<Directory>>) {
        self.children
            .entry(child.borrow().name.to_string())
            .or_insert(Rc::new(RefCell::new(Directory::new(
                self.name.to_string(),
                Rc::clone(&self.parent),
            ))));
    }

    fn add_file(&mut self, name: &str, size: u32) {
        self.files
            .entry(name.to_string())
            .or_insert(File::new(name.to_string(), size));
    }

    fn descend(self, name: String) -> Rc<RefCell<Directory>> {
        Rc::clone(&self.children.get(&name).unwrap())
    }

    fn ascend(self) -> Rc<RefCell<Self>> {
        self.parent
    }
}

fn main() {
    let file = read_file("./d7_input.txt");
    println!("{:?}", file);
}
