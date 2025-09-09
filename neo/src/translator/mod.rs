// src/translator/mod.rs
use std::collections::HashMap;
use crate::types::Translations;

pub struct Translator {
    pub vocabulary: HashMap<&'static str, &'static str>,
}

impl Translator {
    pub fn new() -> Self {
        let mut vocabulary = HashMap::new();
        vocabulary.insert("theory", "📜");
        vocabulary.insert("write", "✍️");
        vocabulary.insert("commit", "💾");
        vocabulary.insert("zkp", "🤫️🔒");
        vocabulary.insert("hash", "#️⃣");
        vocabulary.insert("merkle_tree", "🌳");
        vocabulary.insert("goedel_number", "🔢");
        vocabulary.insert("distributed", "🌐");
        vocabulary.insert("module", "📦");
        vocabulary.insert("equivalence", "🤝");
        vocabulary.insert("rust", "🦀");
        vocabulary.insert("lean", "🧐");
        vocabulary.insert("emoji", "😀");
        vocabulary.insert("plan", "🗺️");
        vocabulary.insert("audit", "🕵️‍♂️");
        Translator { vocabulary }
    }

    pub fn translate(&self, theory_string: &str) -> Translations {
        let mut emoji_res = theory_string.to_string();
        let mut lean_res = theory_string.to_string();
        let mut rust_res = theory_string.to_string();

        for (key, val) in &self.vocabulary {
            emoji_res = emoji_res.replace(key, val);
            lean_res = lean_res.replace(key, &format!("T_{}", key.to_uppercase()));
            rust_res = rust_res.replace(key, &format!("Concept::{}", key.to_uppercase()));
        }

        Translations {
            emoji: emoji_res,
            lean: lean_res,
            rust: rust_res,
        }
    }
}
