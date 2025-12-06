use ::aoc2025::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("inputs/06-12-2025.txt");
    let operations: Vec<char> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|op: &str| op.chars().next().unwrap())
        .collect();
    let mut previous_numbers: Vec<u64> = lines
        .first()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for line in &lines[1..lines.len() - 1] {
        let numbers: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for (i, operation) in operations.iter().enumerate() {
            previous_numbers[i] = if *operation == '+' {
                previous_numbers[i] + numbers[i]
            } else {
                previous_numbers[i] * numbers[i]
            }
        }
    }

    println!("{}", previous_numbers.iter().copied().sum::<u64>());
}
