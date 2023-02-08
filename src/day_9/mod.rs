use std::collections::HashSet;

pub fn main() {
    println!("----- DAY 9 -----");
    println!("Part 1: {}", part_1("src/day_9/data.txt"));
    println!("Part 2: {}", part_2("src/day_9/data.txt"));
}

fn print_status(h_coords: (i32, i32), t_coords: (i32, i32), grid_size: usize) {
    for i in (0..grid_size).rev() {
        let mut line: String = "".to_string();
        for j in 0..grid_size {
            if h_coords.0 == j.try_into().unwrap() && h_coords.1 == i.try_into().unwrap() {
                line.push_str("H");
            } else if t_coords.0 == j.try_into().unwrap() && t_coords.1 == i.try_into().unwrap() {
                line.push_str("T");
            } else {
                line.push_str(".");
            }
        }
        println!("{}", line);
    }
    println!("---------");
}

fn corner_rule(h_coords: &(i32, i32), t_coords: &mut (i32, i32)) {
    if ((h_coords.0 - t_coords.0) > 1 && (h_coords.1 - t_coords.1) > 0) || ((h_coords.0 - t_coords.0) > 0 && (h_coords.1 - t_coords.1) > 1) {
        t_coords.0 += 1;
        t_coords.1 += 1;
    } else if ((h_coords.0 - t_coords.0) > 1 && (h_coords.1 - t_coords.1) < 0) || ((h_coords.0 - t_coords.0) > 0 && (h_coords.1 - t_coords.1) < -1) {
        t_coords.0 += 1;
        t_coords.1 -= 1;
    } else if ((h_coords.0 - t_coords.0) < -1 && (h_coords.1 - t_coords.1) > 0) || ((h_coords.0 - t_coords.0) < 0 && (h_coords.1 - t_coords.1) > 1) {
        t_coords.0 -= 1;
        t_coords.1 += 1;
    } else if ((h_coords.0 - t_coords.0) < -1 && (h_coords.1 - t_coords.1) < 0) || ((h_coords.0 - t_coords.0) < 0 && (h_coords.1 - t_coords.1) < -1) {
        t_coords.0 -= 1;
        t_coords.1 -= 1;
    }
}

fn part_1(file_path: &str) -> usize {
    let input = std::fs::read_to_string(file_path).unwrap();

    let mut h_coords: (i32, i32) = (0, 0);
    let mut t_coords: (i32, i32) = (0, 0);

    let mut t_log: Vec<(i32, i32)> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        print_status(h_coords, t_coords, 30);

        match parts[..] {
            ["U", num] => {
                println!("U{}", num);
                for _ in 0..num.parse::<i32>().unwrap() {
                    h_coords.1 += 1;
                    if h_coords.0 == t_coords.0 && i32::abs(h_coords.1 - t_coords.1) > 1 {
                        t_coords.1 += 1;
                    } else {
                        corner_rule(&h_coords, &mut t_coords);
                    }
                    t_log.push(t_coords);
                }
            }
            ["D", num] => {
                println!("D{}", num);
                for _ in 0..num.parse::<i32>().unwrap() {
                    h_coords.1 -= 1;
                    if h_coords.0 == t_coords.0 && i32::abs(h_coords.1 - t_coords.1) > 1 {
                        t_coords.1 -= 1;
                    } else {
                        corner_rule(&h_coords, &mut t_coords)
                    }
                    t_log.push(t_coords);
                }
            }
            ["L", num] => {
                println!("L{}", num);
                for _ in 0..num.parse::<i32>().unwrap() {
                    h_coords.0 -= 1;
                    if h_coords.1 == t_coords.1 && i32::abs(h_coords.0 - t_coords.0) > 1 {
                        t_coords.0 -= 1;
                    } else {
                        corner_rule(&h_coords, &mut t_coords)
                    }
                    t_log.push(t_coords);
                }
            }
            ["R", num] => {
                println!("R{}", num);
                for _ in 0..num.parse::<i32>().unwrap() {
                    h_coords.0 += 1;
                    if h_coords.1 == t_coords.1 && i32::abs(h_coords.0 - t_coords.0) > 1 {
                        t_coords.0 += 1;
                    } else {
                        corner_rule(&h_coords, &mut t_coords)
                    }
                    t_log.push(t_coords);
                }
            }
            _ => {}
        }
    }

    print_status(h_coords, t_coords, 30);

    println!("{:?}", h_coords);
    println!("{:?}", t_coords);

    // Convert the vec into a HashSet to remove duplicates
    let set: HashSet<_> = t_log.drain(..).collect();

    // Convert the HashSet back into a Vec
    let unique_t_log: Vec<_> = set.into_iter().collect();
    
    for i in (0..30).rev() {
        let mut line: String = "".to_string();
        for j in 0..30 {
            let mut matched = false;
            for t_coords in &unique_t_log {
                if t_coords.0 == j && t_coords.1 == i {
                    matched = true;
                    break;
                }
            }
            if matched {
                line.push_str("#");
            } else {
                line.push_str(".");
            }
        }
        println!("{}", line);
    }

    println!("{}", unique_t_log.len());

    unique_t_log.len()
}

fn part_2(file_path: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use crate::day_9::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("src/day_9/test.txt"), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("src/day_9/test.txt"), 0);
    }
}
