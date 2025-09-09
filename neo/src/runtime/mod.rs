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
        }
    }
}
