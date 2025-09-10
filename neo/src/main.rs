// src/main.rs
// main.rs: The minimal kernel for the theory runtime.

mod translator;
mod file_finder;
mod codex_generator;
mod args_parser;
mod modes;
mod types;
mod runtime;
mod kantspell;
mod emoji_interpreter; // Added emoji_interpreter module

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
        args_parser::RunMode::InterpretEmojiPoem(path) => {
            use std::fs;
            if let Ok(poem_content) = fs::read_to_string(&path) {
                let interpreter = emoji_interpreter::EmojiInterpreter::new();
                let interpreted_text = interpreter.interpret_poem(&poem_content);
                println!("\n--- Interpreted Emoji Poem ---");
                println!("{}", interpreted_text);
                println!("------------------------------");
            } else {
                eprintln!("Error: Could not read emoji poem file: {}", path.display());
            }
        }
    }
}