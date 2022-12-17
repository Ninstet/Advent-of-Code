use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let words: Vec<&str> = line.split_whitespace().collect();

        println!("{:#?}", words);

        
    }
}
