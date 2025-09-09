// src/args_parser/mod.rs
use std::env;
use std::path::PathBuf; // Added for PathBuf

pub enum RunMode {
    Codex(String),
    Translation,
    Kantspell(PathBuf), // Added Kantspell mode
}

pub fn parse_args() -> RunMode {
    let args: Vec<String> = env::args().collect();

    // Handle --emit-report-table
    if let Some(arg) = args.iter().find(|a| a.starts_with("--emit-report-table=")) {
        let filename = arg.split('=').nth(1).unwrap_or("table.md").to_string();
        return RunMode::Codex(filename);
    }

    // Handle --kantspell
    if let Some(arg_index) = args.iter().position(|a| a == "--kantspell") {
        if let Some(path_str) = args.get(arg_index + 1) {
            let path = PathBuf::from(path_str);
            return RunMode::Kantspell(path);
        } else {
            eprintln!("Error: --kantspell requires a path argument.");
            // Fallback or exit, for now, let's default to Translation
            return RunMode::Translation;
        }
    }

    RunMode::Translation
}