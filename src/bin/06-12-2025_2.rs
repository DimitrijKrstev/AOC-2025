use ::aoc2025::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("inputs/06-12-2025.txt");
    // let col_length: usize = lines.len();
    let num_length: usize = lines
        .last()
        .unwrap()
        .chars()
        .skip(1)
        .take_while(|c| c.is_whitespace())
        .count();
    // let operations: Vec<char> = lines
    //     .last()
    //     .unwrap()
    //     .split_whitespace()
    //     .map(|op: &str| op.chars().next().unwrap())
    //     .collect();

    // let numbers: Vec<&str> = lines.first().unwrap().split(" ").collect();
    let numbers: Vec<String> = lines
        .first()
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .chunks(num_length)
        .map(|chunk: &[char]| chunk.into_iter().collect())
        .collect();

    // for number in numbers {
    //     for digit in number.chars() {}
    // }
    for (i, char) in lines.first().unwrap().chars().enumerate() {
        if i % num_length == 0 {
            continue;
        }
        let num: u32 = if char.is_digit(10) {
            char.to_digit(10).unwrap()
        } else {
            0u32
        };
    }

    println!("{:?}", numbers)

    // for (idx, line) in lines[1..lines.len() - 1].iter().enumerate() {
    //     for (op_i, operation) in operations.iter().enumerate() {}
    // }
}
