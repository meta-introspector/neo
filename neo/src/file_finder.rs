// src/file_finder.rs
// This module is responsible for finding theory files, as per the refactoring in theory14.

use std::fs;
use std::path::PathBuf;

pub fn find_theory_files() -> Vec<PathBuf> {
    let mut theory_files = vec![];
    if let Ok(paths) = fs::read_dir("./") {
        for path in paths {
            if let Ok(path) = path {
                let path_buf = path.path();
                if path_buf.is_file() {
                    if let Some(filename) = path_buf.file_name().and_then(|s| s.to_str()) {
                        if filename.starts_with("theory") && filename.ends_with(".txt") {
                            theory_files.push(path_buf.clone());
                        }
                    }
                }
            }
        }
    }
    theory_files.sort();
    theory_files
}
