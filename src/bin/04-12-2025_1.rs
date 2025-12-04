use ::aoc2025::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("inputs/04-12-2025.txt");
    let mut matrix: Vec<Vec<char>> = lines
        .iter()
        .map(|line: &String| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.insert(0, '.');
            chars.push('.');
            chars
        })
        .collect();

    let filler_line: Vec<char> = (0..lines[0].len() + 2).map(|_| '.').collect();
    matrix.insert(0, filler_line.clone());
    matrix.push(filler_line.clone());

    let mut counter: i32 = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == '.' {
                continue;
            }
            let mut num_paper_rolls = 0;
            for row_index in i - 1..=i + 1 {
                for col_index in j - 1..=j + 1 {
                    if matrix[row_index][col_index] == '@' {
                        num_paper_rolls += 1;
                    }
                }
            }
            // include self
            if num_paper_rolls <= 4 {
                counter += 1;
            }
        }
    }

    println!("{}", counter)
}
