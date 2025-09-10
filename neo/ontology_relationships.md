# Ontology Relationships Matrix (Class-Level)

This document outlines the relationships between the classes defined in `neo_ontology.ttl`.

## Classes Involved:

*   `:Concept`
*   `:RustEntity`
*   `:Emoji`
*   `:Function`
*   `:EnumValue`
*   `:RunMode`
*   `:TokenType`
*   `:RustStruct`
*   `:RustEnum`
*   `:RustField`
*   `:RustMethod`
*   `:RustTrait`
*   `:File`
*   `:Directory`
*   `:Crate`
*   `:Parameter`
*   `:RustType`
*   `:LeanRepresentation`

## Relationships:

Below, we systematically explore potential and existing relationships between each pair of classes. '->' indicates a property from the first class to the second. 'Existing' refers to properties already defined in `neo_ontology.ttl`. 'Proposed' suggests new properties that could be added.

---

### :Concept to other Classes

*   **:Concept -> :Emoji**
    *   Existing: `:hasEmojiRepresentation`

*   **:Concept -> :RustEntity**
    *   Existing: `:hasRustRepresentation`

*   **:Concept -> :LeanRepresentation**
    *   Existing: `:hasLeanRepresentationString` (DatatypeProperty, but links to a string representing Lean concept)

*   **:Concept -> :Concept**
    *   Proposed: `:isSubConceptOf`, `:relatesToConcept` (more general relationship)

---

### :RustEntity to other Classes

*   **:RustEntity -> :Emoji**
    *   Existing: `:hasEmoji`

*   **:RustEntity -> :Function**
    *   Proposed: `:hasFunction` (if a RustEntity can contain a general function, e.g., a module)

*   **:RustEntity -> :RustStruct**
    *   Proposed: `:isStruct` (if a RustEntity can be specifically a struct)

*   **:RustEntity -> :RustEnum**
    *   Proposed: `:isEnum` (if a RustEntity can be specifically an enum)

*   **:RustEntity -> :RustMethod**
    *   Proposed: `:hasMethod` (if a RustEntity can contain a method)

*   **:RustEntity -> :RustType**
    *   Proposed: `:hasType` (if a RustEntity can be of a certain type)

*   **:RustEntity -> :RustTrait**
    *   Existing: `:implementsTrait` (Domain `owl:Thing`, so applies to RustEntity)

---

### :Emoji to other Classes

*   **:Emoji -> :Concept**
    *   Existing: `:hasConcept` (inverse of `:hasEmojiRepresentation` if defined)

*   **:Emoji -> :RustEntity**
    *   Proposed: `:representsRustEntity` (inverse of `:hasEmoji`)

---

### :Function to other Classes

*   **:Function -> :Parameter**
    *   Existing: `:hasParameter`

*   **:Function -> :RustType**
    *   Existing: `:hasReturnType`

*   **:Function -> :Function**
    *   Proposed: `:callsFunction`, `:isCalledBy`

---

### :EnumValue to other Classes

*   **:EnumValue -> :TokenType**
    *   Existing: `:isTokenType`

*   **:EnumValue -> :RunMode**
    *   Existing: `:isRunMode`

*   **:EnumValue -> :RustEnum**
    *   Proposed: `:isVariantOf` (inverse of `:hasVariant`)

---

### :RunMode to other Classes

*   **:RunMode -> :File**
    *   Proposed: `:processesFile` (if a RunMode operates on a file, e.g., Kantspell)

---

### :TokenType to other Classes

*   **:TokenType -> :RustType**
    *   Existing: `:hasType` (Domain `owl:Thing`, so applies to TokenType)

---

### :RustStruct to other Classes

*   **:RustStruct -> :RustField**
    *   Existing: `:hasField`

*   **:RustStruct -> :RustMethod**
    *   Existing: `:hasMethod`

*   **:RustStruct -> :RustTrait**
    *   Existing: `:implementsTrait`

---

### :RustEnum to other Classes

*   **:RustEnum -> :EnumValue**
    *   Existing: `:hasVariant`

*   **:RustEnum -> :RustMethod**
    *   Existing: `:hasMethod`

*   **:RustEnum -> :RustTrait**
    *   Existing: `:implementsTrait`

---

### :RustField to other Classes

*   **:RustField -> :RustType**
    *   Existing: `:hasType`

---

### :RustMethod to other Classes

*   **:RustMethod -> :Parameter**
    *   Existing: `:hasParameter`

*   **:RustMethod -> :RustType**
    *   Existing: `:hasReturnType`

*   **:RustMethod -> :RustStruct**
    *   Proposed: `:isMethodOfStruct` (inverse of `:hasMethod`)

*   **:RustMethod -> :RustEnum**
    *   Proposed: `:isMethodOfEnum` (inverse of `:hasMethod`)

---

### :RustTrait to other Classes

*   **:RustTrait -> :RustStruct**
    *   Proposed: `:isImplementedByStruct` (inverse of `:implementsTrait`)

*   **:RustTrait -> :RustEnum**
    *   Proposed: `:isImplementedByEnum` (inverse of `:implementsTrait`)

---

### :File to other Classes

*   **:File -> :Directory**
    *   Existing: `:isInDirectory`

---

### :Directory to other Classes

*   **:Directory -> :File**
    *   Proposed: `:containsFile` (inverse of `:isInDirectory`)

*   **:Directory -> :Directory**
    *   Proposed: `:containsDirectory`, `:isSubdirectoryOf`

---

### :Crate to other Classes

*   **:Crate -> :Crate**
    *   Existing: `:dependsOnCrate`
    *   Proposed: `:isDependencyOf` (inverse of `:dependsOnCrate`)

*   **:Crate -> :File**
    *   Proposed: `:containsFile` (if a crate can contain files directly)

---

### :Parameter to other Classes

*   **:Parameter -> :RustType**
    *   Existing: `:hasType`

*   **:Parameter -> :Function**
    *   Proposed: `:isParameterOf` (inverse of `:hasParameter`)

---

### :RustType to other Classes

*   **:RustType -> :RustField**
    *   Proposed: `:isTypeOfField` (inverse of `:hasType`)

*   **:RustType -> :Parameter**
    *   Proposed: `:isTypeOfParameter` (inverse of `:hasType`)

*   **:RustType -> :Function**
    *   Proposed: `:isReturnTypeOf` (inverse of `:hasReturnType`)

---

### :LeanRepresentation to other Classes

*   **:LeanRepresentation -> :Concept**
    *   Proposed: `:representsConcept` (inverse of `:hasLeanRepresentationString`)

---

This matrix provides a comprehensive overview of existing and proposed relationships between the core classes in our ontology. It highlights areas where new properties could enrich the semantic connections within the system.
