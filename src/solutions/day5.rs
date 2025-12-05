use crate::utils::file_utils::read_file_to_list;

pub fn run() {
    let lines = read_file_to_list("src/Problems/day5.txt");

    let mut ranges = Vec::<(i64, i64)>::new();
    let mut values = Vec::<i64>::new();

    for line in lines.iter().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        if let Some((a, b)) = line.split_once('-') {
            let start: i64 = a.trim().parse().unwrap();
            let end: i64 = b.trim().parse().unwrap();
            ranges.push((start, end));
        } else {
            values.push(line.parse().unwrap());
        }
    }

    // Sort ranges by start value
    ranges.sort_by_key(|r| r.0);

    let mut merged = Vec::<(i64, i64)>::new();
    for (start, end) in ranges {
        match merged.last_mut() {
            Some((s, e)) if start <= *e + 1 => {
                *e = (*e).max(end);
            }
            _ => merged.push((start, end)),
        }
    }

    let total_fresh: i64 = merged.iter().map(|(s, e)| e - s + 1).sum();
    println!("total fresh {}", total_fresh);

    let mut count = 0;

    for value in values {
        let found = merged
            .binary_search_by(|&(first, last)| {
                if value < first {
                    std::cmp::Ordering::Greater
                } else if value > last {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .is_ok();

        if found {
            count += 1;
        }
    }

    println!("{}", count);
}
