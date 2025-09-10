// src/emoji_interpreter.rs
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation; // Added for grapheme iteration

// This struct will hold all our emoji to concept/Rust mappings
pub struct EmojiInterpreter {
    // Maps emoji string to a descriptive string (e.g., "theory", "rust", "Path")
    emoji_to_concept: HashMap<&'static str, String>,
    // Maps emoji string to a more specific Rust type/function name if applicable
    emoji_to_rust_mapping: HashMap<&'static str, String>,
}

impl EmojiInterpreter {
    pub fn new() -> Self {
        let mut emoji_to_concept = HashMap::new();
        let mut emoji_to_rust_mapping = HashMap::new();

        // Populate from codex.md (Term -> Emoji, so we reverse it)
        emoji_to_concept.insert("📜", "theory".to_string());
        emoji_to_concept.insert("✍️", "write".to_string());
        emoji_to_concept.insert("💾", "commit".to_string());
        emoji_to_concept.insert("🤫", "zkp".to_string()); // ZKP is 🤫️🔒, but often just 🤫
        emoji_to_concept.insert("🔒", "zkp_lock".to_string()); // Part of ZKP
        emoji_to_concept.insert("#️⃣", "hash".to_string()); // Hash is #️⃣
        emoji_to_concept.insert("🌳", "merkle_tree".to_string());
        emoji_to_concept.insert("🔢", "goedel_number".to_string());
        emoji_to_concept.insert("🌐", "distributed".to_string());
        emoji_to_concept.insert("📦", "module".to_string());
        emoji_to_concept.insert("🤝", "equivalence".to_string());
        emoji_to_concept.insert("🦀", "rust".to_string());
        emoji_to_concept.insert("🧐", "lean".to_string());
        emoji_to_concept.insert("😀", "emoji".to_string());
        emoji_to_concept.insert("🗺️", "plan".to_string());
        emoji_to_concept.insert("🕵️‍♂️", "audit".to_string());
        emoji_to_concept.insert("👣", "proof_path".to_string());

        // Populate from rust_emoji_map.md (Emoji -> Rust Type/Function)
        emoji_to_rust_mapping.insert("🛣️", "Path".to_string());
        emoji_to_rust_mapping.insert("📁", "PathBuf".to_string());
        emoji_to_rust_mapping.insert("✅", "Result_Success".to_string());
        emoji_to_rust_mapping.insert("❌", "Result_Failure".to_string());
        emoji_to_rust_mapping.insert("📦", "Box".to_string()); // Overlap with module, context matters
        emoji_to_rust_mapping.insert("💥", "Error".to_string());
        emoji_to_rust_mapping.insert("🚶‍♀️", "WalkDir_Walker".to_string());
        emoji_to_rust_mapping.insert("🌳", "WalkDir_Tree".to_string()); // Overlap with merkle_tree
        emoji_to_rust_mapping.insert("🧹", "Kantspell_Clean".to_string());
        emoji_to_rust_mapping.insert("✨", "Kantspell_Sparkle".to_string());
        emoji_to_rust_mapping.insert("📖", "File_Open_Book".to_string());
        emoji_to_rust_mapping.insert("🔓", "File_Open_Lock".to_string());
        emoji_to_rust_mapping.insert("🔄", "String_Replace".to_string());
        emoji_to_rust_mapping.insert("🗣️", "Command_New".to_string());
        emoji_to_rust_mapping.insert("➕", "Git_Add".to_string());
        emoji_to_rust_mapping.insert("🪙", "Token_Unit".to_string());
        emoji_to_rust_mapping.insert("✂️", "Lexer_Cut".to_string());
        emoji_to_rust_mapping.insert("💡", "Expression_Idea".to_string());
        emoji_to_rust_mapping.insert("📝", "TheoryStatement_Doc".to_string());
        emoji_to_rust_mapping.insert("🧠", "Parser_Brain".to_string());
        emoji_to_rust_mapping.insert("🧩", "Parser_Puzzle".to_string());
        emoji_to_rust_mapping.insert("➡️", "Flow_Arrow".to_string()); // General flow
        emoji_to_rust_mapping.insert("✖️", "Multiply_Times".to_string());
        emoji_to_rust_mapping.insert("🤖", "LLM_Robot".to_string());
        emoji_to_rust_mapping.insert("🧬", "Biosemiosis_DNA".to_string());
        emoji_to_rust_mapping.insert("🎲", "Game_Theory_Dice".to_string());
        emoji_to_rust_mapping.insert("💻", "Hackathon_Computer".to_string());
        emoji_to_rust_mapping.insert("🌱", "Genetic_Algorithm_Seed".to_string());
        emoji_to_rust_mapping.insert("🌌", "Godel_Universe".to_string());
        emoji_to_rust_mapping.insert("🔥", "Hott_Fire".to_string());
        emoji_to_rust_mapping.insert("💭", "Brainstorm_Thought".to_string());
        emoji_to_rust_mapping.insert("🔗", "Connected_Link".to_string());
        emoji_to_rust_mapping.insert("🚫", "Not_Forbidden".to_string());
        emoji_to_rust_mapping.insert("🏗️", "Machine_Construction".to_string());
        emoji_to_rust_mapping.insert("🚶‍♀️", "Walk_Person".to_string());
        emoji_to_rust_mapping.insert("🕰️", "Foucaults_Pendulum_Time".to_string());

        Self { emoji_to_concept, emoji_to_rust_mapping }
    }

    pub fn interpret_line(&self, line: &str) -> String {
        let mut interpretation = String::new();
        for grapheme in line.graphemes(true) {
            if let Some(concept) = self.emoji_to_concept.get(grapheme) {
                interpretation.push_str(&format!("{} ", concept));
            } else if let Some(rust_map) = self.emoji_to_rust_mapping.get(grapheme) {
                interpretation.push_str(&format!("{} ", rust_map));
            } else {
                interpretation.push_str(&format!("{} ", grapheme)); // Keep unrecognized graphemes
            }
        }
        interpretation.trim().to_string()
    }

    pub fn interpret_poem(&self, poem_content: &str) -> String {
        poem_content.lines()
            .map(|line| {
                if line.trim().is_empty() {
                    String::new()
                } else {
                    self.interpret_line(line)
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret_basic_concept_emojis() {
        let interpreter = EmojiInterpreter::new();
        assert_eq!(interpreter.interpret_line("📜✍️💾"), "theory write commit");
        assert_eq!(interpreter.interpret_line("🦀🧐😀"), "rust lean emoji");
    }

    #[test]
    fn test_interpret_basic_rust_emojis() {
        let interpreter = EmojiInterpreter::new();
        assert_eq!(interpreter.interpret_line("🛣️📁✅"), "Path PathBuf Result_Success");
        assert_eq!(interpreter.interpret_line("🧹✨📖"), "Kantspell_Clean Kantspell_Sparkle File_Open_Book");
    }

    #[test]
    fn test_interpret_mixed_emojis() {
        let interpreter = EmojiInterpreter::new();
        assert_eq!(interpreter.interpret_line("📜➡️🦀"), "theory Flow_Arrow rust");
        assert_eq!(interpreter.interpret_line("🧠🧩💡"), "Parser_Brain Parser_Puzzle Expression_Idea");
    }

    #[test]
    fn test_interpret_unrecognized_emojis() {
        let interpreter = EmojiInterpreter::new();
        assert_eq!(interpreter.interpret_line("🚀👽"), "🚀 👽"); // Should keep unrecognized
        assert_eq!(interpreter.interpret_line("📜🚀"), "theory 🚀");
    }

    #[test]
    fn test_interpret_empty_and_whitespace_lines() {
        let interpreter = EmojiInterpreter::new();
        assert_eq!(interpreter.interpret_line(""), "");
        assert_eq!(interpreter.interpret_line("   "), "");
    }

    #[test]
    fn test_interpret_complex_emoji_sequences() {
        let interpreter = EmojiInterpreter::new();
        // These are treated as individual graphemes by UnicodeSegmentation
        assert_eq!(interpreter.interpret_line("🤫🔒"), "zkp zkp_lock");
        assert_eq!(interpreter.interpret_line("#️⃣"), "hash");
    }

    #[test]
    fn test_interpret_poem() {
        let interpreter = EmojiInterpreter::new();
        let poem = "📦✨\n📜➡️🦀\n\n🧠🧩";
        let expected_interpretation = "module Kantspell_Sparkle\ntheory Flow_Arrow rust\n\nParser_Brain Parser_Puzzle";
        assert_eq!(interpreter.interpret_poem(poem), expected_interpretation);
    }
}
