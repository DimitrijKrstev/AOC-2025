use ::aoc2025::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("inputs/03-12-2025.txt");
    let mut sum: u64 = 0;

    for line in lines {
        let numbers: Vec<u32> = line
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|c: &char| c.to_digit(10).unwrap())
            .collect();

        let length: &usize = &numbers.len();
        let (max_index, _) = numbers[0..*length - 11]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, val)| val)
            .unwrap();

        let mut chosen_indexes: Vec<usize> = [max_index].to_vec();

        while chosen_indexes.len() < 12 {
            let last_index: &usize = chosen_indexes.last().unwrap();
            let (offset_index, _) = numbers
                [*last_index + 1..numbers.len() - (11 - chosen_indexes.len())]
                .iter()
                .enumerate()
                .rev()
                .max_by_key(|&(_, val)| val)
                .unwrap();

            chosen_indexes.push(last_index + offset_index + 1);
        }

        let num = chosen_indexes.iter().fold(0u64, |acc: u64, idx: &usize| {
            acc * 10 + numbers[*idx as usize] as u64
        });
        sum += num;
        println!("{} ", num);
    }

    println!("{}", sum);
}
