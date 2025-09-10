// src/runtime/mod.rs
// The core theory runtime, as per the vision of theory17.

use crate::args_parser::{self, RunMode};
use crate::modes::{codex_mode, translation_mode};

pub struct Runtime;

impl Runtime {
    pub fn new() -> Self {
        Runtime
    }

    pub fn run(&self) {
        match args_parser::parse_args() {
            RunMode::Codex(filename) => codex_mode::run(&filename),
            RunMode::Translation => translation_mode::run(),
            RunMode::Kantspell(_) => {
                // This branch should ideally not be reached if main.rs handles it first.
                // Or, if Runtime is meant to be a central dispatcher, it would call kantspell::run_kantspell here.
                // For now, to resolve the non-exhaustive pattern error:
                todo!("Kantspell mode not handled by Runtime directly yet.");
            }
            RunMode::InterpretEmojiPoem(_) => { // Added InterpretEmojiPoem arm
                // Similar to Kantspell, main.rs handles this.
                todo!("InterpretEmojiPoem mode not handled by Runtime directly yet.");
            }
        }
    }
}
