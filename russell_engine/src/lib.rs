use russell_ast::ASTNode;
use russell_parser::parse;

#[derive(Default, Debug)]
pub struct Engine {}

impl Engine {
    pub fn eval_str(&self, input: String) -> anyhow::Result<bool> {
        Ok(self.eval(parse(input)?))
    }

    pub fn eval(&self, expr: ASTNode) -> bool {
        true
    }
}
