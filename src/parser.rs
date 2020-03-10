use crate::lexer::Lexer;
use crate::token::Token;

struct Parser<'a> {
    l: Lexer<'a>,
    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    fn new(l: Lexer<'a>) -> Self {
        let p = Parser {
            l,
            cur_token: None,
            peek_token: None,
        };
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.l.next_token());
    }
}
