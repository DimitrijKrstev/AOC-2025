use ::aoc2025::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("inputs/03-12-2025.txt");
    let mut sum: u32 = 0;

    for line in lines {
        let numbers: Vec<u32> = line
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|c: &char| c.to_digit(10).unwrap())
            .collect();

        let length: &usize = &numbers.len();
        let (max_index, _) = numbers[0..*length - 1]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, val)| val)
            .unwrap();

        let (offset_index_2, _) = numbers[max_index + 1..]
            .iter()
            .enumerate()
            .max_by_key(|&(_, val)| val)
            .unwrap();
        let max_index_2: usize = max_index + 1 + offset_index_2;

        sum += numbers[max_index] * 10 + numbers[max_index_2];
    }

    println!("{}", sum);
}
