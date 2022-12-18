use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    // Open the file "input.txt"
    let file = File::open("input.txt").unwrap();
    // Create a buffered reader for the file
    let reader = BufReader::new(file);

    // Initialize a counter for the number of lines that contain overlapping pairs of integers
    let mut no_contains = 0;

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

        // Print the Vec of integers
        println!("{:#?}", numbers);

        // Check if the two pairs of integers are overlapping
        if (numbers[0] <= numbers[2] && numbers[1] >= numbers[3]) || (numbers[0] >= numbers[2] && numbers[1] <= numbers[3]) {
            // If they are, increment the counter and print the line
            no_contains += 1;
            println!("{}", line);
        }
    }

    // Print the final count of lines with overlapping pairs
    println!("{}", no_contains);
}
