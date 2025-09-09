// src/args_parser/mod.rs
use std::env;

pub enum RunMode {
    Codex(String),
    Translation,
}

pub fn parse_args() -> RunMode {
    let args: Vec<String> = env::args().collect();
    if let Some(arg) = args.iter().find(|a| a.starts_with("--emit-report-table=")) {
        let filename = arg.split('=').nth(1).unwrap_or("table.md").to_string();
        return RunMode::Codex(filename);
    }
    RunMode::Translation
}
