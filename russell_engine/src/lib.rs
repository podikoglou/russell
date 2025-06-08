use std::collections::HashMap;

use anyhow::anyhow;
use russell_ast::ASTNode;
use russell_parser::parse;

type Assignments = HashMap<char, bool>;

#[derive(Default, Debug)]
pub struct Engine {}

impl Engine {
    pub fn eval_str(&self, input: String, assignments: &Assignments) -> anyhow::Result<bool> {
        self.eval(parse(input)?, assignments)
    }

    pub fn eval(&self, expr: ASTNode, assignments: &Assignments) -> anyhow::Result<bool> {
        match expr {
            ASTNode::Variable(symbol) => match assignments.get(&symbol) {
                Some(val) => Ok(*val),
                None => Err(anyhow!("")),
            },

            ASTNode::Literal(value) => Ok(value),

            ASTNode::Not(node) => Ok(!self.eval(*node, assignments)?),

            ASTNode::And(p, q) => Ok(self.eval(*p, assignments)? && self.eval(*q, assignments)?),

            ASTNode::Or(p, q) => Ok(self.eval(*p, assignments)? && self.eval(*q, assignments)?),

            ASTNode::Implies(p, q) => {
                Ok(!self.eval(*p, assignments)? || self.eval(*q, assignments)?)
            }

            ASTNode::Paren(inner) => self.eval(*inner, assignments),
        }
    }
}
