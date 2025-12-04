use std::fs;

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e))
}


pub fn read_file_to_list(path: &str) -> Vec<String> {
    let input = read_file(path);
    input
        .lines()
        .map(|line| line.to_string())
        .collect()
}