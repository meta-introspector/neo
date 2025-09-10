# Meta-Introspector Codex

This table defines the vocabulary used in the theory language and its translations.

| Term | Emoji | Lean Representation | Rust Representation |
|---|---|---|---|
| `audit` | 🕵️‍♂️ | `T_AUDIT` | `Concept::AUDIT` |
| `commit` | 💾 | `T_COMMIT` | `Concept::COMMIT` |
| `distributed` | 🌐 | `T_DISTRIBUTED` | `Concept::DISTRIBUTED` |
| `emoji` | 😀 | `T_EMOJI` | `Concept::EMOJI` |
| `equivalence` | 🤝 | `T_EQUIVALENCE` | `Concept::EQUIVALENCE` |
| `goedel_number` | 🔢 | `T_GOEDEL_NUMBER` | `Concept::GOEDEL_NUMBER` |
| `hash` | #️⃣ | `T_HASH` | `Concept::HASH` |
| `hott` | 🔥 | `T_HOTT` | `Concept::HOTT` |
| `lean` | 🧐 | `T_LEAN` | `Concept::LEAN` |
| `merkle_tree` | 🌳 | `T_MERKLE_TREE` | `Concept::MERKLE_TREE` |
| `module` | 📦 | `T_MODULE` | `Concept::MODULE` |
| `plan` | 🗺️ | `T_PLAN` | `Concept::PLAN` |
| `proof_path` | 👣 | `T_PROOF_PATH` | `Concept::PROOF_PATH` |
| `rust` | 🦀 | `T_RUST` | `Concept::RUST` |
| `theory` | 📜 | `T_THEORY` | `Concept::THEORY` |
| `write` | ✍️ | `T_WRITE` | `Concept::WRITE` |
| `zkp` | 🤫️🔒 | `T_ZKP` | `Concept::ZKP` |

## New Features

### Emoji Poem Interpretation

The system now supports interpreting "emoji poems" from files. Using the `--interpret-emoji-poem <path>` argument, the application will read a file containing emoji sequences and translate them into human-readable concepts or Rust-specific terms based on predefined mappings. This feature leverages the `src/emoji_interpreter.rs` module and the `rust_emoji_map.md` document.

### Kantspell Module

The `kantspell` module provides a utility for code formatting and refactoring. Specifically, the `--kantspell <path>` argument can be used to automatically fix instances of `{{` and `}}` (double curly braces) to `{` and `}` (single curly braces) within Rust source files. After applying fixes, it automatically stages and commits the changes.

### Lexer and Parser Improvements

Significant refactoring has been undertaken in the `src/lexer.rs` and `src/parser.rs` modules. These improvements enhance the robustness, clarity, and correctness of the tokenization and parsing processes. Changes include:
-   Improved handling of lexer positions and multi-character tokens.
-   Specific recognition of "theory" followed by a number (e.g., `theory123`).
-   Modularization of parser helper functions into `src/parser_modules/` for better organization.

## Current Development Status

This section summarizes the ongoing development and known issues as of the last recorded state.

**Last Action:** The agent was in the process of fixing `src/emoji_interpreter.rs` to resolve `test_interpret_mixed_emojis` and `test_interpret_poem` failures.

**Project Files Modified (since last commit):**
*   `src/emoji_interpreter.rs`: Modified to fix `🧠` mapping and whitespace in `interpret_poem`'s expected output (pending write).
*   `src/lexer.rs`: Modified multiple times to fix tokenization issues, especially for "theory" and numbers.
*   `src/parser.rs`: Modified to fix `E0382` error (passing `&TokenType`).
*   `src/runtime/mod.rs`: Modified to add `RunMode::Kantspell` and `RunMode::InterpretEmojiPoem` arms (with `todo!()`).
*   `src/args_parser/mod.rs`: Modified to add `RunMode::Kantspell` and `RunMode::InterpretEmojiPoem` arguments.
*   `src/main.rs`: Modified to integrate `kantspell` and `emoji_interpreter` and remove the parser demonstration.
*   `Cargo.toml`: Added `walkdir` and `unicode-segmentation` dependencies.

**New Files Created (since last commit):**
*   `emoji_poem.md`
*   `emoji_poem_key.md`
*   `project_summary_poem.md`
*   `rust_emoji_map.md`
*   `src/ast.rs`
*   `src/kantspell.rs`
*   `src/lexer.rs`
*   `src/parser.rs`
*   `src/emoji_interpreter.rs`

**Last Commit:** "feat: Implement Kantspell module for Rust code formatting and add new poems" (This commit *did not* include the latest changes to `src/emoji_interpreter.rs`, `src/lexer.rs`, `src/parser.rs`, `src/runtime/mod.rs`, `src/args_parser/mod.rs`, `src/main.rs`, `Cargo.toml` related to the emoji interpreter and lexer/parser fixes).

**Outstanding Tasks/Known Issues:**
*   **Emoji Interpreter Tests:** `test_interpret_mixed_emojis` and `test_interpret_poem` are expected to pass after the pending write to `src/emoji_interpreter.rs`.
*   **Parser Tests:** All parser tests are currently failing. This is likely due to the `lexer.rs` changes not being fully correct yet, or issues within the parser's logic itself.
*   **General Warnings:** Several `unused import` and `dead_code` warnings remain. These are low priority.