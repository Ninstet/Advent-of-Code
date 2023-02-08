
use std::fs::File;
use std::io::{BufReader, BufRead};

// #[derive(Debug)]
// enum State {
//     Command,
//     Response,
// }

// struct File {
//     name: String,
//     size: i32
// }

// enum FileType {
//     file_size: i32,
//     Directory(Directory),
// }

// impl File {
//     fn new(name: String, size: i32) {
//         File { name, size }
//     }
// }

#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<i32>,
    directories: Vec<Directory>
}

impl Directory {
    fn new(name: String) -> Self {
        Directory { name, files: Vec::new(), directories: Vec::new() }
    }

    fn add_file(&mut self, file: i32) -> i32 {
        self.files.push(file);
        file
    }

    fn add_directory(&mut self, directory: Directory) -> &mut Directory {
        self.directories.push(directory);
        self.directories.last_mut().unwrap()
    }

    fn sum_files(&self) -> i32 {
        self.files.iter().sum()
    }

    fn sum_directories(&mut self) -> i32 {
        let mut sum: i32 = 0;
        for directory in &self.directories {
            println!("{}: {}", directory.name, directory.sum_files());
            if directory.sum_files() <= 100000 && directory.name != "/" {
                sum += directory.sum_files();
            }
        }
        sum
    }
}

fn process_buffer(pwd: &mut Directory, buffer: &mut Vec<String>) {
    println!("to process in {}:  {:?}", pwd.name, buffer);

    // let mut directory: Directory = Directory::new(pwd.to_string());

    for line in &mut *buffer {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words[0] != "dir" {
            pwd.add_file(words[0].parse().unwrap());
        }
    }

    // Reset the buffer
    *buffer = Vec::new();
}

pub fn main() {
    // Open the file "input.txt"
    let file = File::open("test.txt").unwrap();

    // Create a buffered reader for the file
    let reader = BufReader::new(file);

    // let mut pwd: String = "/".to_string();

    let mut filesystem: Directory = Directory::new("/".to_string());
    let mut pwd: &mut Directory = &mut filesystem;
    let mut buffer: Vec<String> = Vec::new();

    // Iterate over the lines in the file
    for line in reader.lines() {
        // Get the line as a string
        let line = line.unwrap();

        if line == "$ cd /" {
            continue;
        }

        let words: Vec<&str> = line.split_whitespace().collect();

        match words[0] {
            "$" => match words[1] {
                "cd" => {
                    if buffer.len() > 0 {
                        process_buffer(&mut pwd, &mut buffer);
                    }
                    pwd = pwd.add_directory(Directory::new(words[2].to_string()));
                },
                _ => ()
            },
            _ => buffer.push(line)
        }
    }

    process_buffer(&mut pwd, &mut buffer);
    println!("{}", filesystem.sum_directories());
    println!("{:#?}", filesystem);
}
