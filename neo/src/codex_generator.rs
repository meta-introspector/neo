// src/codex_generator.rs
// Generates a markdown table codex of the vocabulary, as per theory15.

use std::collections::HashMap;
use std::fs;

pub fn generate_table(vocabulary: &HashMap<&'static str, &'static str>, filename: &str) -> std::io::Result<()> {
    let mut table = String::new();
    table.push_str("# Meta-Introspector Codex\n\n");
    table.push_str("This table defines the vocabulary used in the theory language and its translations.\n\n");
    table.push_str("| Term | Emoji | Lean Representation | Rust Representation |\n");
    table.push_str("|---|---|---|---|
");

    let mut sorted_vocab: Vec<_> = vocabulary.iter().collect();
    sorted_vocab.sort();

    for (term, emoji) in sorted_vocab {
        let lean_rep = format!("T_{}", term.to_uppercase());
        let rust_rep = format!("Concept::{}", term.to_uppercase());
        table.push_str(&format!("| `{}` | {} | `{}` | `{}` |\n", term, emoji, lean_rep, rust_rep));
    }

    fs::write(filename, table)
}

