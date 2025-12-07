use std::env;
use crate::solutions::{day1, day2, day3, day4, day5, day6, day7};

mod solutions;
mod utils;

fn main() {

    let args: Vec<String> = env::args().collect();
    let day: u8 = match args[1].parse() {

        Ok(num) => num,
        Err(_) => {
            eprintln!("Eric didnt do that day yet!");
            return;
        }
    };

    match day {

        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),
        _ => println!("Eric didnt do that day yet!"),
    }
}
