use crate::lexer::Token;
use logos::Logos;

mod lexer;
mod utils;

fn main() {
    let file = utils::open_file("../main.exo").unwrap();
    let lexer = Token::lexer(&file);

    for token in lexer {
        println!("{:?}", token);
    }
}
