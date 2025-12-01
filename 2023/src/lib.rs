use std::io::Result;

mod day1;
mod day2;
mod day4;
mod day5;
mod day6;
mod day8;
mod day11;

pub fn run_day(day: u8) -> Result<String> {
    match day {
        1 => day1::run(),
        2 => day2::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        8 => day8::run(),
        11 => day11::run(),
        _ => panic!("Uh oh"),
    }
}