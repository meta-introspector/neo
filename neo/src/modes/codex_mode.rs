// src/modes/codex_mode.rs
use crate::translator::Translator;
use crate::codex_generator;

pub fn run(filename: &str) {
    println!("--- Codex Generation Mode ---");
    println!("Generating codex table to {}", filename);
    let translator = Translator::new();
    codex_generator::generate_table(&translator.vocabulary, filename)
        .expect("Failed to generate codex table.");
    println!("Codex generation complete.");
}
