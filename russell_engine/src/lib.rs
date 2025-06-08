use std::collections::HashMap;

use anyhow::anyhow;
use russell_ast::ASTNode;
use russell_parser::parse;

type Assignments = HashMap<char, bool>;

#[derive(Default, Debug)]
pub struct Engine {}

impl Engine {
    pub fn parse(&self, input: String) -> anyhow::Result<ASTNode> {
        parse(input)
    }

    pub fn eval_str(&self, input: String, assignments: &Assignments) -> anyhow::Result<bool> {
        self.eval(self.parse(input)?, assignments)
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
            ASTNode::Or(p, q) => Ok(self.eval(*p, assignments)? || self.eval(*q, assignments)?),
            ASTNode::Implies(p, q) => {
                Ok(!self.eval(*p, assignments)? || self.eval(*q, assignments)?)
            }
            ASTNode::Equivalent(p, q) => {
                Ok(self.eval(*p, assignments)? == self.eval(*q, assignments)?)
            }
            ASTNode::Paren(inner) => self.eval(*inner, assignments),
        }
    }

    pub fn collect_variables(&self, expr: &ASTNode) -> Vec<char> {
        match expr {
            ASTNode::Variable(symbol) => vec![*symbol],

            ASTNode::Literal(_) => vec![],

            ASTNode::Not(node) => self.collect_variables(node),

            ASTNode::And(p, q) => [self.collect_variables(p), self.collect_variables(q)].concat(),

            ASTNode::Or(p, q) => [self.collect_variables(p), self.collect_variables(q)].concat(),

            ASTNode::Implies(p, q) => {
                [self.collect_variables(p), self.collect_variables(q)].concat()
            }

            ASTNode::Equivalent(p, q) => {
                [self.collect_variables(p), self.collect_variables(q)].concat()
            }

            ASTNode::Paren(node) => self.collect_variables(node),
        }
    }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_variables() {
        let engine = Engine::default();

        assert_eq!(
            engine.collect_variables(
                &engine
                    .parse("(a) && (b && c || ((((((d) || e)))))) => f == g".to_string())
                    .unwrap()
            ),
            vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']
        );
    }
}
