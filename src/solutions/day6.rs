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
    let rows = read_file_to_list("src/Problems/day6.txt");

    if rows.is_empty() {
        println!("Input file is empty.");
        return;
    }

    let max_width = rows.iter().map(|s| s.len()).max().unwrap_or(0);
    let grid: Vec<Vec<char>> = rows.iter()
        .map(|s| format!("{:width$}", s, width = max_width).chars().collect())
        .collect();

    let operator_row_idx = grid.iter()
        .position(|row| row.contains(&'*') || row.contains(&'+'))
        .expect("No operator row found");

    let mut grand_total: i64 = 0;

    let mut current_block_cols: Vec<usize> = Vec::new();

    for col in 0..max_width {
        let is_empty_col = (0..=operator_row_idx).all(|row| grid[row][col] == ' ');

        if is_empty_col {
            if !current_block_cols.is_empty() {
                grand_total += solve_block(&grid, &current_block_cols, operator_row_idx);
                current_block_cols.clear();
            }
        } else {
            current_block_cols.push(col);
        }
    }

    if !current_block_cols.is_empty() {
        grand_total += solve_block(&grid, &current_block_cols, operator_row_idx);
    }

    println!("=== Grand Total = {} ===", grand_total);
}

fn solve_block(grid: &[Vec<char>], cols: &[usize], op_row_idx: usize) -> i64 {
    let mut operator = None;
    for &col in cols {
        let char_at_op = grid[op_row_idx][col];
        if char_at_op == '+' {
            operator = Some(Operator::Add);
        } else if char_at_op == '*' {
            operator = Some(Operator::Multiply);
        }
    }

    let operator = operator.expect("Problem block missing operator");

    let mut numbers: Vec<i64> = Vec::new();

    for &col in cols {
        let mut num_str = String::new();
        for row in 0..op_row_idx {
            let c = grid[row][col];
            if c.is_digit(10) {
                num_str.push(c);
            }
        }
        if !num_str.is_empty() {
            numbers.push(num_str.parse::<i64>().unwrap());
        }
    }

    match operator {
        Operator::Add => numbers.iter().sum(),
        Operator::Multiply => numbers.iter().product(),
    }
}