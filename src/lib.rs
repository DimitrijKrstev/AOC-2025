use std::fs;

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", filename))
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_input(filename)
        .lines()
        .map(|s| s.to_string())
        .collect()
}
