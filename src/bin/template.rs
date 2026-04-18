use std::fs;

fn parse() {
    let file_path = "input_day1.txt";
    let content =
        fs::read_to_string(file_path).expect(&format!("Error reading through {}", file_path));
}

fn solve() {}

fn main() {
    let res = solve();
    //println!("{res}");
}
