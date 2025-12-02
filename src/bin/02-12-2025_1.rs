use ::aoc2025::read_input;
use std::ops::RangeInclusive;

const BASE_POW: u32 = 10;

fn main() {
    let input: String = read_input("inputs/02-12-2025.txt");
    let ranges: Vec<&str> = input.split(',').collect();
    let mut sum: u64 = 0;

    for range in ranges {
        let parts: Vec<&str> = range.split('-').collect();
        let start: u32 = parts[0].parse().unwrap();
        let end: u32 = parts[1].parse().unwrap();
        let range: RangeInclusive<u32> = start..=end;

        for num in range {
            let length: u32 = num.checked_ilog10().unwrap_or(0) + 1;
            if length % 2 != 0 {
                continue;
            }

            let half_len: u32 = length / 2;
            let first_half: u32 = (num as u32) / BASE_POW.checked_pow(half_len).unwrap();
            let second_half: u32 = (num as u32) % BASE_POW.checked_pow(half_len).unwrap();
            if first_half == second_half {
                sum += num as u64;
            }
        }
    }

    println!("{}", sum);
}
