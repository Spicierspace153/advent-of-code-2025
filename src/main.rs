use std::env;
use crate::solutions::day1;

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
        _ => println!("Eric didnt do that day yet!"),
    }
}
