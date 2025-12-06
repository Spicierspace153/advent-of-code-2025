use crate::utils::file_utils::read_file_to_list;

pub fn run() {
    let rows = read_file_to_list("src/Problems/day6.txt");

    let raw_numbers: Vec<&String> = rows.iter().take(3).collect();
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

    for i in (0..tokens.len()).rev() {
        let column_strings: Vec<String> = numbers
            .iter()
            .map(|row| row[i].to_string())
            .collect();

        let max_length = column_strings.iter().map(|s| s.len()).max().unwrap();

        let padded: Vec<String> = column_strings
            .iter()
            .map(|s| format!("{:0<width$}", s, width = max_length))
            .collect();

        let split_digits: Vec<Vec<char>> = padded
            .iter()
            .map(|s| s.chars().collect())
            .collect();

        for j in 0..max_length {
            let digits: String = split_digits.iter().map(|row| row[j]).collect();
            println!("{}", digits);
        }
    }

    println!("Grand total: {}", total);
}
