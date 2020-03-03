pub mod lexer;
pub mod token;

use lexer::Lexer;
use std::io::{self, BufRead, Write};
use token::Token;

fn main() {
    let stdin = io::stdin();
    print!(">> ");
    io::stdout().flush().expect("Error flushing stdout");

    let mut line = String::new();
    stdin
        .lock()
        .read_line(&mut line)
        .expect("Error reading from stdin");
    let mut lexer = Lexer::new(&line);

    loop {
        let tok = lexer.next_token();
        println!("{:?}", tok);
        if tok == Token::Eof {
            break;
        }
    }
}
