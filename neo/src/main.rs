// src/main.rs
// main.rs: The minimal kernel for the theory runtime.

mod translator;
mod file_finder;
mod codex_generator;
mod args_parser;
mod modes;
mod types;
mod runtime;
mod kantspell; // Added kantspell module

// Modules for the parser (kept for potential future use, but not directly called in main)
mod lexer;
mod ast;
mod parser;

fn main() {
    let run_mode = args_parser::parse_args();

    match run_mode {
        args_parser::RunMode::Codex(filename) => {
            modes::codex_mode::run(&filename);
        }
        args_parser::RunMode::Translation => {
            let runtime = runtime::Runtime::new();
            runtime.run();
        }
        args_parser::RunMode::Kantspell(path) => {
            if let Err(e) = kantspell::run_kantspell(&path) {
                eprintln!("Kantspell error: {}", e);
            }
        }
    }
}
