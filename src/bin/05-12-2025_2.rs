use std::ops::RangeInclusive;

use ::aoc2025::read_input;

fn main() {
    let input: String = read_input("inputs/05-12-2025.txt");
    let (ranges, _) = input.split_once("\n\n").unwrap();
    // let mut ingredients: HashSet<u64> = HashSet::new();
    let mut sorted_ranges: Vec<&str> = ranges.lines().collect::<Vec<&str>>();
    sorted_ranges.sort();
    let mut final_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    for range in sorted_ranges.iter() {
        let parts: Vec<&str> = range.split('-').collect();
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        if let Some(found_range) = final_ranges.iter_mut().find(|r| {
            r.contains(&start) || r.contains(&end) || (start <= *r.start() && end >= *r.end())
        }) {
            let new_start: u64 = std::cmp::min(*found_range.start(), start);
            let new_end: u64 = std::cmp::max(*found_range.end(), end);
            *found_range = new_start..=new_end;
        } else {
            final_ranges.push(start..=end);
        }
    }

    println!(
        "{}",
        final_ranges
            .iter()
            .fold(0u64, |acc, range| acc + *range.end() - *range.start() + 1)
    );
}
