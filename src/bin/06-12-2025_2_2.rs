use ::aoc2025::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("inputs/06-12-2025.txt");

    let operations: Vec<char> = lines.last().unwrap().chars().collect();
    let mut operation: char = *operations.iter().nth(0).unwrap();
    let mut result: u64 = if operation == '*' { 1 } else { 0 };
    let mut full_result: u64 = 0;

    for col_i in 0..operations.len() {
        let number: u64 = lines
            .iter()
            .take(lines.len() - 1)
            .filter_map(|line| line.chars().nth(col_i).and_then(|c| c.to_digit(10)))
            .fold(0, |acc, digit| acc * 10 + digit as u64);

        if number == 0 {
            operation = operations[col_i + 1];
            full_result += result;
            result = if operation == '*' { 1 } else { 0 };
            continue;
        }

        result = if operation == '*' {
            result * number
        } else {
            result + number
        }
    }

    println!("{}", full_result + result)
}
