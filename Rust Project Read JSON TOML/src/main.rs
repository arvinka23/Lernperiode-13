use std::env;
use std::fs;
use serde_json;
use toml;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }
    let file_path = &args[1];
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    if file_path.ends_with(".json") || content.trim_start().starts_with('{') {
        match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(value) => println!("{:#?}", value),
            Err(e) => eprintln!("Error parsing JSON: {}", e),
        }
    } else {
        match toml::from_str::<toml::Value>(&content) {
            Ok(value) => println!("{:#?}", value),
            Err(e) => eprintln!("Error parsing TOML: {}", e),
        }
    }
}
