use std::collections::{HashMap, HashSet};

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
        let vars = match expr {
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
        };

        // NOTE: this is one of the worst things I've ever written
        vars.iter()
            .map(|x| *x)
            .collect::<HashSet<char>>()
            .iter()
            .map(|x| *x)
            .collect::<Vec<char>>()
    }

    pub fn compute_assignments(&self, variables: Vec<char>) -> Vec<Assignments> {
        // this is how many rows our truth table of sorts will have
        let rows = usize::pow(2, variables.len() as u32);

        (0..=rows)
            .map(|i| {
                // 'i' is a number which if we perform certain bitwise
                // operations on, will give us the truth values for each
                // variable
                let mut assignments = Assignments::default();

                let mut pos = 0;

                for var in &variables {
                    // we extract the bit on the nth position (where n is the
                    // position we're currently in, and turn it into a boolean
                    // by checking if it's 1
                    let value = (i >> pos) & 1 == 1;

                    assignments.insert(*var, value);

                    pos += 1
                }

                assignments
            })
            .collect::<Vec<Assignments>>()
    }

    pub fn check_tautology(&self, expr: ASTNode) -> anyhow::Result<bool> {
        let variables: Vec<char> = self.collect_variables(&expr);
        let assignments = self.compute_assignments(variables);

        for assignments in assignments {
            // NOTE: cloning -- bad
            if !self.eval(expr.clone(), &assignments)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn check_contradiction(&self, expr: ASTNode) -> anyhow::Result<bool> {
        let variables: Vec<char> = self.collect_variables(&expr);
        let assignments = self.compute_assignments(variables);

        for assignments in assignments {
            // NOTE: cloning -- bad
            if self.eval(expr.clone(), &assignments)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn check_contingency(&self, expr: ASTNode) -> anyhow::Result<bool> {
        Ok(!self.check_tautology(expr.clone())? && !self.check_contradiction(expr)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_variables() {
        let engine = Engine::default();

        let mut actual = engine.collect_variables(
            &engine
                .parse("(a) && (b && c || ((((((d) || e)))))) => f == g".to_string())
                .unwrap(),
        );
        actual.sort();

        let expected = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_tautology() {
        let engine = Engine::default();

        // a || !a is a tautology
        let expr = engine.parse("a || !a".to_string()).unwrap();
        assert!(engine.check_tautology(expr).unwrap());

        // (a => b) => ((!b) => (!a)) is a tautology (contrapositive)
        let expr = engine
            .parse("(a => b) => ((!b) => (!a))".to_string())
            .unwrap();
        assert!(engine.check_tautology(expr).unwrap());

        // a && !a is not a tautology
        let expr = engine.parse("a && !a".to_string()).unwrap();
        assert!(!engine.check_tautology(expr).unwrap());

        // a is not a tautology
        let expr = engine.parse("a".to_string()).unwrap();
        assert!(!engine.check_tautology(expr).unwrap());
    }

    #[test]
    fn test_contradiction() {
        let engine = Engine::default();

        // a && !a is a contradiction
        let expr = engine.parse("a && !a".to_string()).unwrap();
        assert!(engine.check_contradiction(expr).unwrap());

        // (a && b) && (!a || !b) is a contradiction
        let expr = engine.parse("(a && b) && (!a || !b)".to_string()).unwrap();
        assert!(engine.check_contradiction(expr).unwrap());

        // a || !a is not a contradiction
        let expr = engine.parse("a || !a".to_string()).unwrap();
        assert!(!engine.check_contradiction(expr).unwrap());

        // a is not a contradiction
        let expr = engine.parse("a".to_string()).unwrap();
        assert!(!engine.check_contradiction(expr).unwrap());
    }

    #[test]
    fn test_contingency() {
        let engine = Engine::default();

        // a is a contingency (neither tautology nor contradiction)
        let expr = engine.parse("a".to_string()).unwrap();
        assert!(engine.check_contingency(expr).unwrap());

        // a && b is a contingency
        let expr = engine.parse("a && b".to_string()).unwrap();
        assert!(engine.check_contingency(expr).unwrap());

        // a || b is a contingency
        let expr = engine.parse("a || b".to_string()).unwrap();
        assert!(engine.check_contingency(expr).unwrap());

        // a => b is a contingency
        let expr = engine.parse("a => b".to_string()).unwrap();
        assert!(engine.check_contingency(expr).unwrap());

        // a || !a is not a contingency (it's a tautology)
        let expr = engine.parse("a || !a".to_string()).unwrap();
        assert!(!engine.check_contingency(expr).unwrap());

        // a && !a is not a contingency (it's a contradiction)
        let expr = engine.parse("a && !a".to_string()).unwrap();
        assert!(!engine.check_contingency(expr).unwrap());
    }
}

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
