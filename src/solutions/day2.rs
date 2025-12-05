use std::time::Instant;
use crate::utils::file_utils::*;


pub fn run(){

    let input = read_file("src/Problems/day2.txt");

    let ranges : Vec<String> = input
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let mut total: i64 = 0;
    for  range in ranges {

        let values: Vec<i64> = range
            .split("-")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();
        let start = values[0];
        let end = values[1];
        for  n in start..=end {

            // hahahahahha im a fucking moron
            if is_valid(n){
                total += n;
            }

        }
    }
    println!("Total: {}", total);
}

fn is_valid(n: i64) -> bool{
    let number = n.to_string();
    let len = number.len();
    // try all chunks
    for i in 1..=len/2 {
        if len % i != 0 {
            continue;
        }

        let chunk = &number[..i];

        // create the seperated sections
        let repeated = chunk.repeat(len / i);
        if repeated == number {
            // check the chunks
            return true;
        }
    }
    false
}
