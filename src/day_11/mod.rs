pub fn main() {
    println!("----- DAY 11 -----");
    println!("Part 1: {}", part_1("src/day_11/data.txt"));
    println!("Part 2: {}", part_2());
}

#[derive(Debug, Clone)]
enum Operator {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    NONE,
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    rhs: u32,
}

#[derive(Debug, Clone)]
struct Monkey {
    id: u32,
    items: Vec<u32>,
    operation: Option<Operation>,
    test: u32,
    true_monkey: u32,
    false_monkey: u32,
}

impl Monkey {
    fn default() -> Self {
        Self {
            id: 0,
            items: vec![],
            operation: Option::None,
            test: 0,
            true_monkey: 0,
            false_monkey: 0,
        }
    }
}

fn parse_monkeys(input: String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut current_monkey: Monkey = Monkey::default();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts[..] {
            ["Monkey", id] => {
                current_monkey.id = id.trim_end_matches(':').parse::<u32>().unwrap();
            }
            ["Starting", "items:", ..] => {
                let mut items = vec![];
                for item in &parts[2..] {
                    if let Ok(number) = item.trim_end_matches(',').parse::<u32>() {
                        items.push(number);
                    }
                }
                if !items.is_empty() {
                    current_monkey.items = items;
                }
            }
            ["Operation:", "new", "=", "old", operator, rhs] => {
                current_monkey.operation = Some(Operation {
                    operator: match operator {
                        "+" => Operator::ADD,
                        "-" => Operator::SUBTRACT,
                        "*" => Operator::MULTIPLY,
                        "/" => Operator::DIVIDE,
                        _ => Operator::NONE,
                    },
                    rhs: rhs.parse::<u32>().unwrap_or(0),
                });
            }
            ["Test:", "divisible", "by", val] => {
                current_monkey.test = val.parse::<u32>().unwrap();
            }
            ["If", "true:", "throw", "to", "monkey", val] => {
                current_monkey.true_monkey = val.parse::<u32>().unwrap();
            }
            ["If", "false:", "throw", "to", "monkey", val] => {
                current_monkey.false_monkey = val.parse::<u32>().unwrap();
            }
            [] => {
                monkeys.push(current_monkey.clone());
            }
            _ => {}
        }
    }

    monkeys.push(current_monkey.clone());
    monkeys
}

fn calculate_round(monkeys: &mut Vec<Monkey>, verbose: bool) -> Vec<u32> {
    let mut inspections: Vec<u32> = Vec::new();
    for _ in 0..monkeys.len() {
        inspections.push(0);
    }

    for id in 0..monkeys.len() {
        let monkey = monkeys[id].clone();
        let operation = monkey.operation.clone().unwrap();

        if verbose {
            println!("Monkey {}:", monkey.id);
        }

        inspections[id] += monkeys[id].items.len() as u32;

        for monkey_item in &monkey.items {
            let mut item = monkey_item.clone();
            let orig_item = item;

            // Monkey inspects item
            if verbose {
                println!("  Monkey inspects an item with a worry level of {}.", item);
            }
            let rhs = match operation.rhs {
                0 => item.clone(),
                _ => operation.rhs,
            };
            match operation.operator {
                Operator::ADD => {
                    item += rhs;
                    if verbose {
                        println!("    Worry level increases by {} to {}.", rhs, item);
                    }
                }
                Operator::SUBTRACT => {
                    item -= rhs;
                    if verbose {
                        println!("    Worry level decreases by {} to {}.", rhs, item);
                    }
                }
                Operator::MULTIPLY => {
                    item *= rhs;
                    if verbose {
                        println!("    Worry level is multiplied by {} to {}.", rhs, item);
                    }
                }
                Operator::DIVIDE => {
                    item /= rhs;
                    if verbose {
                        println!("    Worry level divided by {} to {}.", rhs, item);
                    }
                }
                Operator::NONE => panic!(),
            }

            // Monkey gets bored
            item /= 3;
            if verbose {
                println!(
                    "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                    item
                );
            }

            // Check test
            let index = monkeys[monkey.id as usize]
                .items
                .iter()
                .position(|i| *i == orig_item)
                .unwrap();
            monkeys[monkey.id as usize].items.remove(index);
            if item % monkey.test == 0 {
                // Give to true monkey
                if verbose {
                    println!("    Current worry level is divisible by {}.", monkey.test);
                }
                monkeys[monkey.true_monkey as usize].items.push(item);
                if verbose {
                    println!(
                        "    Item with worry level {} is thrown to monkey {}.",
                        item, monkey.true_monkey
                    );
                }
            } else {
                // Give to false monkey
                if verbose {
                    println!(
                        "    Current worry level is not divisible by {}.",
                        monkey.test
                    );
                }
                monkeys[monkey.false_monkey as usize].items.push(item);
                if verbose {
                    println!(
                        "    Item with worry level {} is thrown to monkey {}.",
                        item, monkey.false_monkey
                    );
                }
            }
        }
    }

    inspections
}

fn print_items(monkeys: &Vec<Monkey>) {
    for monkey in monkeys {
        println!("Monkey {}: {:?}", monkey.id, monkey.items);
    }
}

fn part_1(file_path: &str) -> u32 {
    let input = std::fs::read_to_string(file_path).unwrap();

    let mut monkeys: Vec<Monkey> = parse_monkeys(input);

    let mut inspections: Vec<u32> = Vec::new();
    for _ in 0..monkeys.len() {
        inspections.push(0);
    }

    for round in 1..21 {
        println!("\nRound {}:", round);
        let round_inspections = calculate_round(&mut monkeys, false);
        print_items(&monkeys);

        for id in 0..monkeys.len() {
            inspections[id] += round_inspections[id];
        }
    }

    println!("\nInspections: {:?}", inspections);

    inspections.sort_unstable_by(|a, b| b.cmp(a));
    inspections[0] * inspections[1]
}

fn part_2() -> u32 {
    0
}
