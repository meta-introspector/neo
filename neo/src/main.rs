use std::collections::HashMap;
use std::process::Command;

// From theory42: A vocabulary of concepts.
const THEORY42_CONCEPTS: &[&str] = &[
    "biosemiosis", "game theory", "hackathon", "genetic algorithm", "selfish gene", "alife",
    "autosemiotics", "autopoeisis", "semiotics", "cybernetics", "digital twin", "dualism",
    "godel", "tarski", "prime numbers", "p = np", "scheduling", "tsp", "market making",
    "bipartite graphs", "bott perioditiy", "monster group", "lattice", "algebra",
    "polynomial commitment", "proof path", "topology", "hott", "vladimir voevodsky", "ias",
    "princeton", "escher", "bach", "hofstadter", "kant", "heidegger", "pierce", "lie",
    "galious", "fixed point", "daofp", "cat theory", "isomorphism", "transformation",
    "list", "brainstorm", "self reflection", "introspector", "termite mound", "mind",
    "brain", "self", "ego", "id", "collective unconsiousness", "muses",
    "spectral decomposition", "fundamental topology of the mind",
    "partitioning of space with neurons", "quadrants", "2^2", "proof by induction",
    "oeis", "math", "logic", "philosophy", "wisdom", "knowledge", "kaballah", "kether",
    "foucaults pendulum",
];

// Represents a "theory" as a data structure.
#[derive(Debug, Clone)]
struct Theory {
    id: usize,
    name: String,
    description: String,
    dependencies: Vec<usize>,
    concepts: Vec<String>,
}

// The main struct for our meta-introspector.
struct MetaIntrospector {
    theories: HashMap<usize, Theory>,
}

impl MetaIntrospector {
    fn new() -> Self {
        let mut introspector = MetaIntrospector {
            theories: HashMap::new(),
        };
        introspector.load_theories();
        introspector
    }

    // `splice-paste-write`: Load the theories into memory.
    fn load_theories(&mut self) {
        // Theory 42: The foundational concepts.
        let theory42 = Theory {
            id: 42,
            name: "Theory 42".to_string(),
            description: "A foundational vocabulary of concepts.".to_string(),
            dependencies: vec![],
            concepts: THEORY42_CONCEPTS.iter().map(|s| s.to_string()).collect(),
        };
        self.theories.insert(42, theory42);

        // Theory 1: The self-rewriting principle.
        let theory1 = Theory {
            id: 1,
            name: "Theory 1".to_string(),
            description: "A theory that will rewrite itself with an LLM, 42 times.".to_string(),
            dependencies: vec![42],
            concepts: vec!["self-rewriting".to_string(), "llm".to_string()],
        };
        self.theories.insert(1, theory1);

        // Theory 2: The verifiability principle.
        let theory2 = Theory {
            id: 2,
            name: "Theory 2".to_string(),
            description: "A theory that compiles with a proof assistant (Lean4).".to_string(),
            dependencies: vec![1],
            concepts: vec!["proof".to_string(), "verification".to_string()],
        };
        self.theories.insert(2, theory2);
    }

    // `demonstrate-tape`: Show the relationships between theories.
    fn demonstrate(&self, theory_id: usize) {
        if let Some(theory) = self.theories.get(&theory_id) {
            println!("Demonstrating Theory {}: {}", theory.id, theory.name);
            println!("  Description: {}", theory.description);
            if !theory.dependencies.is_empty() {
                println!("  Dependencies:");
                for dep_id in &theory.dependencies {
                    if let Some(dep_theory) = self.theories.get(dep_id) {
                        println!("    - Theory {}: {}", dep_theory.id, dep_theory.name);
                    }
                }
            }
            if !theory.concepts.is_empty() {
                println!("  Concepts (first 5):");
                for concept in theory.concepts.iter().take(5) {
                    println!("    - {}", concept);
                }
            }
            println!();
        }
    }

    fn commit_changes(&self) {
        println!("--- Committing changes to git ---");

        let add_output = Command::new("git")
            .arg("add")
            .arg(".")
            .output()
            .expect("Failed to execute git add");

        if !add_output.status.success() {
            println!("Error staging changes:");
            println!("{}", String::from_utf8_lossy(&add_output.stderr));
            return;
        }

        let commit_message = "feat: Self-commit based on Theory 1";
        let commit_output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit_message)
            .output()
            .expect("Failed to execute git commit");

        if commit_output.status.success() {
            println!("Successfully committed changes with message: '{}'", commit_message);
        } else {
            println!("Error committing changes:");
            println!("{}", String::from_utf8_lossy(&commit_output.stderr));
        }
    }
}

// `execute`: The main entry point.
fn main() {
    // `apply, llm`: The LLM (me) is orchestrating this process.
    let mut introspector = MetaIntrospector::new();

    // `(splice-paste-write-biosemiosis-output-prove-execute-trace-demonstrate-tape theory [2,1,42])`
    // The sequence of operations is encoded in the structure of this program.
    // We will demonstrate the theories in the specified order.
    let demonstration_order = vec![2, 1, 42];

    println!("--- Meta-Introspector Execution Trace ---");
    println!("Executing demonstration based on Theory 3.");
    println!();

    for id in demonstration_order {
        introspector.demonstrate(id);
    }

    println!("--- End of Trace ---");
    println!();
    println!("Next step: This program will now commit itself to git.");
    
    introspector.commit_changes();
}

// `prove`: A testing module to verify the integrity of the theories.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theory_dependencies() {
        let introspector = MetaIntrospector::new();

        // Test Theory 2's dependency on Theory 1
        let theory2 = introspector.theories.get(&2).unwrap();
        assert!(theory2.dependencies.contains(&1));

        // Test Theory 1's dependency on Theory 42
        let theory1 = introspector.theories.get(&1).unwrap();
        assert!(theory1.dependencies.contains(&42));
    }

    #[test]
    fn test_theory_concepts() {
        let introspector = MetaIntrospector::new();
        let theory42 = introspector.theories.get(&42).unwrap();
        assert!(theory42.concepts.contains(&"biosemiosis".to_string()));
        assert_eq!(theory42.concepts.len(), THEORY42_CONCEPTS.len());
    }
}