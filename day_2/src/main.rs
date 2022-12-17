use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_score = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let words: Vec<&str> = line.split_whitespace().collect();

        // 1 for Rock, 2 for Paper, and 3 for Scissors
        // 0 if you lost, 3 if the round was a draw, and 6 if you won

        let score = match words[1] {
            "X" => { // Rock
                1 + match words[0] {
                    "A" => 3, // Rock
                    "B" => 0, // Paper
                    "C" => 6, // Scissors
                    _ => 0
                }
            },
            "Y" => { // Paper
                2 + match words[0] {
                    "A" => 6, // Rock
                    "B" => 3, // Paper
                    "C" => 0, // Scissors
                    _ => 0
                }
            },
            "Z" => { // Scissors
                3 + match words[0] {
                    "A" => 0, // Rock
                    "B" => 6, // Paper
                    "C" => 3, // Scissors
                    _ => 0
                }
            },
            _ => 0
        };

        total_score += score;
    }

    println!("Total score is {}", total_score);

    // Bonus
    bonus();
}

fn bonus() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_score = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let words: Vec<&str> = line.split_whitespace().collect();

        // 1 for Rock, 2 for Paper, and 3 for Scissors
        // 0 if you lost, 3 if the round was a draw, and 6 if you won

        let score = match words[1] {
            "X" => { // Lose
                0 + match words[0] {
                    "A" => 3, // Rock
                    "B" => 1, // Paper
                    "C" => 2, // Scissors
                    _ => 0
                }
            },
            "Y" => { // Draw
                3 + match words[0] {
                    "A" => 1, // Rock
                    "B" => 2, // Paper
                    "C" => 3, // Scissors
                    _ => 0
                }
            },
            "Z" => { // Win
                6 + match words[0] {
                    "A" => 2, // Rock
                    "B" => 3, // Paper
                    "C" => 1, // Scissors
                    _ => 0
                }
            },
            _ => 0
        };

        total_score += score;
    }

    println!("Total score is {}", total_score);
}