# Standard Operating Procedure (SOP) for Theories

## Purpose
To ensure consistency and parsability of theory statements within the project.

## Format
Each theory statement should be a single line in the following format:

`theory<ID>=[<concept_or_function_call_1>, <concept_or_function_call_2>, ...]`

## Components

*   **`theory<ID>`**: A unique identifier for the theory, consisting of the literal "theory" followed by a positive integer (e.g., `theory1`, `theory21`).

*   **`=`**: An equals sign, separating the theory ID from its content.

*   **`[` and `]`**: Square brackets enclosing a comma-separated list of concepts or function calls.

*   **`<concept_or_function_call>`**:
    *   **Concept**: A single identifier (e.g., `program_memory`, `hott`).
    *   **Function Call**: An identifier representing a function, followed by parentheses `()` enclosing a comma-separated list of arguments. Arguments can be concepts, other function calls, or literals (e.g., `generate(vernacular_ontologies, from(program_memory))`, `onto(our_ontology, with(hott))`).

*   **Whitespace**: Minimal whitespace is preferred for clarity, but the parser should be robust to variations.
