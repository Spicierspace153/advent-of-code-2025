use std::io::repeat;
use std::str::Chars;
use crate::utils::file_utils::{read_file, read_file_to_list};
pub fn run() {
    let rows = read_file_to_list("src/Problems/day6.txt");
    let raw_numbers: Vec<&String> = rows.iter().take(4).collect();
    let numbers: Vec<Vec<i64>> = raw_numbers
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let tokens = rows
        .iter()
        .rev()
        .take(1)
        .flat_map(|s| s.chars())
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<char>>();

    println!("Tokens: {:?}", tokens);

    let mut total: i64 = 0;

    for (i, &token) in tokens.iter().enumerate() {
        let column_values: Vec<i64> = numbers.iter().map(|row| row[i]).collect();
        let column_result = match token {
            '*' => column_values.iter().product(),
            '+' => column_values.iter().sum(),
            _ => 0,
        };
        
        total += column_result;
    }

    println!("Grand total: {}", total);
}
