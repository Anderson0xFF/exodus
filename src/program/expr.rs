use super::types::Type;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    VOID,
    Ident(String),
    Var {
        name: String,
        type_of: Type,
        value: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        consequence: Vec<Expr>,
        alternative: Option<Vec<Expr>>,
    },
    String(String),
    Boolean(bool),
    Unary {
        prefix: Option<Prefix>,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    Reassignment {
        name: String,
        value: Box<Expr>,
    },
    Call {
        function: String,
        arguments: Vec<Expr>,
    },
    Return(Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Prefix {
    NOT,  // !
    POINTER, // *
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    GreaterThan,
    LessThan,
    Equals,
    NotEquals,
}
