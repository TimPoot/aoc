use std::{fs::File, io::{BufReader, BufRead}};


fn main() {
    let filepath = "./input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut winning_games = Vec::<u16>::new();
    let red_limit: u8 = 12;
    let blue_limit: u8 = 13;
    let green_limit: u8 = 14;

    for line in reader.lines() {
        let safe_line = match line {
            Ok(safe_line) => safe_line,
            Err(_) => panic!("Uh oh"),
        };

        let game_id = match safe_line.split(':').collect::<Vec<&str>>().get(0) {
            Some(game_id) => *game_id,
            None => panic!("Uh oh"),
        };
        let game = match safe_line.split(':').collect::<Vec<&str>>().get(1) {
            Some(game) => *game,
            None => panic!("Uh oh"),
        };

        let mut valid_game: bool = true;
        for set in game.split(';') {
            for color_pull in set.split(',') {
                let pull: Vec<&str> = color_pull.split_whitespace().collect();

                let color = match pull.get(1) {
                    Some(color) => *color,
                    None => panic!("Uh oh"),
                };
                let n = match pull.get(0) {
                    Some(color) => color.parse::<u8>().unwrap(),
                    None => panic!("Uh oh"),
                };

                if "red".eq(color) {
                    if n > red_limit {
                        valid_game = false;
                    }
                } else if "green".eq(color) {
                    if n > green_limit {
                        valid_game = false;
                    }
                } else if "blue".eq(color) {
                    if n > blue_limit {
                        valid_game = false;
                    }
                }
            }
        }
        if valid_game {
            let winning_game_id = match game_id.split_whitespace().collect::<Vec<&str>>().get(1) {
                Some(game_id) => game_id.parse::<u16>().unwrap(),
                None => todo!(),
            };
            winning_games.push(winning_game_id);
        }
    }

    println!("Winning games: {:?}", winning_games);
    println!("Sum of winningn games: {}", winning_games.iter().sum::<u16>())
}
