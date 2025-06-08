use russell_ast::ASTNode;

/// Parses an input into an [ASTNode]
pub fn parse(input: String) -> anyhow::Result<ASTNode> {
    todo!()
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
                ))
            )
        );
    }
}
