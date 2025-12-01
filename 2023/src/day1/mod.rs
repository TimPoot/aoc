use std::{io::{BufReader, BufRead, Result}, fs::File};

pub fn run() -> Result<String> {
    let filepath = "./src/day1/input.txt";
    let file = File::open(filepath)?;
    println!("THIS HAPPENS");
    let reader = BufReader::new(file);

    let mut number_list = Vec::<u32>::new();

    for line in reader.lines() {
        let digit_vec = line?
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .chars()
            .filter(|char| char.is_digit(10))
            .map(|char| char.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        number_list.push(10 * digit_vec.first().unwrap() + digit_vec.last().unwrap());
    }
    let sum: u32 = number_list.iter().sum();
    println!("Sum: {}", sum);

    Ok(sum.to_string())
}
