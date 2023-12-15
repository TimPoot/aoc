use std::{fs::File, io::{BufReader, BufRead, Result}};

#[derive(Debug, Clone)]
struct Card {
    multiplier: u64,
}

impl Default for Card {
    fn default() -> Self {
        Self {multiplier: 1}
    }
}

pub fn run() -> Result<String> {
    let filepath = "./src/day4/input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let mut cards = vec![Card::default(); lines.len()];

    for (i, line) in lines.iter().enumerate() {
        for card in line.split(':').skip(1) {
            if let Some((winning_numbers_str, own_numbers_str)) = card.split_once('|') {
                let winning_numbers: Vec<u8> = winning_numbers_str.split_whitespace()
                    .filter_map(|x| x.parse().ok())
                    .collect();
                let own_numbers: Vec<u8> = own_numbers_str.split_whitespace()
                    .filter_map(|x| x.parse().ok())
                    .collect();

                let winning_count: usize = own_numbers.iter()
                    .filter(|x| winning_numbers.contains(x))
                    .count();

                let current_multiplier = cards.get(i as usize).unwrap().multiplier;
                for j in 0..winning_count {
                    match cards.get_mut(i + j + 1) {
                        Some(card) => card.multiplier += 1 * current_multiplier,
                        None => break,
                    }
                }

            }
        }
    }
    let total_winnings: u64 = cards.iter()
        .map(|x| x.multiplier)
        .sum();

    Ok(total_winnings.to_string())
}