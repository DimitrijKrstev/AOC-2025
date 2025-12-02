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
            if length == 1 {
                continue;
            }
            let half_len: u32 = length.div_ceil(2);

            for i in 1..=half_len {
                let denominator: u32 = BASE_POW.checked_pow(i).unwrap();
                let first_digits: u32 = num % denominator;
                if first_digits.checked_ilog10().unwrap_or(0) + 1 != i {
                    continue;
                };
                let mut rest: u32 = num / denominator;
                while rest != 0 {
                    let last_digits: u32 = rest % denominator;
                    if first_digits != last_digits {
                        break;
                    }
                    rest /= denominator;
                }
                if rest == 0 {
                    sum += num as u64;
                    println!("{} ", num);
                    break;
                }
            }
        }
    }

    println!("{}", sum);
}
