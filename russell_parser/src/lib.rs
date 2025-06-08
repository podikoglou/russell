use russell_ast::ASTNode;
use chumsky::prelude::*;

/// Parses an input into an [ASTNode]
pub fn parse(input: String) -> anyhow::Result<ASTNode> {
    let parser = expr_parser();
    match parser.parse(input.trim()).into_result() {
        Ok(ast) => Ok(ast),
        Err(errors) => {
            let error_msg = errors
                .iter()
                .map(|e| format!("{:?}", e))
                .collect::<Vec<_>>()
                .join(", ");
            Err(anyhow::anyhow!("Parse error: {}", error_msg))
        }
    }
}

fn expr_parser<'a>() -> impl Parser<'a, &'a str, ASTNode, extra::Err<Rich<'a, char>>> {
    recursive(|expr| {
        // Variables: single lowercase letters
        let variable = one_of('a'..='z').map(ASTNode::Variable);

        // Literals: true and false
        let literal = just("true")
            .to(ASTNode::Literal(true))
            .or(just("false").to(ASTNode::Literal(false)));

        // Parenthesized expressions
        let parenthesized = expr
            .clone()
            .delimited_by(just('('), just(')'))
            .map(|inner| ASTNode::Paren(Box::new(inner)));

        // Atoms: literals, variables, or parenthesized expressions (order matters!)
        let atom = choice((literal, variable, parenthesized)).padded();

        // Not operator (highest precedence, prefix)
        let not_expr = just('!')
            .repeated()
            .foldr(atom, |_op, expr| ASTNode::Not(Box::new(expr)));

        // And operator (left associative)
        let and_expr = not_expr
            .clone()
            .foldl(
                just("&&").padded().ignore_then(not_expr).repeated(),
                |left, right| ASTNode::And(Box::new(left), Box::new(right))
            );

        // Or operator (left associative)
        let or_expr = and_expr
            .clone()
            .foldl(
                just("||").padded().ignore_then(and_expr).repeated(),
                |left, right| ASTNode::Or(Box::new(left), Box::new(right))
            );

        // Implies operator (right associative)
        let implies_expr = or_expr
            .clone()
            .separated_by(just("=>").padded())
            .at_least(1)
            .collect::<Vec<_>>()
            .map(|mut exprs| {
                // Right-fold manually for right associativity
                let mut result = exprs.pop().unwrap();
                while let Some(left) = exprs.pop() {
                    result = ASTNode::Implies(Box::new(left), Box::new(result));
                }
                result
            });

        // Equivalent operator (left associative, lowest precedence)
        let equiv_expr = implies_expr
            .clone()
            .foldl(
                just("==").padded().ignore_then(implies_expr).repeated(),
                |left, right| ASTNode::Equivalent(Box::new(left), Box::new(right))
            );

        equiv_expr
    })
    .then_ignore(end())
    .padded()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variables() {
        for c in 'a'..='z' {
            assert_eq!(parse(c.to_string()).unwrap(), ASTNode::Variable(c));
        }
    }

    #[test]
    fn test_literals() {
        assert_eq!(parse("true".to_string()).unwrap(), ASTNode::Literal(true));
        assert_eq!(parse("false".to_string()).unwrap(), ASTNode::Literal(false));
    }

    #[test]
    fn test_paren() {
        assert_eq!(
            parse("(true)".to_string()).unwrap(),
            ASTNode::Paren(Box::new(ASTNode::Literal(true)))
        );
        assert_eq!(
            parse("(false)".to_string()).unwrap(),
            ASTNode::Paren(Box::new(ASTNode::Literal(false)))
        );

        assert_eq!(
            parse("(x)".to_string()).unwrap(),
            ASTNode::Paren(Box::new(ASTNode::Variable('x')))
        );
        assert_eq!(
            parse("((x))".to_string()).unwrap(),
            ASTNode::Paren(Box::new(ASTNode::Paren(Box::new(ASTNode::Variable('x')))))
        );
    }

    #[test]
    fn test_not() {
        assert_eq!(
            parse("!x".to_string()).unwrap(),
            ASTNode::Not(Box::new(ASTNode::Variable('x')))
        );

        assert_eq!(
            parse("!!x".to_string()).unwrap(),
            ASTNode::Not(Box::new(ASTNode::Not(Box::new(ASTNode::Variable('x')))))
        );
    }

    #[test]
    fn test_and() {
        assert_eq!(
            parse("x && y".to_string()).unwrap(),
            ASTNode::And(
                Box::new(ASTNode::Variable('x')),
                Box::new(ASTNode::Variable('y'))
            )
        );

        assert_eq!(
            parse("x && (y && z)".to_string()).unwrap(),
            ASTNode::And(
                Box::new(ASTNode::Variable('x')),
                Box::new(ASTNode::Paren(Box::new(ASTNode::And(
                    Box::new(ASTNode::Variable('y')),
                    Box::new(ASTNode::Variable('z'))
                ))))
            )
        );
    }

    #[test]
    fn test_or() {
        assert_eq!(
            parse("x || y".to_string()).unwrap(),
            ASTNode::Or(
                Box::new(ASTNode::Variable('x')),
                Box::new(ASTNode::Variable('y'))
            )
        );

        assert_eq!(
            parse("x || (y || z)".to_string()).unwrap(),
            ASTNode::Or(
                Box::new(ASTNode::Variable('x')),
                Box::new(ASTNode::Paren(Box::new(ASTNode::Or(
                    Box::new(ASTNode::Variable('y')),
                    Box::new(ASTNode::Variable('z'))
                ))))
            )
        );
    }

    #[test]
    fn test_implies() {
        assert_eq!(
            parse("x => y".to_string()).unwrap(),
            ASTNode::Implies(
                Box::new(ASTNode::Variable('x')),
                Box::new(ASTNode::Variable('y'))
            )
        );

        assert_eq!(
            parse("x => (y => z)".to_string()).unwrap(),
            ASTNode::Implies(
                Box::new(ASTNode::Variable('x')),
                Box::new(ASTNode::Paren(Box::new(ASTNode::Implies(
                    Box::new(ASTNode::Variable('y')),
                    Box::new(ASTNode::Variable('z'))
                ))))
            )
        );
    }

    #[test]
    fn test_complex_expressions() {
        // Test operator precedence: ! has higher precedence than &&
        assert_eq!(
            parse("!p && q".to_string()).unwrap(),
            ASTNode::And(
                Box::new(ASTNode::Not(Box::new(ASTNode::Variable('p')))),
                Box::new(ASTNode::Variable('q'))
            )
        );

        // Test operator precedence: && has higher precedence than ||
        assert_eq!(
            parse("p || q && r".to_string()).unwrap(),
            ASTNode::Or(
                Box::new(ASTNode::Variable('p')),
                Box::new(ASTNode::And(
                    Box::new(ASTNode::Variable('q')),
                    Box::new(ASTNode::Variable('r'))
                ))
            )
        );

        // Test operator precedence: || has higher precedence than =>
        assert_eq!(
            parse("p => q || r".to_string()).unwrap(),
            ASTNode::Implies(
                Box::new(ASTNode::Variable('p')),
                Box::new(ASTNode::Or(
                    Box::new(ASTNode::Variable('q')),
                    Box::new(ASTNode::Variable('r'))
                ))
            )
        );

        // Test complex expression with parentheses
        assert_eq!(
            parse("(p && !q) || (!p && q)".to_string()).unwrap(),
            ASTNode::Or(
                Box::new(ASTNode::Paren(Box::new(ASTNode::And(
                    Box::new(ASTNode::Variable('p')),
                    Box::new(ASTNode::Not(Box::new(ASTNode::Variable('q'))))
                )))),
                Box::new(ASTNode::Paren(Box::new(ASTNode::And(
                    Box::new(ASTNode::Not(Box::new(ASTNode::Variable('p')))),
                    Box::new(ASTNode::Variable('q'))
                ))))
            )
        );

        // Test whitespace handling
        assert_eq!(
            parse("  p   &&   q  ".to_string()).unwrap(),
            ASTNode::And(
                Box::new(ASTNode::Variable('p')),
                Box::new(ASTNode::Variable('q'))
            )
        );
    }

    #[test]
    fn test_associativity() {
        // Test left associativity of &&
        assert_eq!(
            parse("p && q && r".to_string()).unwrap(),
            ASTNode::And(
                Box::new(ASTNode::And(
                    Box::new(ASTNode::Variable('p')),
                    Box::new(ASTNode::Variable('q'))
                )),
                Box::new(ASTNode::Variable('r'))
            )
        );

        // Test left associativity of ||
        assert_eq!(
            parse("p || q || r".to_string()).unwrap(),
            ASTNode::Or(
                Box::new(ASTNode::Or(
                    Box::new(ASTNode::Variable('p')),
                    Box::new(ASTNode::Variable('q'))
                )),
                Box::new(ASTNode::Variable('r'))
            )
        );

        // Test right associativity of =>
        assert_eq!(
            parse("p => q => r".to_string()).unwrap(),
            ASTNode::Implies(
                Box::new(ASTNode::Variable('p')),
                Box::new(ASTNode::Implies(
                    Box::new(ASTNode::Variable('q')),
                    Box::new(ASTNode::Variable('r'))
                ))
            )
        );
    }
}