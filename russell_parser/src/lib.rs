use chumsky::prelude::*;
use russell_ast::ASTNode;

/// Parses an input into an [ASTNode]
pub fn parse(input: &str) -> anyhow::Result<ASTNode> {
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
        let and_expr = not_expr.clone().foldl(
            just("&&").padded().ignore_then(not_expr).repeated(),
            |left, right| ASTNode::And(Box::new(left), Box::new(right)),
        );

        // Or operator (left associative)
        let or_expr = and_expr.clone().foldl(
            just("||").padded().ignore_then(and_expr).repeated(),
            |left, right| ASTNode::Or(Box::new(left), Box::new(right)),
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
        let equiv_expr = implies_expr.clone().foldl(
            just("==").padded().ignore_then(implies_expr).repeated(),
            |left, right| ASTNode::Equivalent(Box::new(left), Box::new(right)),
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
            assert_eq!(parse(c).unwrap(), ASTNode::Variable(c));
        }
    }

    #[test]
    fn test_literals() {
        assert_eq!(parse("true").unwrap(), ASTNode::Literal(true));
        assert_eq!(parse("false").unwrap(), ASTNode::Literal(false));
    }

    #[test]
    fn test_paren() {
        assert_eq!(
            parse("(true)").unwrap(),
            ASTNode::Paren(Box::new(ASTNode::Literal(true)))
        );
        assert_eq!(
            parse("(false)").unwrap(),
            ASTNode::Paren(Box::new(ASTNode::Literal(false)))
        );

        assert_eq!(
            parse("(x)").unwrap(),
            ASTNode::Paren(Box::new(ASTNode::Variable('x')))
        );
        assert_eq!(
            parse("((x))").unwrap(),
            ASTNode::Paren(Box::new(ASTNode::Paren(Box::new(ASTNode::Variable('x')))))
        );
    }

    #[test]
    fn test_not() {
        assert_eq!(
            parse("!x").unwrap(),
            ASTNode::Not(Box::new(ASTNode::Variable('x')))
        );

        assert_eq!(
            parse("!!x").unwrap(),
            ASTNode::Not(Box::new(ASTNode::Not(Box::new(ASTNode::Variable('x')))))
        );
    }

    #[test]
    fn test_and() {
        assert_eq!(
            parse("x && y").unwrap(),
            ASTNode::And(
                Box::new(ASTNode::Variable('x')),
                Box::new(ASTNode::Variable('y'))
            )
        );

        assert_eq!(
            parse("x && (y && z)").unwrap(),
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
            parse("x || y").unwrap(),
            ASTNode::Or(
                Box::new(ASTNode::Variable('x')),
                Box::new(ASTNode::Variable('y'))
            )
        );

        assert_eq!(
            parse("x || (y || z)").unwrap(),
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
            parse("x => y").unwrap(),
            ASTNode::Implies(
                Box::new(ASTNode::Variable('x')),
                Box::new(ASTNode::Variable('y'))
            )
        );

        assert_eq!(
            parse("x => (y => z)").unwrap(),
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
    fn test_not_has_higher_precedence_than_and() {
        assert_eq!(
            parse("!p && q").unwrap(),
            ASTNode::And(
                Box::new(ASTNode::Not(Box::new(ASTNode::Variable('p')))),
                Box::new(ASTNode::Variable('q'))
            )
        );
    }

    #[test]
    fn test_and_has_higher_precedence_than_or() {
        assert_eq!(
            parse("p || q && r").unwrap(),
            ASTNode::Or(
                Box::new(ASTNode::Variable('p')),
                Box::new(ASTNode::And(
                    Box::new(ASTNode::Variable('q')),
                    Box::new(ASTNode::Variable('r'))
                ))
            )
        );
    }

    #[test]
    fn test_or_has_higher_precedence_than_implies() {
        assert_eq!(
            parse("p => q || r").unwrap(),
            ASTNode::Implies(
                Box::new(ASTNode::Variable('p')),
                Box::new(ASTNode::Or(
                    Box::new(ASTNode::Variable('q')),
                    Box::new(ASTNode::Variable('r'))
                ))
            )
        );
    }

    #[test]
    fn test_parentheses_override_precedence() {
        assert_eq!(
            parse("(p || q) && r").unwrap(),
            ASTNode::And(
                Box::new(ASTNode::Paren(Box::new(ASTNode::Or(
                    Box::new(ASTNode::Variable('p')),
                    Box::new(ASTNode::Variable('q'))
                )))),
                Box::new(ASTNode::Variable('r'))
            )
        );
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(
            parse("  p   &&   q  ").unwrap(),
            ASTNode::And(
                Box::new(ASTNode::Variable('p')),
                Box::new(ASTNode::Variable('q'))
            )
        );
    }

    #[test]
    fn test_equivalent() {
        assert_eq!(
            parse("x == y").unwrap(),
            ASTNode::Equivalent(
                Box::new(ASTNode::Variable('x')),
                Box::new(ASTNode::Variable('y'))
            )
        );

        assert_eq!(
            parse("x == (y == z)").unwrap(),
            ASTNode::Equivalent(
                Box::new(ASTNode::Variable('x')),
                Box::new(ASTNode::Paren(Box::new(ASTNode::Equivalent(
                    Box::new(ASTNode::Variable('y')),
                    Box::new(ASTNode::Variable('z'))
                ))))
            )
        );
    }

    #[test]
    fn test_implies_has_higher_precedence_than_equivalent() {
        assert_eq!(
            parse("p => q == r => s").unwrap(),
            ASTNode::Equivalent(
                Box::new(ASTNode::Implies(
                    Box::new(ASTNode::Variable('p')),
                    Box::new(ASTNode::Variable('q'))
                )),
                Box::new(ASTNode::Implies(
                    Box::new(ASTNode::Variable('r')),
                    Box::new(ASTNode::Variable('s'))
                ))
            )
        );
    }

    #[test]
    fn test_or_has_higher_precedence_than_equivalent() {
        assert_eq!(
            parse("p || q == r || s").unwrap(),
            ASTNode::Equivalent(
                Box::new(ASTNode::Or(
                    Box::new(ASTNode::Variable('p')),
                    Box::new(ASTNode::Variable('q'))
                )),
                Box::new(ASTNode::Or(
                    Box::new(ASTNode::Variable('r')),
                    Box::new(ASTNode::Variable('s'))
                ))
            )
        );
    }

    #[test]
    fn test_complex_equivalent_expression() {
        assert_eq!(
            parse("!(p => q) == p || !q").unwrap(),
            ASTNode::Equivalent(
                Box::new(ASTNode::Not(Box::new(ASTNode::Paren(Box::new(
                    ASTNode::Implies(
                        Box::new(ASTNode::Variable('p')),
                        Box::new(ASTNode::Variable('q'))
                    )
                ))))),
                Box::new(ASTNode::Or(
                    Box::new(ASTNode::Variable('p')),
                    Box::new(ASTNode::Not(Box::new(ASTNode::Variable('q'))))
                ))
            )
        );
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(
            parse("((x))").unwrap(),
            ASTNode::Paren(Box::new(ASTNode::Paren(Box::new(ASTNode::Variable('x')))))
        );
    }

    #[test]
    fn test_multiple_not_operators() {
        assert_eq!(
            parse("!!!x").unwrap(),
            ASTNode::Not(Box::new(ASTNode::Not(Box::new(ASTNode::Not(Box::new(
                ASTNode::Variable('x')
            ))))))
        );
    }

    #[test]
    fn test_parenthesized_equivalent() {
        assert_eq!(
            parse("(p == q)").unwrap(),
            ASTNode::Paren(Box::new(ASTNode::Equivalent(
                Box::new(ASTNode::Variable('p')),
                Box::new(ASTNode::Variable('q'))
            )))
        );
    }

    #[test]
    fn test_and_left_associativity() {
        assert_eq!(
            parse("p && q && r").unwrap(),
            ASTNode::And(
                Box::new(ASTNode::And(
                    Box::new(ASTNode::Variable('p')),
                    Box::new(ASTNode::Variable('q'))
                )),
                Box::new(ASTNode::Variable('r'))
            )
        );
    }

    #[test]
    fn test_or_left_associativity() {
        assert_eq!(
            parse("p || q || r").unwrap(),
            ASTNode::Or(
                Box::new(ASTNode::Or(
                    Box::new(ASTNode::Variable('p')),
                    Box::new(ASTNode::Variable('q'))
                )),
                Box::new(ASTNode::Variable('r'))
            )
        );
    }

    #[test]
    fn test_implies_right_associativity() {
        assert_eq!(
            parse("p => q => r").unwrap(),
            ASTNode::Implies(
                Box::new(ASTNode::Variable('p')),
                Box::new(ASTNode::Implies(
                    Box::new(ASTNode::Variable('q')),
                    Box::new(ASTNode::Variable('r'))
                ))
            )
        );
    }

    #[test]
    fn test_equivalent_left_associativity() {
        assert_eq!(
            parse("p == q == r").unwrap(),
            ASTNode::Equivalent(
                Box::new(ASTNode::Equivalent(
                    Box::new(ASTNode::Variable('p')),
                    Box::new(ASTNode::Variable('q'))
                )),
                Box::new(ASTNode::Variable('r'))
            )
        );
    }

    #[test]
    fn test_single_variable() {
        assert_eq!(parse("x").unwrap(), ASTNode::Variable('x'));
    }

    #[test]
    fn test_single_literal_true() {
        assert_eq!(parse("true").unwrap(), ASTNode::Literal(true));
    }

    #[test]
    fn test_single_literal_false() {
        assert_eq!(parse("false").unwrap(), ASTNode::Literal(false));
    }

    #[test]
    fn test_not_with_parentheses() {
        assert_eq!(
            parse("!(x)").unwrap(),
            ASTNode::Not(Box::new(ASTNode::Paren(Box::new(ASTNode::Variable('x')))))
        );
    }

    #[test]
    fn test_equivalent_with_literals() {
        assert_eq!(
            parse("true == false").unwrap(),
            ASTNode::Equivalent(
                Box::new(ASTNode::Literal(true)),
                Box::new(ASTNode::Literal(false))
            )
        );
    }

    #[test]
    fn test_mixed_operators_precedence() {
        assert_eq!(
            parse("!p && q || r => s == t").unwrap(),
            ASTNode::Equivalent(
                Box::new(ASTNode::Implies(
                    Box::new(ASTNode::Or(
                        Box::new(ASTNode::And(
                            Box::new(ASTNode::Not(Box::new(ASTNode::Variable('p')))),
                            Box::new(ASTNode::Variable('q'))
                        )),
                        Box::new(ASTNode::Variable('r'))
                    )),
                    Box::new(ASTNode::Variable('s'))
                )),
                Box::new(ASTNode::Variable('t'))
            )
        );
    }

    #[test]
    fn test_all_operators_with_parentheses() {
        assert_eq!(
            parse("(p && q) || (r => s) == (t)").unwrap(),
            ASTNode::Equivalent(
                Box::new(ASTNode::Or(
                    Box::new(ASTNode::Paren(Box::new(ASTNode::And(
                        Box::new(ASTNode::Variable('p')),
                        Box::new(ASTNode::Variable('q'))
                    )))),
                    Box::new(ASTNode::Paren(Box::new(ASTNode::Implies(
                        Box::new(ASTNode::Variable('r')),
                        Box::new(ASTNode::Variable('s'))
                    ))))
                )),
                Box::new(ASTNode::Paren(Box::new(ASTNode::Variable('t'))))
            )
        );
    }
}
