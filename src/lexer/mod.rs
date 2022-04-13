#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
use logos::{Logos, Lexer};

#[derive(Debug, PartialEq, Clone)]
pub enum Type{
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Boolean,
    Char,
    String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Relational{
    LT,
    GT,
    EQ,
    NOT_EQ
}


impl ToString for Relational {
    fn to_string(&self) -> String{
        match self{
            Relational::LT => "+".to_string(),
            Relational::GT => "-".to_string(),
            Relational::EQ => "/".to_string(),
            Relational::NOT_EQ => "*".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Arithmetic{
    ADD, 
    SUB,
    MUL,
    DIV,
    MOD,
}

impl ToString for Arithmetic {
    fn to_string(&self) -> String{
        match self{
            Arithmetic::ADD => "+".to_string(),
            Arithmetic::SUB => "-".to_string(),
            Arithmetic::DIV => "/".to_string(),
            Arithmetic::MUL => "*".to_string(),
            Arithmetic::MOD => "%".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Logical{
    XOR,
    AND, 
    OR,
    NOT,
}


impl ToString for Logical {
    fn to_string(&self) -> String{
        match self{
            Logical::XOR => "^".to_string(),
            Logical::AND => "&&".to_string(),
            Logical::OR => "||".to_string(),
            Logical::NOT => "!".to_string(),
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    ARITHMETIC(Arithmetic),
    RELATIONAL(Relational),
    LOGICAL(Logical),
    //REF,
}

impl Operator {
    pub fn is_relational(&self) -> bool {
        match self {
            Operator::RELATIONAL(_) => true,
            _=> false
        }
    }

    pub fn is_logical(&self) -> bool {
        match self {
            Operator::LOGICAL(_) => true,
            _=> false
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement{
    LET,
    IF,
    ELSE,
    WHILE,
    RETURN,
    SELF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration{
    FUNCTION,
    STRUCT,
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[error]
    ERROR,
    EOF,
    #[token("()")]
    VOID,
    #[regex("[a-zA-Z]+", |lexer| lexer.slice().to_owned())]
    IDENT(String),
    #[regex("(i8|i16|i32|i64|u8|u16|u32|u64|f32|f64|string|char|bool)", Token::as_type)]
    TYPE(Type),
    #[regex("-?[0-9]+", |lexer| lexer.slice().parse())]
    IntValue(i64),
    #[regex("-?[0-9]+\\.[0-9]+", |lexer| lexer.slice().parse())]
    FloatValue(f64),
    #[regex(r#""[^"]*""#, |lexer| lexer.slice()[1..(lexer.slice().len()-1)].to_owned())]
    STRING(String),
    #[token("=")]
    ASSIGN,
    #[regex("\\^|\\+|\\-|/|%|\\&|<|>|\\&&|==|!=|\\*|\\|\\||!", Token::as_operator)]
    OPERATOR(Operator),
    #[token(".")]
    DOT,
    #[token(",")]
    COMMA,
    #[token(":")]
    COLON,
    #[token(";")]
    SEMICOLON,
    #[token("(")]
    LPAREN,
    #[token(")")]
    RPAREN,
    #[token("{")]
    LBRACE,
    #[token("}")]
    RBRACE,
    #[token("|")]
    PIPE,
    #[regex("function|struct", Token::as_declaration)]
    DECLARATION(Declaration),
    #[regex("if|else|let|while|return|self", Token::as_statement)]
    STATEMENT(Statement),
    IMPORT,
    #[token("public")]
    PUBLIC,
    #[token("private")]
    PRIVATE,
    #[token("extended")]
    EXTENDED,
    #[token("true")]
    TRUE,
    #[token("false")]
    FALSE,
    #[token(" ")]
    SPACE,
    #[token("\n")]
    LINE_BREAK,
}

impl Token {
    pub fn as_type(lex: &mut Lexer<Token>) -> Option<Type> {
        match lex.slice() {
            "i8" => Some(Type::I8),
            "i16" => Some(Type::I16),
            "i32" => Some(Type::I32),
            "i64" => Some(Type::I64),
            "u8" => Some(Type::U8),
            "u16" => Some(Type::U16),
            "u32" => Some(Type::U32),
            "u64" => Some(Type::U64),
            "f32" => Some(Type::F32),
            "f64" => Some(Type::F64),
            "char" => Some(Type::Char),
            "string" => Some(Type::String),
            "bool" => Some(Type::Boolean),
            _=> None
        }
    }

    pub fn as_declaration(lex: &mut Lexer<Token>) -> Option<Declaration> {
        match lex.slice(){
            "struct" => Some(Declaration::STRUCT),
            "function" => Some(Declaration::FUNCTION),
            _=> None,
        }
    }

    pub fn as_statement(lex: &mut Lexer<Token>) -> Option<Statement> {
        match lex.slice(){
            "if" => Some(Statement::IF),
            "else" => Some(Statement::ELSE),
            "let" => Some(Statement::LET),
            "while" => Some(Statement::WHILE),
            "self" => Some(Statement::SELF),
            "return" => Some(Statement::RETURN),
            _=> None,
        }
    }

    pub fn as_operator(lex: &mut Lexer<Token>) -> Option<Operator>{
        match lex.slice() {
            "+" => Some(Operator::ARITHMETIC(Arithmetic::ADD)),
            "-" => Some(Operator::ARITHMETIC(Arithmetic::SUB)),
            "*" => Some(Operator::ARITHMETIC(Arithmetic::MUL)),
            "/" => Some(Operator::ARITHMETIC(Arithmetic::DIV)),
            "%" => Some(Operator::ARITHMETIC(Arithmetic::MOD)),
           // "&" => Some(Operator::ARITHMETIC(REF)),
            "<" => Some(Operator::RELATIONAL(Relational::LT)),
            ">" => Some(Operator::RELATIONAL(Relational::GT)),
            "==" => Some(Operator::RELATIONAL(Relational::EQ)),
            "!=" => Some(Operator::RELATIONAL(Relational::NOT_EQ)),
            //"^" => Some(Operator::ARITHMETIC(XOR)),
            "&&" => Some(Operator::LOGICAL(Logical::AND)),
            "||" => Some(Operator::LOGICAL(Logical::OR)),
            "!" => Some(Operator::LOGICAL(Logical::NOT)),
            _=> {
                println!("Invalid operator {}", lex.slice());
                None
            }
        }
    }
}

impl ToString for Declaration {
    fn to_string(&self) -> String{
        match self {
            Declaration::STRUCT => "struct".to_string(),
            Declaration::FUNCTION => "function".to_string(),
        }
    }
}

impl ToString for Statement {
    fn to_string(&self) -> String{
        match self {
            Statement::IF => "if".to_string(),
            Statement::ELSE => "else".to_string(),
            Statement::LET => "let".to_string(),
            Statement::WHILE => "while".to_string(),
            Statement::SELF => "self".to_string(),
            Statement::RETURN => "return".to_string(),
        }
    }
}

impl ToString for Operator {
    fn to_string(&self) -> String{
        match self {
            Operator::ARITHMETIC(op) => op.to_string(),
            Operator::LOGICAL(op) => op.to_string(),
            Operator::RELATIONAL(op) => op.to_string(),
        }
    }
}

impl ToString for Type {
    fn to_string(&self) -> String  {
        match self {
            Type::I8 => "i8".to_string(),
            Type::I16 => "i16".to_string(),
            Type::I32 => "i32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::U8 => "u8".to_string(),
            Type::U16 => "u16".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U64 => "u64".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::Char => "char".to_string(),
            Type::String => "string".to_string(),
            Type::Boolean => "bool".to_string(),
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::ERROR => "ERROR".to_string(),
            Token::EOF => "EOF".to_string(),
            Token::VOID => "()".to_string(),
            Token::IDENT(value) => value.to_string(),
            Token::TYPE(typ) => typ.to_string(),
            Token::IntValue(number) => number.to_string(),
            Token::FloatValue(float) => float.to_string(),
            Token::STRING(text) => text.to_string(),
            Token::DECLARATION(declaration) => declaration.to_string(),
            Token::STATEMENT(statement) => statement.to_string(),
            Token::ASSIGN => "=".to_string(),
            Token::DOT => ".".to_string(),
            Token::COMMA => ",".to_string(),
            Token::COLON => ":".to_string(),
            Token::SEMICOLON => ";".to_string(),
            Token::LPAREN => "(".to_string(),
            Token::RPAREN => ")".to_string(),
            Token::LBRACE => "{".to_string(),
            Token::RBRACE => "}".to_string(),
            Token::PIPE => "|".to_string(),
            Token::PUBLIC => "public".to_string(),
            Token::PRIVATE => "private".to_string(),
            Token::EXTENDED => "extended".to_string(),
            Token::TRUE => "true".to_string(),
            Token::FALSE => "false".to_string(),
            Token::SPACE => " ".to_string(),
            Token::LINE_BREAK => "\\n".to_string(),
            Token::OPERATOR(operator) => operator.to_string(),
            Token::IMPORT => todo!(),
        }
    }
}