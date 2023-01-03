use std::collections::HashMap;
use std::fs;

fn read_file(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(filename).expect("Can't read file");
    let rows: Vec<String> = file.split("\n").map(|x| x.to_owned()).collect();
    rows
}

#[derive(Default, Debug)]
struct File<'a> {
    name: &'a str,
    size: usize,
}

#[derive(Default, Debug)]
struct Directory<'a> {
    name: &'a str,
    parent: Option<&'a str>,
    files: Vec<File<'a>>,
    children: Vec<&'a str>,
}
struct FileSystem<'a> {
    directories: HashMap<&'a str, Directory<'a>>,
}

impl<'a> Directory<'a> {
    fn new(name: &'a str, parent: Option<&'a str>) -> Self {
        Directory {
            name: name,
            parent: parent,
            ..Default::default()
        }
    }

    fn add_file(mut self, name: &'a str, size: usize) -> () {
        self.files.push(File::new(name, size));
    }

    fn add_child(mut self, name: &'a str) -> () {
        self.children.push(name);
    }
}

impl<'a> File<'a> {
    fn new(name: &'a str, size: usize) -> Self {
        File {
            name: name,
            size: size,
        }
    }
}

impl<'a> FileSystem<'a> {
    fn new() -> Self {
        FileSystem {
            directories: HashMap::new(),
        }
    }

    fn enter_directory(
        &mut self,
        dir_name: &'a str,
        current_directory: Option<&'a str>,
    ) -> &mut Directory<'a> {
        self.directories
            .entry(dir_name)
            .or_insert(Directory::new(dir_name, current_directory))
    }

    fn exit_directory(&self, curr_dir: &'a str) -> Result<&Directory, &str> {
        let parent_name = self.directories.get(curr_dir).unwrap().parent;
        match parent_name {
            Some(parent_name) => Ok(self.directories.get(parent_name).unwrap()),
            None => Err("Parent not found"),
        }
    }
}

fn run_tree(files: Vec<String>) -> () {
    let filesystem = FileSystem::new();

    for item in files.iter() {
        let command = item;

        match command {}
    }
}

fn main() {
    let file = read_file("./d7_input.txt");
    println!("{:?}", file);
}
