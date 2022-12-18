
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

fn main() {
    // Open the file "input.txt"
    let file = File::open("input.txt").unwrap();

    // Create a buffered reader for the file
    let reader = BufReader::new(file);

    // Iterate over the lines in the file
    for line in reader.lines() {
        // Get the line as a string
        let line = line.unwrap();

        let mut buffer: Vec<char> = Vec::new();
        let mut i = 4;

        for c in line.chars() {
            buffer.push(c);

            if buffer.len() > 4 {
                buffer.remove(0);
                i += 1;

                let set: HashSet<char> = buffer.iter().cloned().collect();
                
                if buffer.len() == set.len() {
                    break
                }
            }
        }

        println!("{}: {:?}", i, buffer)
    }
}
