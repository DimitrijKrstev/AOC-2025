use ::aoc2025::read_lines;
use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = read_lines("inputs/07-12-2025.txt");
    let start_col: usize = lines.first().unwrap().find('S').unwrap();

    let mut beam_counts: HashMap<usize, usize> = HashMap::new();
    beam_counts.insert(start_col, 1);

    for line in lines.iter().skip(1) {
        let chars: Vec<char> = line.chars().collect();
        let mut new_beam_counts: HashMap<usize, usize> = HashMap::new();

        for (&pos, &count) in &beam_counts {
            if chars[pos] == '^' {
                if pos > 0 {
                    *new_beam_counts.entry(pos - 1).or_insert(0) += count;
                }
                if pos < chars.len() - 1 {
                    *new_beam_counts.entry(pos + 1).or_insert(0) += count;
                }
            } else {
                *new_beam_counts.entry(pos).or_insert(0) += count;
            }
        }

        beam_counts = new_beam_counts;
    }

    println!("{}", beam_counts.values().sum::<usize>());
}
