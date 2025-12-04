use crate::utils::file_utils::read_file_to_list;

pub fn run() {
    let lines: Vec<String> = read_file_to_list("src/Problems/day3.txt");

    let mut total: i128 = 0;

    for line in lines {
        println!("for list {}", line);

        let digits: Vec<i32> = line.chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        let mut max_charge = [0; 12];
        let mut out_index = 0;
        let mut start = 0;
        let needed = max_charge.len();
        let n = digits.len();
        let mut remaining = needed;
        while remaining > 0 {
            let end = n - remaining;
            let mut max_digit = -1;
            let mut max_index = start;
            for i in start..=end {
                if digits[i] > max_digit {
                    max_digit = digits[i];
                    max_index = i;
                }
            }
            max_charge[out_index] = max_digit;
            out_index += 1;
            remaining -= 1;
            start = max_index + 1;
        }
        let mut value: i128 = 0;
        println!("max_charge = {:?}", max_charge);
        for d in max_charge {
            value = value * 10 + d as i128;
        }
        total += value;
    }

    println!("Total = {}", total);
}

pub fn part_1(){
    let input_list: Vec<String> = read_file_to_list("src/Problems/day3.txt");

    let mut total = 0;

    for line in input_list {
        let digits: Vec<i32> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();

        let mut best = 0;

        for i in 0..digits.len() {
            for j in i + 1..digits.len() {
                //creates a two-digit number
                let value = digits[i] * 10 + digits[j];
                if value > best {
                    best = value;
                }
            }
        }

        total += best;
        println!("best for {} {}", line, best);
    }

    println!("{}", total);
}