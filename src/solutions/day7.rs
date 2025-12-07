use std::collections::{HashSet, VecDeque};
use crate::utils::file_utils::read_file_to_list;

pub fn run() {
    let inputs = read_file_to_list("src/Problems/day7.txt");
    if inputs.is_empty() {
        println!("0");
        return;
    }

    let grid: Vec<Vec<char>> = inputs.iter().map(|s| s.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();


    let (start_r, start_c) = {
        let mut sr = 0;
        let mut sc = 0;
        let mut found = false;

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 'S' {
                    sr = r;
                    sc = c;
                    found = true;
                    break;
                }
            }
            if found { break; }
        }

        if !found {
            println!("0");
            return;
        }

        (sr, sc)
    };

    let mut counted = HashSet::<(usize, usize)>::new();
    let mut total_splits = 0usize;

    let mut next_row_beams = HashSet::<usize>::new();
    if start_r + 1 < rows {
        next_row_beams.insert(start_c);
    }


    for r in (start_r + 1)..rows {
        let current_beams = next_row_beams;
        next_row_beams = HashSet::new();

        let mut queue = VecDeque::<usize>::new();
        let mut seen = HashSet::<usize>::new();

        for &c in &current_beams {
            if c < cols && seen.insert(c) {
                queue.push_back(c);
            }
        }

        let mut down_cols = HashSet::<usize>::new();

        while let Some(c) = queue.pop_front() {
            if c >= cols { continue; }

            match grid[r][c] {
                '.' | 'S' => {
                    // Beam passes downward
                    down_cols.insert(c);
                }
                '^' => {
                    if counted.insert((r, c)) {
                        total_splits += 1;
                    }

                    if c > 0 && seen.insert(c - 1) {
                        queue.push_back(c - 1);
                    }
                    if c + 1 < cols && seen.insert(c + 1) {
                        queue.push_back(c + 1);
                    }
                }
                _ => {
                    down_cols.insert(c);
                }
            }
        }

        next_row_beams = down_cols;
        if next_row_beams.is_empty() {
            break;
        }
    }

    println!("Total splits: {}", total_splits);
}
