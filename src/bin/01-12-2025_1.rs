use aoc2025::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("inputs/01-12-2025.txt");

    let mut pos: i64 = 50;
    let mut counter: i64 = 0;

    for line in lines {
        let chars: Vec<char> = line.chars().collect::<Vec<char>>();
        let amount: i64 = line[1..].parse().unwrap();

        if chars[0] == 'R' {
            pos += amount;
            counter += pos / 100;
            pos %= 100;
        } else {
            if pos == 0 {
                counter += amount / 100;
            } else if amount >= pos {
                counter += (amount - pos) / 100 + 1;
            }
            pos = ((pos - amount) % 100 + 100) % 100;
        }
    }

    println!("{}", counter);
}
