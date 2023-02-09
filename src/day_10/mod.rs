use std::io;
use std::io::Write; // <--- bring flush() into scope

pub fn main() {
    println!("----- DAY 10 -----");
    println!("Part 1: {}", part_1("src/day_10/data.txt"));
    println!("Part 2: {}", part_2("src/day_10/data.txt"));
}

fn part_1(file_path: &str) -> i32 {
    let input = std::fs::read_to_string(file_path).unwrap();

    let mut X: i32 = 1;
    let mut cycle: i32 = 1;
    let mut signal_strength: Vec<i32> = Vec::new();

    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
            check_signal_strength(&mut signal_strength, line, X, cycle);
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[..] {
            ["addx", num] => {
                cycle += 1;
                check_signal_strength(&mut signal_strength, line, X, cycle);
                cycle += 1;
                X += num.parse::<i32>().unwrap();
                check_signal_strength(&mut signal_strength, line, X, cycle);
            }
            _ => {}
        }
    }

    println!("{:?}", signal_strength);

    signal_strength.iter().sum()
}

fn part_2(file_path: &str) -> i32 {
    let input = std::fs::read_to_string(file_path).unwrap();

    let mut pixel_position: i32 = 0;
    let mut sprite_position: i32 = 1; // Cell with index 1 is the second cell from the left, which is the middle of the 3 cell wide sprite
    let mut display: Vec<bool> = vec![true];

    for line in input.lines() {
        if line == "noop" {
            pixel_position += 1;
            display.push(is_pixel_lit(&mut pixel_position, sprite_position));
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[..] {
            ["addx", num] => {
                pixel_position += 1;
                display.push(is_pixel_lit(&mut pixel_position, sprite_position));
                pixel_position += 1;
                sprite_position += num.parse::<i32>().unwrap();
                display.push(is_pixel_lit(&mut pixel_position, sprite_position));
            }
            _ => {}
        }
    }

    for i in 0..display.len() {
        if i % 40 == 0 {
            println!()
        }
        print!("{}", if display[i] { '#' } else { '.' });
        io::stdout().flush().unwrap();
    }

    println!();
    0
}

fn check_signal_strength(signal_strength: &mut Vec<i32>, line: &str, X: i32, cycle: i32) {
    if (cycle % 40) - 20 == 0 {
        signal_strength.push(X * cycle);
        println!("{}: {} = {}", cycle, line, X);
    }
}

fn is_pixel_lit(pixel_position: &mut i32, sprite_position: i32) -> bool {
    if *pixel_position >= 40 {
        *pixel_position = 0
    }
    *pixel_position == sprite_position
        || *pixel_position == sprite_position - 1
        || *pixel_position == sprite_position + 1
}

#[cfg(test)]
mod test {
    use crate::day_10::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/day_10/test.txt"), 13140);
    }
}
