use std::fs;

// Turn a file into a string or die.
pub fn file_to_str(filepath: &str) -> String {
    match fs::read_to_string(filepath) {
        Ok(lines) => { lines },
        Err(_) => { panic!("D1P1 Error: File not found: {}", filepath) }
    }
}
