// src/kantspell.rs
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const OLD_OPEN: &str = "{{";
const OLD_CLOSE: &str = "}}";
const NEW_OPEN: &str = "{";
const NEW_CLOSE: &str = "}";

pub fn run_kantspell(target_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("--- Kantspell: Analyzing {} ---", target_path.display());

    let mut files_to_modify: Vec<PathBuf> = Vec::new();
    let mut report_lines: Vec<String> = Vec::new();

    // Phase 1: Report
    for entry in WalkDir::new(target_path) {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            let mut content = String::new();
            fs::File::open(path)?.read_to_string(&mut content)?;

            let mut found_issues = false;
            for (line_num, line) in content.lines().enumerate() {
                if line.contains(OLD_OPEN) || line.contains(OLD_CLOSE) {
                    report_lines.push(format!("{}:{}: {}", path.display(), line_num + 1, line));
                    found_issues = true;
                }
            }

            if found_issues {
                files_to_modify.push(path.to_path_buf());
            }
        }
    }

    if report_lines.is_empty() {
        println!("No `{{` or `}}` issues found in .rs files.");
        return Ok(());
    }

    println!("\n--- Kantspell: Found Issues ---");
    for line in &report_lines {
        println!("{}", line);
    }
    println!("-------------------------------\n");

    // Phase 2: Fix
    println!("--- Kantspell: Applying Fixes ---");
    let mut changes_applied_count = 0;
    for path in &files_to_modify {
        let mut content = String::new();
        fs::File::open(path)?.read_to_string(&mut content)?;

        let original_content = content.clone();

        content = content.replace(OLD_OPEN, NEW_OPEN);
        content = content.replace(OLD_CLOSE, NEW_CLOSE);

        if content != original_content {
            fs::File::create(path)?.write_all(content.as_bytes())?;
            println!("Fixed: {}", path.display());
            changes_applied_count += 1;
        }
    }
    println!("Applied fixes to {} files.", changes_applied_count);

    // Phase 3: Commit
    if changes_applied_count > 0 {
        println!("\n--- Kantspell: Committing Changes ---");
        // Add modified files to staging area
        let output_add = std::process::Command::new("git")
            .arg("add")
            .args(&files_to_modify)
            .output()?;

        if !output_add.status.success() {
            eprintln!("Failed to git add files:");
            eprintln!("{}", String::from_utf8_lossy(&output_add.stderr));
            return Err("Git add failed".into());
        }

        // Commit changes
        let commit_message = "Kantspell: Fixed `{{` or `}}` to `{}` in Rust files.";
        let output_commit = std::process::Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit_message)
            .output()?;

        if !output_commit.status.success() {
            eprintln!("Failed to git commit changes:");
            eprintln!("{}", String::from_utf8_lossy(&output_commit.stderr));
            return Err("Git commit failed".into());
        }

        println!("Successfully committed changes.");
    } else {
        println!("\nNo changes applied, skipping commit.");
    }

    println!("\n--- Kantspell: Complete ---");
    Ok(())
}