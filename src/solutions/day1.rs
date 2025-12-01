use crate::utils::file_utils::*;

pub fn run() {
    let input = read_file("src/Problems/day1.txt");

    let mut pos: i32 = 50;    // dial starts at 50
    let mut total_hits = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        let dir = line.chars().next().unwrap();
        let dist: i32 = line[1..].parse().unwrap();
        let start = pos;

        total_hits += count_zero_hits(start, dist, dir);

        // update the final position
        pos = match dir {
            'L' | 'l' => (start - dist).rem_euclid(100),
            'R' | 'r' => (start + dist).rem_euclid(100),
            _ => panic!("Unknown direction"),
        };
    }

    println!("{}", total_hits);
}

fn count_zero_hits(start: i32, dist: i32, dir: char) -> i32 {
    let sign = match dir {
        'L' | 'l' => -1,
        'R' | 'r' =>  1,
        _ => panic!("Unknown dir"),
    };
    let mut steps_to_zero = ((0 - start).rem_euclid(100) + 100) % 100;
    if sign == -1 {
        steps_to_zero = (100 - steps_to_zero) % 100;
    }
    if steps_to_zero == 0 {
        if dist >= 100 { 1 + (dist - 100) / 100 } else { 0 }
    } else if steps_to_zero > dist {
        0
    } else {
        // One at steps_to_zero, then every +100
        1 + (dist - steps_to_zero) / 100
    }
}
