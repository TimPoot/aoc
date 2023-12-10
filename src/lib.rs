use std::io::Result;

mod day1;
mod day2;
mod day5;
mod day6;

pub fn run_day(day: u8) -> Result<String> {
    match day {
        1 => day1::run(),
        2 => day2::run(),
        5 => day5::run(),
        6 => day6::run(),
        _ => panic!("Uh oh"),
    }
}