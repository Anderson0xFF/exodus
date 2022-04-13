use std::collections::HashMap;

use super::{expr::Expr, types::Type};

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Class {
        name: String,
        atributes: HashMap<String, super::types::Type>,
        body: Vec<Expr>,
    },
    Function {
        name: String,
        parameters: HashMap<String, super::types::Type>,
        body: Vec<Expr>,
    },

    EXIT,
}
