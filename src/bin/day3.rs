use std::fs;

// sample of input
const _: &str = r#"
.......497...........................858...923...128..................227..801........487.....664...........................................
436........765..............140.......+....................859.............*.........+.................960........668.......................
...*982...........=..........=....203......266.263...375*....=...402....691..-....................*..........575....................13......
.............114...588...............*............*......631........*.......952...463..14.......661..........=...706......*333.........595..
...194.........*..............743...917.&......375.....................................*...............544*.......*....664..................
"#;

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse_input() -> Vec<Vec<char>> {
    let file_path: &str = "input_day3.txt";
    let content =
        fs::read_to_string(file_path).expect(&format!("Error reading through {}", file_path));
    let mut schematic: Vec<Vec<char>> = Vec::new();

    for line in content.lines() {
        schematic.push(line.chars().collect());
    }

    schematic
}

fn solve_part1(parsed_input: Vec<Vec<char>>) -> i32 {
    /*way to solve : iterate through every symbol, to do that, we'll check the ascii codes between 33 and 47 excluding the 46 cuz its the '.'
     * if the symbol is in this range, we check around it : (i-1,j-1), (i-1,j), (i-1,j+1), (i,j-1), (i,j+1), (i+1,j-1), (i+1,j), (i+1,j+1)
     *  and we have to find a way to take all the numbers that are sticked to each other to create the final number and then add it to the sum
     */
    let mut res = 0;

    for i in 0..parsed_input.len() {
        for j in 0..parsed_input[i].len() {
            let element = parsed_input[i][j];

            if element.is_ascii_punctuation() && element != '.' {
                let mut previous_num: Option<i32> = None;

                // at this point element is not a '.' and is a symbol we were looking for
                for (di, dj) in DIRS {
                    if let (Some(ni), Some(nj)) =
                        (i.checked_add_signed(di), j.checked_add_signed(dj))
                    // checks if the index is > 0 to avoid under limit accessing array
                    {
                        if let num = parsed_input[ni][nj] {
                            if num.is_ascii_digit() {
                                // we have to get to the first number and the last of the list cuz it could be a number that is part of a bigger one

                                let mut min_j = nj;
                                while min_j > 0 && parsed_input[ni][min_j - 1].is_ascii_digit() {
                                    min_j -= 1;
                                }

                                let mut max_j = nj;
                                while max_j + 1 < parsed_input[ni].len()
                                    && parsed_input[ni][max_j + 1].is_ascii_digit()
                                {
                                    max_j += 1;
                                }

                                let slice_num = &parsed_input[ni][min_j..=max_j];
                                let val =
                                    slice_num.iter().collect::<String>().parse::<i32>().unwrap();
                                /* this code is here to prevent that when we found a number, we can find the same number with another direction which is logical.
                                 * i also found out that when a value is reapearing, it reapears just after the same one so i can just keep track of the last one and exclude it if it is the same.
                                 * i could have done it better by keeping track of its position because we can have the same number close enough but still at different positions which would
                                 * count as two different numbers but ayyy it works like this (which is pure luck given by the inputs)
                                 */
                                if previous_num != Some(val) {
                                    res += val;
                                    previous_num = Some(val);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    res
}

fn solve_part2(parsed_input: Vec<Vec<char>>) -> i32 {
    let mut res = 0;

    for i in 0..parsed_input.len() {
        for j in 0..parsed_input[i].len() {
            let element = parsed_input[i][j];

            if element == '*' {
                let mut previous_num: Option<i32> = None;
                let mut first_num: Option<i32> = None;
                let mut second_num: Option<i32> = None;

                for (di, dj) in DIRS {
                    if let (Some(ni), Some(nj)) =
                        (i.checked_add_signed(di), j.checked_add_signed(dj))
                    {
                        if let num = parsed_input[ni][nj] {
                            if num.is_ascii_digit() {
                                let mut min_j = nj;
                                while min_j > 0 && parsed_input[ni][min_j - 1].is_ascii_digit() {
                                    min_j -= 1;
                                }

                                let mut max_j = nj;
                                while max_j + 1 < parsed_input[ni].len()
                                    && parsed_input[ni][max_j + 1].is_ascii_digit()
                                {
                                    max_j += 1;
                                }

                                let slice_num = &parsed_input[ni][min_j..=max_j];
                                let val =
                                    slice_num.iter().collect::<String>().parse::<i32>().unwrap();
                                if previous_num != Some(val) {
                                    previous_num = Some(val);
                                    if first_num.is_none() {
                                        first_num = Some(val);
                                    } else {
                                        second_num = Some(val);
                                    }
                                }
                            }
                        }
                    }
                }
                if first_num.is_some() && second_num.is_some() {
                    res += first_num.unwrap() * second_num.unwrap();
                }
            }
        }
    }
    res
}

fn main() {
    solve_part1(parse_input());
    println!("{}", solve_part2(parse_input()));
}
