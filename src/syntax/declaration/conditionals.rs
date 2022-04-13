use crate::{lexer::Token, syntax::Syntax};

pub fn analyze_if_statement(syntax: &mut Syntax) {
    syntax.expected(Token::SPACE);
    assert_condition(syntax);
    assert_body(syntax);
}

fn assert_condition(syntax: &mut Syntax) {
    loop {
        assert_value(syntax);
        assert_operator(syntax);
        assert_value(syntax);
        if !matches!(syntax.next_token(), Token::OPERATOR(op) if op.is_logical()) {
            syntax.back();
            break;
        }
    }
}

fn assert_value(syntax: &mut Syntax) {
    syntax.skip_next_token(Token::SPACE, true);
    match syntax.next_token() {
        Token::IDENT(_) => (),
        Token::IntValue(_) => (),
        Token::FloatValue(_) => (),
        _ => syntax.unexpected(),
    }
}

fn assert_operator(syntax: &mut Syntax) {
    syntax.skip_next_token(Token::SPACE, true);
    if !matches!(syntax.next_token(), Token::OPERATOR(op) if op.is_relational()) {
        syntax.unexpected();
    }
}


fn assert_body(syntax: &mut Syntax) {

}