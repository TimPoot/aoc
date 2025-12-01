#![allow(unused)]
use std::{fs::File, io::{BufReader, BufRead, Result}, cmp::{max}};

pub fn run() -> Result<String> {
    let filepath = "./src/day2/input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut winning_games = Vec::<u32>::new();
    let mut sum_of_powers: u32 = 0;
    let red_limit: u8 = 12;
    let blue_limit: u8 = 14;
    let green_limit: u8 = 13;

    for line in reader.lines() {
        let usable_line = line.unwrap();
        let mut game_id: u8 = 0;

        let mut red_min: u8 = 0;
        let mut blue_min: u8 = 0;
        let mut green_min: u8 = 0;

        for (i, part) in usable_line.split(':').enumerate() {
            if 0 == i {
                // Get game ID
                game_id = part.split_whitespace()
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .parse::<u8>()
                    .unwrap();

            } else {
                // process actual game
                for pull in part.split([',', ';']) {
                    let mut pull_size: u8 = 0;

                    for (j, pull_part) in pull.split_whitespace().enumerate() {
                        if 0 == j {
                            pull_size = pull_part.parse::<u8>().unwrap();
                        } else {
                            match pull_part {
                                "red" => red_min = max(red_min, pull_size),
                                "green" => green_min = max(green_min, pull_size),
                                "blue" => blue_min = max(blue_min, pull_size),
                                _ => println!("Something weird has happened"),
                            }
                        }
                    }
                }

                sum_of_powers += red_min as u32 * green_min as u32 * blue_min as u32;
                println!("In between count: {}", sum_of_powers);
            }
        }
    }

    // let winning_games_sum: u32 = winning_games.iter().map(|&x| x as u32).sum();
    // println!("Winning games: {:?}", winning_games);
    // println!("Sum of winningn games: {}", winning_games_sum);
    println!("Sum of powers: {}", sum_of_powers);

    Ok(sum_of_powers.to_string())
}
