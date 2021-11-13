use std::{fs::read_to_string, io::Error};

use crate::lexer::Token;
use logos::Logos;

mod lexer;



fn open_file(path: &str) -> Result<String, Error> {
    match read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let file = open_file("../main.exo").unwrap();

    let lexer = Token::lexer(&file);

    for token in  lexer {
        println!("{:?}", token);

    }
}
