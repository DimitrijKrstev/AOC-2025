use std::collections::HashSet;

use ::aoc2025::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("inputs/07-12-2025.txt");
    let mut beam_indexes: HashSet<usize> = [lines.first().unwrap().find('S').unwrap()].into();

    let mut counter: u32 = 0;
    for line in lines.iter().skip(1) {
        let chars: Vec<char> = line.chars().collect();
        let mut new_beam_indexes: HashSet<usize> = HashSet::new();

        for beam in beam_indexes {
            if chars[beam] == '^' {
                counter += 1;
                if beam > 0 {
                    new_beam_indexes.insert(beam - 1);
                }
                if beam < chars.len() - 1 {
                    new_beam_indexes.insert(beam + 1);
                }
            } else {
                new_beam_indexes.insert(beam);
            }
        }
        beam_indexes = new_beam_indexes;
    }

    println!("{}", counter)
}
