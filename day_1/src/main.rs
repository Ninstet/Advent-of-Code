use std::fs::File;
use std::io::{BufReader, BufRead};

fn bonus(mut calories: Vec<i32>) -> i32 {
    calories.sort();

    let last_three: Vec<i32> = calories.iter().skip(calories.len() - 3).cloned().collect();
    let sum: i32 = last_three.iter().sum();

    sum
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut current_elf = 0;
    let mut calories = vec![0, 1];

    // Read the input
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            // Move on to the next Elf
            current_elf += 1;
            calories.push(0);
        } else {
            // Add the number of Calories to the current Elf's total
            calories[current_elf] += line.parse::<i32>().unwrap();
        }
    }

    // Find the Elf with the highest total number of Calories
    let mut max_calories = 0;
    let mut max_elf = 0;
    for (i, c) in calories.iter().enumerate() {
        if *c > max_calories {
            max_calories = *c;
            max_elf = i;
        }
    }

    // Print the result
    println!("Elf {} is carrying the most Calories: {}", max_elf + 1, max_calories);

    // Bonus
    println!("{}", bonus(calories));
}
