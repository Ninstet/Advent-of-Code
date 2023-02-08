/*
    This Rust code reads a file called input.txt and processes the contents according to the following logic:

    The file is read line by line using a buffered reader.
    The code is in a state machine with two states: Arrangement and Instruction.
    When the state is Arrangement, the code processes the lines of the file as follows:

    It creates a Vec of Vecs called arrangement to represent the arrangement of crates in the warehouse.
    It parses each line of the file to extract the names of the crates and adds them to the arrangement Vec.
    If the line is empty, it reverses the order of the elements in each Vec in the arrangement Vec, and changes the state to Instruction.

    When the state is Instruction, the code processes the lines of the file as follows:

    It splits each line into words using split_whitespace, and parses the numbers from the words using parse.
    It moves the top crate from the stack specified by the first number to the stack specified by the second number, and prints a message with the details of the move.

    Finally, the code creates a new String by collecting the last element of each Vec in the arrangement Vec, and prints the String.
 */

use std::fs::File;
use std::io::{BufReader, BufRead};

// Define an enum for the state of the state machine
enum State {
    Arrangement,  // State for processing the arrangement of crates
    Instruction,  // State for processing the instructions
}

pub fn main() {
    // Open the file "input.txt"
    let file = File::open("input.txt").unwrap();

    // Create a buffered reader for the file
    let reader = BufReader::new(file);

    // Set the initial state to "Arrangement"
    let mut state = State::Arrangement;

    // Initialize an empty Vec of Vecs to store the arrangement of crates
    let mut arrangement: Vec<Vec<char>> = Vec::new();
    let mut bonus_arrangement: Vec<Vec<char>> = Vec::new();

    // Iterate over the lines in the file
    for line in reader.lines() {
        // Get the line as a string
        let line = line.unwrap();

        // Match on the current state
        match state {
            // If the state is "Arrangement", process the line as an arrangement of crates
            State::Arrangement => {
                // Calculate the number of stacks in the arrangement
                let no_stacks = (line.len() + 1) / 4;

                // Convert the line to a Vec of chars
                let chars: Vec<char> = line.chars().collect();

                // Iterate over the stacks in the arrangement
                for i in 1..(no_stacks + 1) {
                    // Initialize an empty Vec for the stack if it doesn't exist yet
                    if arrangement.len() < no_stacks {
                        arrangement.push(Vec::new());
                        bonus_arrangement.push(Vec::new());
                    }

                    // Get the crate name from the line
                    let crate_name: char = chars[(4 * i) - 3];

                    // Add the crate to the stack if it's not a space or a number
                    if crate_name != ' ' && !crate_name.is_numeric() {
                        arrangement[i - 1].push(crate_name);
                        bonus_arrangement[i - 1].push(crate_name);
                    }
                }

                // If the line is empty, reverse the order of the crates in each stack and switch to the "Instruction" state
                if line == "" {
                    for vec in arrangement.iter_mut() {
                        vec.reverse();
                    }
                    for vec in bonus_arrangement.iter_mut() {
                        vec.reverse();
                    }
                    state = State::Instruction;
                }
            },
            // If the state is "Instruction", process the line as an instruction
            State::Instruction => {
                // Split the line into words and convert them to a Vec of &str
                let words: Vec<&str> = line.split_whitespace().collect();

                let mut crates: Vec<char> = Vec::new();

                // Get the stack indices for the move
                let from_stack: usize = words[3].parse::<usize>().unwrap() - 1;
                let to_stack: usize = words[5].parse::<usize>().unwrap() - 1;

                // Perform the specified number of moves
                for _ in 0..numbers[0] as usize {
                    let element: char = arrangement[from_stack].pop().unwrap();
                    let bonus_element: char = bonus_arrangement[from_stack].pop().unwrap();

                    crates.push(bonus_element); // Char implements the 'Copy' trait so is copied to the Vec
                    arrangement[to_stack].push(element);

                    println!("Moved {} from {} to {}", element, from_stack + 1, to_stack + 1);
                }

                // Reverse crates to insert them in the original order
                crates.reverse();
                for element in crates {
                    bonus_arrangement[to_stack].push(element);

                    println!("[BONUS] Moved {} from {} to {}", element, from_stack + 1, to_stack + 1);
                }
            }
        }
    }

    let string: String = arrangement.iter().map(|v| v[v.len() - 1].to_string()).collect();
    let bonus_string: String = bonus_arrangement.iter().map(|v| v[v.len() - 1].to_string()).collect();

    println!("Main: {}", string);
    println!("Bonus: {}", bonus_string);
}
