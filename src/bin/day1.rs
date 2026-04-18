use std::fs;

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

fn solve() -> i32 {
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

fn main() {
    let res = solve();
    println!("{res}");
}
