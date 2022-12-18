use std::fs::File;
use std::io::{BufReader, BufRead};

enum Step {
    Arrangement,
    Instruction,
}

fn main() {
    // Open the file "input.txt"
    let file = File::open("test.txt").unwrap();

    // Create a buffered reader for the file
    let reader = BufReader::new(file);

    let mut step = Step::Arrangement;

    let mut arrangement: Vec<Vec<char>> = Vec::new();

    // Iterate over the lines in the file
    for line in reader.lines() {
        // Get the line as a string
        let line = line.unwrap();

        if line == "" {
            step = Step::Instruction;
            continue;
        }

        match step {
            Step::Arrangement => {
                println!("arrangement");
                
                let no_stacks = (line.len() + 1) / 4;
                let chars: Vec<char> = line.chars().collect();
                

                for i in 1..(no_stacks + 1) {
                    arrangement.push(Vec::new());
                    let crate_name: char = chars[(4 * i) - 3];
                    println!("{}: {}", i, crate_name);
                    arrangement[i - 1].push(crate_name);
                }
            },
            Step::Instruction => {
                println!("instruction");
            }

        }
        println!("{:#?}", arrangement);
    }
}
