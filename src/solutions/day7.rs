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
    let mut total_splits = 0usize;

    let mut beams: HashSet<usize> = HashSet::new();
    let mut visited_cells = HashSet::<(usize, usize)>::new();

    if start_r + 1 < rows {
        beams.insert(start_c);
    }

    for r in (start_r + 1)..rows {
        if beams.is_empty() {
            break;
        }

        let mut next_beams = HashSet::new();
        let mut new_side_beams = HashSet::new();

        for &c in &beams {
            if c >= cols {
                continue;
            }

            visited_cells.insert((r, c));

            match grid[r][c] {
                '.' | 'S' => {
                    next_beams.insert(c);
                }
                '^' => {
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
                    next_beams.insert(c);
                }
            }
        }

        for c in new_side_beams {
            visited_cells.insert((r, c));
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

    println!("Part 1 - Total splits: {}", total_splits);


    let mut dp_curr = vec![0u128; cols];
    if start_r + 1 < rows {
        dp_curr[start_c] = 1;
    }

    for r in (start_r + 1)..rows {
        let mut row_inflight = dp_curr.clone();

        loop {
            let mut changed = false;
            let mut next = row_inflight.clone();

            for c in 0..cols {
                let count = row_inflight[c];
                if count == 0 {
                    continue;
                }
                if grid[r][c] == '^' {
                    next[c] = 0; // consumed
                    if c > 0 {
                        next[c - 1] += count;
                    }
                    if c + 1 < cols {
                        next[c + 1] += count;
                    }
                    changed = true;
                }
            }

            row_inflight = next;
            if !changed {
                break;
            }
        }

        let mut dp_next = vec![0u128; cols];
        for c in 0..cols {
            let count = row_inflight[c];
            if count == 0 {
                continue;
            }
            if grid[r][c] != '^' {
                dp_next[c] += count;
            }
        }

        dp_curr = dp_next;
    }

    let total_timelines: u128 = dp_curr.iter().sum();
    println!("Part 2 - Total timelines: {}", total_timelines);

}
