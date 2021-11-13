#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[error]
    ERROR,
    EOF,
    #[token("()")]
    VOID,
    #[token("char")]
    CHAR,
    #[token("string")]
    String,
    #[regex("[a-zA-Z]+", |lexer| lexer.slice().to_owned())]
    IDENT(String),
    #[token("i8")]
    I8,
    #[token("i16")]
    I16,
    #[token("i32")]
    I32,
    #[token("i64")]
    I64,
    #[token("u8")]
    U8,
    #[token("u16")]
    U16,
    #[token("u32")]
    U32,
    #[token("u64")]
    U64,
    #[token("f32")]
    F32,
    #[token("f64")]
    F64,
    #[regex("-?[0-9]+", |lexer| lexer.slice().parse())]
    IntValue(i64),
    #[regex("-?[0-9]+\\.[0-9]+", |lex| lex.slice().parse())]
    FloatValue(f64),
    #[regex(r#""[^"]*""#, |lexer| lexer.slice()[1..(lexer.slice().len()-1)].to_owned())]
    STRING(String),
    #[token("=")]
    ASSIGN,
    #[token("+")]
    PLUS,
    #[token("-")]
    MINUS,
    #[token("/")]
    SLASH,
    #[token("*")]
    ASTERISK,
    #[token(".")]
    DOT,
    #[token("<")]
    LT,
    #[token(">")]
    GT,
    #[token("!")]
    BANG,
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
    #[token("class")]
    CLASS,
    #[token("struct")]
    STRUCT,
    #[token("public")]
    PUBLIC,
    #[token("private")]
    PRIVATE,
    #[token("extended")]
    EXTENDED,
    #[token("function")]
    FUNCTION,
    #[token("let")]
    VAR,
    #[token("if")]
    IF,
    #[token("else")]
    ELSE,
    #[token("return")]
    RETURN,
    #[token("true")]
    TRUE,
    #[token("false")]
    FALSE,
    #[token("==")]
    EQ,
    #[token("&&")]
    AND,
    #[token("||")]
    OR,
    #[token(" ")]
    SPACE,
    #[token("\n")]
    LINE_BREAK,
    #[token("!=")]
    NOT_EQ,
}


impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::ERROR => "ERROR".to_string(),
            Token::EOF => "EOF".to_string(),
            Token::VOID => "()".to_string(),
            Token::CHAR => "char".to_string(),
            Token::String => "string".to_string(),
            Token::IDENT(value) => value.to_string(),
            Token::I8 => "i8".to_string(),
            Token::I16 => "i16".to_string(),
            Token::I32 => "i32".to_string(),
            Token::I64 => "i64".to_string(),
            Token::U8 => "u8".to_string(),
            Token::U16 => "u16".to_string(),
            Token::U32 => "u32".to_string(),
            Token::U64 => "u64".to_string(),
            Token::F32 => "f32".to_string(),
            Token::F64 => "f64".to_string(),
            Token::IntValue(number) => number.to_string(),
            Token::FloatValue(float) => float.to_string(),
            Token::STRING(text) => text.to_string(),
            Token::ASSIGN => "=".to_string(),
            Token::PLUS => "+".to_string(),
            Token::MINUS => "-".to_string(),
            Token::SLASH => "/".to_string(),
            Token::ASTERISK => "*".to_string(),
            Token::DOT => ".".to_string(),
            Token::LT => "<".to_string(),
            Token::GT => ">".to_string(),
            Token::BANG => "!".to_string(),
            Token::COMMA => ",".to_string(),
            Token::COLON => ":".to_string(),
            Token::SEMICOLON => ";".to_string(),
            Token::LPAREN => "(".to_string(),
            Token::RPAREN => ")".to_string(),
            Token::LBRACE => "{".to_string(),
            Token::RBRACE => "}".to_string(),
            Token::PIPE => "|".to_string(),
            Token::CLASS => "class".to_string(),
            Token::STRUCT => "struct".to_string(),
            Token::PUBLIC => "public".to_string(),
            Token::PRIVATE => "private".to_string(),
            Token::EXTENDED => "extended".to_string(),
            Token::FUNCTION => "function".to_string(),
            Token::VAR => "var".to_string(),
            Token::IF => "if".to_string(),
            Token::ELSE => "else".to_string(),
            Token::RETURN => "return".to_string(),
            Token::TRUE => "true".to_string(),
            Token::FALSE => "false".to_string(),
            Token::EQ => "==".to_string(),
            Token::AND => "&&".to_string(),
            Token::OR => "||".to_string(),
            Token::SPACE => " ".to_string(),
            Token::LINE_BREAK => "\\n".to_string(),
            Token::NOT_EQ => "!=".to_string(),
        }
    }
}