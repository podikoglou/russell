#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ASTNode {
    Variable(char),
    Literal(bool),

    Not(Box<ASTNode>),
    And(Box<ASTNode>, Box<ASTNode>),
    Or(Box<ASTNode>, Box<ASTNode>),
    Implies(Box<ASTNode>, Box<ASTNode>),
    Equivalent(Box<ASTNode>, Box<ASTNode>),

    Paren(Box<ASTNode>),
}
