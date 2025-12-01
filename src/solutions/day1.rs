use crate::utils::file_utils::*;

pub fn run() {
    let input = read_file("src/Problems/day1.txt");

    // dial starts at 50
    let mut pos: i32 = 50;
    //

    let mut total_hits = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        // parse string
        let dir = line.chars().next().unwrap();
        let dist: i32 = line[1..].parse().expect("dumbass");
        let start = pos;

        let mut current_hits = 0;

        // ngl this could be smarter im just a dumb potato
        //simulate each dial run pass manually i couldn't find a way to do this better tbh
        for i in 1..=dist {
            let step_pos = match dir {
                'L' | 'l' => (start - i).rem_euclid(100),
                'R' | 'r' => (start + i).rem_euclid(100),
                _ => panic!("Unknown direction"),
            };
            //0 check
            if step_pos == 0 {
                current_hits += 1;
            }
        }
        total_hits += current_hits;
        // old part1 code
        // actually update the position
        pos = match dir {
            'L' | 'l' => (start - dist).rem_euclid(100),
            'R' | 'r' => (start + dist).rem_euclid(100),
            _ => panic!("Unknown direction"),
        };
    }

    println!("{}", total_hits);
}
