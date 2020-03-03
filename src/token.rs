#[derive(Debug,PartialEq)]
pub enum Token {
    // Special purpose tokens
    Illegal,
    Eof,

    // Identifiers and literals
    Ident(String),
    Int(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,

    // Delimiters
    Comma,
    SemiColon,

    // Grouping
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return
}

pub fn lookup_ident(literal: String) -> Token {
   match literal.as_str() {
       "let" => Token::Let,
       "fn" => Token::Function,
       "true" => Token::True,
       "false" => Token::False,
       "if" => Token::If,
       "else" => Token::Else,
       "return" => Token::Return,
       _ => Token::Ident(literal)
   }
}
