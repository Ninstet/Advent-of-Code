/*
    This code reads from a file named "input.txt" and processes each line in the file. It uses the std::fs::File type to open the file and create a buffered reader for it using the std::io::BufReader type.
    The code then iterates over the lines in the file and splits each line on the comma character to get a vector of chunks. It then flattens the vector of chunks into a single iterator and splits each chunk on the dash character to get a vector of integers.
    The code then checks if the two pairs of integers are overlapping by checking if either of the following conditions are true:
     - The first pair of integers is contained within the second pair
     - The second pair of integers is contained within the first pair
    If either of these conditions is true, the code increments the no_contains counter. It also has a separate counter, no_contains_bonus, that is incremented if the two pairs of integers intersect (i.e. they have at least one integer in common).
    Finally, the code prints the final count of lines with overlapping pairs.
 */

use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    // Open the file "input.txt"
    let file = File::open("input.txt").unwrap();

    // Create a buffered reader for the file
    let reader = BufReader::new(file);

    // Initialize a counter for the number of lines that contain overlapping pairs of integers
    let mut no_contains = 0;

    // Initialize a counter for the number of lines that contain intersecting pairs of integers
    let mut no_contains_bonus = 0;

    // Iterate over the lines in the file
    for line in reader.lines() {
        // Get the line as a string
        let line = line.unwrap();

        // Split the line on the comma character to get a Vec of chunks
        let chunks: Vec<&str> = line.split(',').collect();

        // Flatten the Vec of chunks into a single iterator and split each chunk on the dash character
        let numbers: Vec<i32> = chunks.iter()
            .flat_map(|chunk| chunk.split('-'))
            // Parse the substrings as integers
            .map(|s| s.parse().unwrap())
            .collect();

        // Check if the two pairs of integers are overlapping
        if (numbers[0] <= numbers[2] && numbers[1] >= numbers[3]) || (numbers[0] >= numbers[2] && numbers[1] <= numbers[3]) {
            // If they are, increment the counter
            no_contains += 1;
        }

        // Check if the two pairs of integers intersect (have at least one integer in common)
        if numbers[1] >= numbers[2] && numbers[0] <= numbers[3] {
            // If they do, increment the counter
            no_contains_bonus += 1;
        }
    }

    // Print the final count of lines with overlapping pairs
    println!("Main: {}", no_contains);

    // Print the final count of lines with intersecting pairs
    println!("Bonus: {}", no_contains_bonus);
}
