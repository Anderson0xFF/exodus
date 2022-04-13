use crate::lexer::Token;
use logos::Logos;
use syntax::Syntax;

mod lexer;
mod syntax;
mod utils;

const FILENAME: &str = "main.exo";
fn main() {
    let file = utils::open_file(FILENAME).unwrap();
    let lexer = Token::lexer(&file);
    
    
   let mut syntax = Syntax::new(FILENAME.to_string(), lexer);
   syntax.analyze();
    
}
