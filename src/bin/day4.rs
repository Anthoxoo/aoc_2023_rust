// sample of input : Card   1: 34 50 18 44 19 35 47 62 65 26 | 63  6 27 15 60  9 98  3 61 89 31 43 80 37 54 49 92 55  8  7 10 16 52 33 45
use std::fs;

struct ScratchGame {
    winning_cards: Vec<i32>,
    game_cards: Vec<i32>,
}

fn parse_input() -> Vec<ScratchGame> {
    let file_path = "input_day3.txt";
    let binding =
        fs::read_to_string(&file_path).expect(&format!("Error reading through {}", &file_path));
    let mut parsed: Vec<ScratchGame> = Vec::new();
    for line in binding.lines() {
        let content: Vec<&str> = line.split(':').nth(1).unwrap().split('|').collect();

        let winning: Vec<i32> = content[0]
            .split_whitespace()
            .filter_map(|elem| elem.parse().ok())
            .collect();
        let game: Vec<i32> = content[1]
            .split_whitespace()
            .filter_map(|elem| elem.parse().ok())
            .collect();
        let current = ScratchGame {
            winning_cards: winning,
            game_cards: game,
        };

        parsed.push(current);
    }

    parsed
}

fn solve_part1(parsed_input: Vec<ScratchGame>) -> i32 {
    let mut res = 0;

    for elem in parsed_input {
        let mut nb_found = 0;
        for winning in elem.winning_cards {
            if elem.game_cards.contains(&winning) {
                nb_found += 1;
            }
        }

        let mut card_points = 0;

        for _ in 0..nb_found {
            if card_points == 0 {
                card_points = 1;
            } else {
                card_points *= 2;
            }
        }
        res += card_points;
    }
    res
}

fn main() {
    dbg!(solve_part1(parse_input()));
}
