// sample of input: Game 1: 9 red, 2 green, 13 blue; 10 blue, 2 green, 13 red; 8 blue, 3 red, 6 green; 5 green, 2 red, 1 blue

use std::cmp;
use std::fs;

#[derive(Debug)]
struct Cube {
    id: i32,
    red: i32,   // max 12
    green: i32, // max 13
    blue: i32,  // max 14
}

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn parse() -> Vec<Vec<Cube>> {
    let file_path: &str = "input_day2.txt";
    let content =
        fs::read_to_string(file_path).expect(&format!("Error reading through {}", file_path));
    let mut set_list: Vec<Vec<Cube>> = Vec::new();

    for line in content.lines() {
        let mut game_list: Vec<Cube> = Vec::new();
        let (game, colors) = line.split_once(':').unwrap();
        // finding the id by taking the second element after the space chracter. ex : Game 1: ... -> ["Game", "1"] -> 1
        let id: i32 = game.split_whitespace().collect::<Vec<_>>()[1]
            .parse()
            .unwrap();

        let colors_vec: Vec<&str> = colors.split(';').collect();

        for sequence in colors_vec {
            let set: Vec<&str> = sequence.trim().split(',').collect();
            let (mut red, mut green, mut blue) = (0, 0, 0);
            for element in set {
                let value: &mut i32;
                if element.contains("red") {
                    value = &mut red;
                } else if element.contains("green") {
                    value = &mut green;
                } else if element.contains("blue") {
                    value = &mut blue;
                } else {
                    continue;
                }

                *value = element.split_whitespace().nth(0).unwrap().parse().unwrap();
            }

            // by default if there is no value, the field will be = 0
            let game = Cube {
                id: id,
                red: red,
                green: green,
                blue: blue,
            };
            game_list.push(game);
        }
        set_list.push(game_list);
    }
    set_list
}

fn solve_part1(input_parsed: Vec<Vec<Cube>>) -> i32 {
    let mut res = 0;
    for set in input_parsed {
        let mut valid = true;
        for game in &set {
            if valid == false {
                break;
            }

            if game.blue > MAX_BLUE || game.red > MAX_RED || game.green > MAX_GREEN {
                valid = false;
            }
        }
        if valid {
            res += set[0].id;
        }
    }
    res
}

fn solve_part2(input_parsed: Vec<Vec<Cube>>) -> i32 {
    let mut res = 0;
    for set in input_parsed {
        let (mut max_red, mut max_blue, mut max_green) = (0, 0, 0);
        for game in &set {
            max_blue = cmp::max(game.blue, max_blue);
            max_green = cmp::max(game.green, max_green);
            max_red = cmp::max(game.red, max_red);
        }
        res += max_blue * max_green * max_red;
    }
    res
}

fn main() {
    let parsed_input = parse();
    //let res_d1 = solve_part1(parsed_input);
    //println!("answer d1 : {}", res_d1);
    let res_d2 = solve_part2(parsed_input);
    println!("answer d2 : {}", res_d2);
}
