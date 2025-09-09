// src/ast.rs
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    Call(String, Vec<Expression>), // Function name and arguments
}

#[derive(Debug, PartialEq, Clone)]
pub struct TheoryStatement {
    pub id: u32,
    pub concepts: Vec<Expression>,
}
