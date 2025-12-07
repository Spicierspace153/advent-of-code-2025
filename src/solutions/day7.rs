use std::collections::HashSet;
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

    let (start_r, start_c) = match (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .find(|&(r, c)| grid[r][c] == 'S')
    {
        Some(pos) => pos,
        None => {
            println!("0");
            return;
        }
    };

    let mut visited_splitters = HashSet::<(usize, usize)>::new();
    let mut total_splits = 0;

    let mut beams: HashSet<usize> = HashSet::new();
    if start_r + 1 < rows {
        beams.insert(start_c);
    }

    for r in (start_r + 1)..rows {
        if beams.is_empty() {
            break;
        }

        let mut next_beams: HashSet<usize> = HashSet::new();
        let mut new_side_beams: HashSet<usize> = HashSet::new();

        for &c in &beams {
            if c >= cols { continue; }

            match grid[r][c] {
                '.' | 'S' => {
                    // beam continues downward
                    next_beams.insert(c);
                }
                '^' => {
                    // splitter
                    if visited_splitters.insert((r, c)) {
                        total_splits += 1;
                    }
                    if c > 0 {
                        new_side_beams.insert(c - 1);
                    }
                    if c + 1 < cols {
                        new_side_beams.insert(c + 1);
                    }
                }
                _ => {
                    // treat as empty
                    next_beams.insert(c);
                }
            }
        }

        for c in new_side_beams {
            if grid[r][c] == '^' {
                if visited_splitters.insert((r, c)) {
                    total_splits += 1;
                }
            } else {
                next_beams.insert(c);
            }
        }

        beams = next_beams;
    }

    println!("Total splits: {}", total_splits);
}
