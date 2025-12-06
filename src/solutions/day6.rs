use crate::utils::file_utils::read_file_to_list;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Multiply,
    Add,
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<i64>,
    operator: Operator,
}

pub fn run() {
    let rows: Vec<String> = read_file_to_list("src/Problems/day6.txt");

    if rows.is_empty() {
        println!("Input file is empty.");
        return;
    }

    let max_width: usize = rows.iter().map(|s| s.len()).max().unwrap_or(0);

    let grid: Vec<Vec<char>> = rows
        .iter()
        .map(|s| format!("{:width$}", s, width = max_width).chars().collect())
        .collect();

    let op_row: usize = grid
        .iter()
        .position(|row| row.contains(&'*') || row.contains(&'+'))
        .expect("No operator row found");

    let values: Vec<Vec<usize>> = (0..max_width)
        .scan(Vec::<usize>::new(), |current_block, col| {
            let is_empty: bool = (0..=op_row).all(|row| grid[row][col] == ' ');
            if is_empty {
                if current_block.is_empty() {
                    Some(None)
                } else {
                    let block: Vec<usize> = current_block.clone();
                    current_block.clear();
                    Some(Some(block))
                }
            } else {
                current_block.push(col);
                Some(None)
            }
        })
        .filter_map(|x| x)
        .collect();

    let total: i64 = values
        .iter()
        .map(|cols: &Vec<usize>| -> i64 {
            let operator: Operator = cols
                .iter()
                .filter_map(|&col| match grid[op_row][col] {
                    '+' => Some(Operator::Add),
                    '*' => Some(Operator::Multiply),
                    _ => None,
                })
                .next()
                .expect("Problem block missing operator");

            let numbers: Vec<i64> = cols
                .iter()
                .filter_map(|&col| {
                    let num_str: String = (0..op_row)
                        .filter_map(|row| {
                            let c: char = grid[row][col];
                            if c.is_digit(10) { Some(c) } else { None }
                        })
                        .collect();
                    if num_str.is_empty() { None } else { Some(num_str.parse::<i64>().unwrap()) }
                })
                .collect();

            match operator {
                Operator::Add => numbers.iter().sum(),
                Operator::Multiply => numbers.iter().product(),
            }
        })
        .sum();

    println!("{} ", total);
}
