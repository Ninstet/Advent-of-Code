use std::collections::HashMap;
use std::path::PathBuf;

pub fn part_1() -> u32 {
    let input = std::fs::read_to_string("test.txt").unwrap();
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in input.lines() {
        // Ignore these commands
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        println!("{:?}", affected);
        println!("{:?}", sizes);

        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }

    sizes
        .into_values()
        .filter(|size| *size <= 100_000)
        .sum()
}

pub fn main() {
    println!("{}", part_1())
}
