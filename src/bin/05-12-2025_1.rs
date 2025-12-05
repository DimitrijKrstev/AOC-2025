use std::ops::RangeInclusive;

use ::aoc2025::read_input;

fn main() {
    let input: String = read_input("inputs/05-12-2025.txt");
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let mut range_map: Vec<RangeInclusive<u64>> = Vec::new();

    for range in ranges.lines() {
        let parts: Vec<&str> = range.split('-').collect();
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();
        range_map.push(start..=end);
    }

    let mut counter: u32 = 0;

    for ingredient in ingredients.lines() {
        let ingredient_value: u64 = ingredient.parse().unwrap();
        if range_map
            .iter()
            .any(|range| range.contains(&ingredient_value))
        {
            counter += 1;
        }
    }

    println!("{}", counter)
}
