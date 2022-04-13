use crate::lexer::{Token, Statement};

use super::{variable, Syntax, conditionals};

/**
 Parses the function to verify that it complies with language standards. Some examples:
    ```
    //statement - identifier - args - return type
        func main(arg: i32, args: [i32]) -> bool {
            sun(args[0], args[1]);
            return true;
        }

        func sun(a: i32, b: i32) -> i32 {
            return a + b;
        }

    ```
*/
pub fn analyze(syntax: &mut Syntax) {
    syntax.expected(Token::SPACE);
    analyze_prototypes(syntax);
    syntax.skip_next_token(Token::SPACE, false);
    analyze_body(syntax);
}

fn analyze_prototypes(syntax: &mut Syntax) {
    syntax.assert_ident();
    syntax.skip_next_token(Token::SPACE, false);
    syntax.expected(Token::LPAREN);
    analyze_arguments(syntax);
    syntax.expected(Token::RPAREN);
}

fn analyze_arguments(syntax: &mut Syntax) {
    loop {
        match syntax.next_token() {
            Token::IDENT(_) => {
                syntax.expected(Token::COLON);
                syntax.skip_next_token(Token::SPACE, false);
                syntax.assert_type();
            }
            Token::COMMA => syntax.skip_next_token(Token::SPACE, false),
            Token::VOID => break,
            Token::RPAREN => {
                syntax.back();
                break;
            }
            _ => syntax.unexpected(),
        }
    }
}

fn analyze_body(syntax: &mut Syntax) {
    syntax.expected(Token::LBRACE);

    loop {
        match syntax.next_token() {
            Token::STATEMENT(statement) => match statement {
                Statement::LET => variable::analyze(syntax),
                Statement::IF => conditionals::analyze_if_statement(syntax),
                Statement::ELSE => todo!(),
                Statement::WHILE => todo!(),
                Statement::RETURN => todo!(),
                Statement::SELF => todo!(),
            },
            Token::SPACE => syntax.skip_next_token(Token::SPACE, false),
            Token::RBRACE => break,
            _ => syntax.unexpected(),
        }
    }
}
