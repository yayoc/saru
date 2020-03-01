use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            Some(ch) => match ch {
                '=' => Token::Assign,
                '+' => Token::Plus,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                ',' => Token::Comma,
                ';' => Token::SemiColon,
                _ => {
                    if ch.is_alphabetic() {
                        let literal = self.read_identifier();
                        if literal == String::from("let") {
                            return Token::Let;
                        } else if literal == String::from("fn") {
                            return Token::Function;
                        } else {
                            return Token::Ident(literal);
                        }
                    } else if ch.is_numeric() {
                        let literal = self.read_number();
                        return Token::Int(literal);
                    } else {
                        return Token::Illegal;
                    }
                }
            },
            None => Token::Illegal,
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
        let p = self.position;
        while self.ch.is_some() && self.ch.unwrap().is_alphabetic() {
            self.read_char();
        }
        let substring = &self.input.to_string()[p..self.position];
        String::from(substring)
    }

    fn read_number(&mut self) -> String {
        let p = self.position;
        while self.ch.is_some() && self.ch.unwrap().is_numeric() {
            self.read_char();
        }
        let substring = &self.input.to_string()[p..self.position];
        String::from(substring)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_some() && self.ch.unwrap().is_whitespace() {
            self.read_char()
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;\
        let ten = 10;\
        \
        let add = fn(x,y) {\
            x + y;\
        };\
        \
        let result = add(five, ten);";
        let mut lexer = Lexer::new(input);
        let expected = [
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::SemiColon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(String::from("10")),
            Token::SemiColon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::SemiColon,
            Token::RBrace,
            Token::SemiColon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::SemiColon,
        ];

        for e in expected.iter() {
            let t = &lexer.next_token();
            println!("expected {:?}, lexed {:?} ", e, t);
            assert_eq!(t, e);
        }
    }
}
