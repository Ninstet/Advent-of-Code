use std::collections::HashMap;
use std::path::PathBuf;

pub fn main() {
    println!("----- DAY 7 -----");
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> u32 {
    let input = std::fs::read_to_string("src/day_7/data.txt").unwrap();
    let filesystem: HashMap<PathBuf, u32> = get_filesystem(input);

    filesystem
        .into_values()
        .filter(|size| *size <= 100_000)
        .sum()
}

fn part_2() -> u32 {
    let input: String = std::fs::read_to_string("src/day_7/data.txt").unwrap();
    let filesystem: HashMap<PathBuf, u32> = get_filesystem(input);

    let remaining_size: u32 = 70_000_000 - filesystem.get(&PathBuf::from("/")).unwrap();
    let required_space: u32 = 30_000_000 - remaining_size;

    filesystem
        .into_values()
        .filter(|size| *size >= required_space)
        .min()
        .unwrap()
}

fn get_filesystem(input: String) -> HashMap<PathBuf, u32> {
    let mut filesystem: HashMap<PathBuf, u32> = HashMap::new();
    let mut pwd: Vec<&str> = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                pwd.pop();
            }
            ["$", "cd", name] => {
                pwd.push(name);
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..pwd.len() {
                    let path = PathBuf::from_iter(&pwd[..=idx]);
                    *filesystem.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }

    filesystem
}
