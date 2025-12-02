use aoc2025::read_lines;

fn main() {
    let lines: Vec<String> = read_lines("inputs/01-12-2025.txt");

    let mut pos: i64 = 50;
    let mut counter: i64 = 0;

    for line in lines {
        let chars: Vec<char> = line.chars().collect::<Vec<char>>();
        let amount: i64 = line[1..].parse().unwrap();
        pos = if chars[0] == 'R' {
            pos + amount
        } else {
            pos - amount
        };

        if pos > 99 {
            pos %= 100;
        } else if pos < 0 {
            pos = ((pos % 100) + 100) % 100;
        }
        if pos == 0 {
            counter += 1
        }
    }

    println!("{}", counter);
}
