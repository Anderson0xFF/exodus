use super::Syntax;
use crate::lexer::{Declaration};

mod function;
mod variable;
mod conditionals;

pub fn analyze(syntax: &mut Syntax, declaration: Declaration) {
    match declaration {
        Declaration::FUNCTION => function::analyze(syntax),
        Declaration::STRUCT => (),
    }
}
