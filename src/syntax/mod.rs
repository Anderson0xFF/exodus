mod declaration;

use crate::Token;
use std::process::abort;

pub struct Syntax {
    tokens: Vec<Token>,
    index: usize,
    last_token: Option<Token>,
    token: Option<Token>,
    line: usize,
    filename: String,
}

impl Syntax {
    pub fn new(filename: String, lexer: logos::Lexer<Token>) -> Syntax {
        let mut tokens = Vec::new();
        for token in lexer {
            tokens.push(token);
        }
        Syntax {
            tokens,
            index: 0,
            last_token: None,
            token: None,
            line: 1,
            filename,
        }
    }

    fn next_token(&mut self) -> Token {
        if self.index < self.tokens.len() {
            let token = self.tokens[self.index].clone();
            self.index += 1;

            if token.eq(&Token::LINE_BREAK) {
                self.line += 1;
                return self.next_token();
            }
            if !token.eq(&Token::SPACE) {
                self.last_token = Some(self.tokens[self.index - 1].clone());
            }
            self.token = Some(token.clone());
            return token;
        }
        return Token::EOF;
    }

    fn error_expected_token(&self, token: Token) {
        println!(
            "\n🔧 SyntaxError: expected '{}' but got '{}'.",
            token.to_string(),
            self.token.as_ref().unwrap().to_string()
        );
        println!("\n                      ^^^         ^^^");
        println!("Filename: [{}]  Line: [{}]", self.filename, self.line);
        abort();
    }

    fn unexpected(&self) {
        println!(
            "\n🔧SyntaxError: unexpected token '{}'.",
            self.token.as_ref().unwrap().to_string()
        );
        println!("\n                              ^^^");
        println!("Filename: [{}]  Line: [{}]", self.filename, self.line);
        abort();
    }

    fn expected(&mut self, tk: Token) {
        if !(self.next_token() == tk) {
            self.error_expected_token(tk)
        }
    }

    fn assert_ident(&mut self) {
        if !matches!(self.next_token(), Token::IDENT(_)) {
            println!(
                "\nSyntaxError: expected declaration of a name after '{}'",
                self.last_token.as_ref().unwrap().to_string()
            );
            println!("Filename: [{}]  Line: [{}]", self.filename, self.line);
            abort();
        }
    }

    fn assert_type(&mut self) {
        match self.next_token() {
            Token::IDENT(_) => (),
            Token::TYPE(_) => (),
            _ => {
                println!(
                    "SyntaxError: expected 'type' but got '{}'",
                    self.token.as_ref().unwrap().to_string()
                );
                println!("                                     ^^^");
                println!("Filename: [{}]  Line: [{}]", self.filename, self.line);
                abort();
            }
        }
    }

    fn back(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    fn skip_next_token(&mut self, token: Token, panic: bool) {
        if self.next_token().eq(&token) {
            if self.token.as_ref().unwrap().eq(&Token::SPACE) {
                self.skip_next_token(token, panic);
            }
            return;
        }
        if panic {
            self.error_expected_token(token)
        }
        self.back();
    }

    pub fn analyze(&mut self) {
        loop {
            match self.next_token() {
                Token::DECLARATION(declaration) => declaration::analyze(self, declaration),
                Token::IMPORT => (),
                Token::EOF => break,
                _ => self.unexpected(),
            }
        }
    }
}
