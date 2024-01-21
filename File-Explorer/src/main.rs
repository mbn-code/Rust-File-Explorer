extern crate walkdir;
extern crate regex;


use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;
use walkdir::WalkDir;

fn search_by_name(directory: &str, file_name: &str) {
    let file_name_regex = Regex::new(file_name).expect("Invalid regex");

    for entry in WalkDir::new(directory) {
        let entry = entry.expect("Error reading directory entry");
        let file_path = entry.path();

        if let Some(file_name) = file_path.file_name() {
            if file_name_regex.is_match(file_name.to_str().unwrap_or("")) {
                println!("{}", file_path.display());
            }
        }
    }
}

fn search_by_content(directory: &str, content: &str) {
    for entry in WalkDir::new(directory) {
        let entry = entry.expect("Error reading directory entry");
        let file_path = entry.path();

        if file_path.is_file() {
            if let Ok(file) = File::open(file_path) {
                let reader = io::BufReader::new(file);

                for line in reader.lines() {
                    if let Ok(line) = line {
                        if line.contains(content) {
                            println!("{}", file_path.display());
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <directory> <search_term> [options: -n for name, -c for content]", args[0]);
        return;
    }

    let directory = &args[1];
    let search_term = &args[2];

    if args.len() == 3 {
        // Default search by name
        search_by_name(directory, search_term);
    } else {
        let option = &args[3];
        match option.as_str() {
            "-n" => search_by_name(directory, search_term),
            "-c" => search_by_content(directory, search_term),
            _ => println!("Invalid option: {}", option),
        }
    }
}
