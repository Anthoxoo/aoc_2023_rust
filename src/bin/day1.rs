use std::{collections::HashMap, fs};

fn parse() -> Vec<String> {
    let file_path = "input_day1.txt";
    let content =
        fs::read_to_string(file_path).expect(&format!("Error reading through {}", file_path));
    let mut content_vec = Vec::new();
    for line in content.lines() {
        content_vec.push(line.to_string());
    }

    content_vec
}

fn solve_part1() -> i32 {
    let content = parse();
    let mut result = 0;

    for elem in content {
        let mut number_vec: Vec<i32> = Vec::new();

        for carac in elem.chars() {
            let parsed_carac = carac.to_string().parse();

            match parsed_carac {
                Ok(number) => {
                    number_vec.push(number);
                }
                Err(_) => continue,
            }
        }

        if number_vec.len() >= 2 {
            let first_number = number_vec.first().expect("No first value");
            let last_number = number_vec.last().expect("No last value");
            result += *first_number * 10 + *last_number;
        } else if number_vec.len() == 1 {
            let first_number = number_vec.first().expect("No first value");
            result += *first_number * 10 + *first_number;
        }
    }

    result
}

fn solve_part2() -> u32 {
    let content = parse();
    let mut result = 0;
    let word_to_number: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
    ]);

    for elem in content {
        let mut number_vec: Vec<u32> = Vec::new();

        for i in 0..elem.len() {
            let rest_line = &elem[i..];
            let first_carac = rest_line.chars().next().unwrap();

            if let Some(digit) = first_carac.to_digit(10) {
                number_vec.push(digit)
            } else {
                for (key, value) in &word_to_number {
                    if rest_line.starts_with(key) {
                        number_vec.push(*value)
                    }
                }
            }
        }

        if number_vec.len() >= 2 {
            let first_number = number_vec.first().expect("No first value");
            let last_number = number_vec.last().expect("No last value");

            result += *first_number * 10 + *last_number;
        } else if number_vec.len() == 1 {
            let first_number = number_vec.first().expect("No first value");

            result += *first_number * 10 + *first_number;
        }
    }

    result
}

fn main() {
    // let res = solve_part1();
    let res = solve_part2();

    println!("{res}");
}
