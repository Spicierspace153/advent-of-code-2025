use std::fs;

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", path, e))
}