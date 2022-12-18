/*
    This program does the following:
    It reads a file called "input.txt" line by line, and for each line, it calls the find_unique_chars function with the line and a start value of 4.
    The find_unique_chars function takes in a string and a start value, and it processes the string character by character. It maintains a buffer of characters, and it adds each character to the buffer as it processes it. If the buffer size is greater than the start value, it removes the first character in the buffer.
    For each character, the function also creates a HashSet of the characters in the buffer. If the size of the buffer is equal to the size of the HashSet, it means that all characters in the buffer are unique, so the function breaks out of the loop and prints the index and the buffer.
    After processing each line, the program calls the find_unique_chars function again with the same line but with a start value of 14. This prints the index and the buffer for the case where the start value is 14.
    The program then repeats these steps for each line in the file.
 */

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

// This function finds the first contiguous group of unique characters in the given string
// starting from the index specified by the start parameter.
fn find_unique_chars(string: &String, start: usize) {
    // Initialize an empty buffer for storing characters
    let mut buffer: Vec<char> = Vec::new();

    // Initialize an index counter
    let mut i = start;

    // Iterate over the characters in the string
    for c in string.chars() {
        // Add the current character to the buffer
        buffer.push(c);

        // If the buffer is larger than the start value, remove the first character
        if buffer.len() > start {
            buffer.remove(0);

            // Increment the index counter
            i += 1;

            // Create a HashSet of the characters in the buffer
            let set: HashSet<char> = buffer.iter().cloned().collect();

            // If the size of the buffer is equal to the size of the HashSet, it means all
            // characters in the buffer are unique, so we can break out of the loop.
            if buffer.len() == set.len() {
                break
            }
        }
    }

    // Print the index and the buffer
    println!("{}: {:?}", i, buffer)
}

fn main() {
    // Open the file "input.txt"
    let file = File::open("input.txt").unwrap();

    // Create a buffered reader for the file
    let reader = BufReader::new(file);

    // Iterate over the lines in the file
    for line in reader.lines() {
        // Get the line as a string
        let line = line.unwrap();

        // Find the first contiguous group of unique characters starting from index 4
        find_unique_chars(&line, 4);
        
        // Bonus: Find the first contiguous group of unique characters starting from index 14
        find_unique_chars(&line, 14);
    }
}
