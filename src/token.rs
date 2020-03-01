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
}
