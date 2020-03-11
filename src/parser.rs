use crate::token::Token;
use crate::lexer::Lexer;
use crate::ast::Program;

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

    pub fn parse_program(&self) -> Program {
        Program { statements: vec![] }
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::ast::Program;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn test_let_statement() {}

    struct LetTest {
        input: String,
        identifier: String,
        value: String,
    }

    #[test]
    fn test_let_statements() {
        let tests = [LetTest {
            input: String::from("let x = 5;"),
            identifier: String::from("x"),
            value: String::from("5"),
        }];
        for (i, t) in tests.iter().enumerate() {
            let l = Lexer::new(t.input.as_ref());
            let p = Parser::new(l);
            let program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            assert_eq!(program.statements[0].token_literal(), String::from("let"));
        }
    }
}
