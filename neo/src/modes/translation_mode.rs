// src/modes/translation_mode.rs
use crate::translator::Translator;
use crate::file_finder;
use std::fs;

pub fn run() {
    println!("--- Comprehensive Theory Translation Engine (Refactored) ---");
    let translator = Translator::new();
    let theory_files = file_finder::find_theory_files();
    println!("Found {} theory files to translate.", theory_files.len());
    for theory_file in theory_files {
        println!("\n--- Translation for: {} ---", theory_file.display());
        let content = fs::read_to_string(&theory_file).unwrap_or_else(|_| "".to_string());
        if content.is_empty() { continue; }
        let translations = translator.translate(&content);
        println!("  [Original]: {}", content.trim());
        println!("  [Emoji]:    {}", translations.emoji.trim());
        println!("  [Lean]:     {}", translations.lean.trim());
        println!("  [Rust]:     {}", translations.rust.trim());
    }
    println!("\n--- Translation Complete ---");
}
