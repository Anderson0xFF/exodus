use crate::{lexer::Token, syntax::Syntax};

/**
 When analyzing a variable, the system looks for the following corresponding gaps. Some examples:
    ```
    //type     identifier =  value;
      i32       number    =  10;
      string    text      =  "Hello, World!";
    ```
*/
pub fn analyze(syntax: &mut Syntax) {
    syntax.skip_next_token(Token::SPACE, false);
    syntax.assert_ident();
    syntax.skip_next_token(Token::SPACE, false);
    syntax.expected(Token::ASSIGN);
    syntax.skip_next_token(Token::SPACE, false);
    analyze_value(syntax);
    syntax.expected(Token::SEMICOLON);
}

fn analyze_value(syntax: &mut Syntax) {
    match syntax.next_token() {
        Token::IntValue(_) => (),
        Token::FloatValue(_) => (),
        Token::STRING(_) => (),
        Token::IDENT(_) => (),
        _ => syntax.unexpected(),
    }
}
