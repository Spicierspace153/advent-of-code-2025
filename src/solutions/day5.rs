use crate::utils::file_utils::read_file_to_list;

pub fn run() {
    let lines = read_file_to_list("src/Problems/day5.txt");

    let mut ranges = Vec::<(i64, i64)>::new();
    let mut values = Vec::<i64>::new();

    for line in lines.into_iter().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()) {
        if let Some((a, b)) = line.split_once('-') {
            ranges.push((a.trim().parse().unwrap(), b.trim().parse().unwrap()));
        } else {
            values.push(line.parse().unwrap());
        }
    }

    // sort values
    ranges.sort_by_key(|r| r.0);

    // new list to merge it all together
    let mut merged = Vec::<(i64, i64)>::new();

    //overlap and deduplication for once nice clean list
    for (start, end) in ranges {
        match merged.last_mut() {
            Some((s, e)) if start <= *e + 1 => *e = (*e).max(end),

            //push a new new list if seperate
            _ => merged.push((start, end)),
        }
    }

    let mut count = 0;

    let mut total_fresh = 0;
    for (start, end) in &merged {
        total_fresh += end - start + 1;
    }
    println!("total fresh {}", total_fresh);

    for value in values {
        //the input list was soooooooooo large i decided to go with binary search
        let found = merged.binary_search_by(|&(first, last)| {
            if value < first {
                std::cmp::Ordering::Greater
            }
            else if value > last {
                std::cmp::Ordering::Less
            }
            else { std::cmp::Ordering::Equal }
        }).is_ok();

        if found {
            count += 1;
        }
    }

    println!("{}", count);
}
