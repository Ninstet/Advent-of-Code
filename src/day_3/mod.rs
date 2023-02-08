use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

pub fn main() -> std::io::Result<()> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut priorities_sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let half_length = line.chars().count() / 2;
        let first_half: String = line.chars().take(half_length).collect();
        let second_half: String = line.chars().skip(half_length).collect();

        let set1: HashSet<char> = first_half.chars().collect();
        let set2: HashSet<char> = second_half.chars().collect();

        let intersection: HashSet<char> = set1.intersection(&set2).cloned().collect();
        let first_char: char = *intersection.iter().next().unwrap();

        priorities_sum += char_to_int(first_char);
    }

    println!("Total priorities sum is {}", priorities_sum);

    // Bonus
    bonus()?;

    Ok(())
}

fn char_to_int(c: char) -> i32 {
    match c {
        'a'..='z' => (c as i32) - ('a' as i32) + 1,
        'A'..='Z' => (c as i32) - ('A' as i32) + 27,
        _ => 0,
    }
}

fn bonus() -> std::io::Result<()> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut priorities_sum = 0;

    let mut lines = reader.lines();
    while let Some(mut line1) = lines.next() {
        line1 = Ok(line1.unwrap());
        let line2 = lines.next().unwrap();
        let line3 = lines.next().unwrap();

        let set1: HashSet<char> = line1?.chars().collect();
        let set2: HashSet<char> = line2?.chars().collect();
        let set3: HashSet<char> = line3?.chars().collect();
        
        let intersection: HashSet<char> = set1.intersection(&set2).cloned().collect::<HashSet<char>>()
                                              .intersection(&set3).cloned().collect();

        let first_char: char = *intersection.iter().next().unwrap();

        priorities_sum += char_to_int(first_char);
    }

    println!("Total priorities sum is {}", priorities_sum);

    Ok(())
}
