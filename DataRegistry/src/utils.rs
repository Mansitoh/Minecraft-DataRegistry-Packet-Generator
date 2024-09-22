use std::io::{self, Write};
use std::fs;
pub fn generate_input() -> String {
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn read_json_files_from_path(path: &str) -> Vec<String> {
    let mut json_files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        json_files.push(content);
                    }
                }
            }
        }
    }
    json_files
}