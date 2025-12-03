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
        let mut max: u32 = 0;

        for i in 0..*length {
            for j in i + 1..*length {
                let num = numbers[i] * 10 + numbers[j];
                if num > max {
                    max = num;
                }
            }
        }

        sum += max;
    }

    println!("{}", sum);
}
