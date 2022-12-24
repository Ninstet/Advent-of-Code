pub fn main() {
    let (part_1, part_2) = both_parts();

    println!("----- DAY 8 -----");
    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn both_parts() -> (usize, usize) {
    let input = std::fs::read_to_string("data/day_8.txt").unwrap();

    let mut rows: Vec<Vec<u32>> = Vec::new();
    let mut columns: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        let digits: Vec<u32> = line.chars().flat_map(|char| char.to_digit(10)).collect();

        rows.push(digits.clone());

        for i in 0..digits.len() {
            if columns.len() <= i {
                columns.push(vec![digits[i]]);
            } else {
                columns[i].push(digits[i]);
            }
        }
    }

    let mut no_visible: usize = (rows.len() * 2) + (columns.len() * 2) - 4;
    let mut most_visible: usize = 0;

    for i in 1..rows.len() - 1 {
        for j in 1..columns.len() - 1 {
            let value = rows[i][j];

            let rows_left = &rows[i][0..j];
            let rows_right = &rows[i][j + 1..rows.len()];
            let columns_top = &columns[j][0..i];
            let columns_bottom = &columns[j][i + 1..columns.len()];

            if rows_left.iter().all(|x| *x < value) ||
                rows_right.iter().all(|x| *x < value) ||
                columns_top.iter().all(|x| *x < value) ||
                columns_bottom.iter().all(|x| *x < value) {
                
                no_visible += 1;
            }

            let visibility = (1 + rows_left.iter().rev().position(|x| *x >= value).unwrap_or(rows_left.len() - 1)) *
                (1 + rows_right.iter().position(|x| *x >= value).unwrap_or(rows_right.len() - 1)) *
                (1 + columns_top.iter().rev().position(|x| *x >= value).unwrap_or(columns_top.len() - 1)) *
                (1 + columns_bottom.iter().position(|x| *x >= value).unwrap_or(columns_bottom.len() - 1));

            if visibility > most_visible {
                most_visible = visibility;
            }
        }
    }

    (no_visible, most_visible)
}
