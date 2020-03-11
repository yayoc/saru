use crate::token::Token;

pub enum Statement {
    LetStatement { token: Token, name: String, value: String }
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Statement {
    pub(crate) fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement {token, name, value } => token.string(),
            _ => String::from("")
        }
    }

    fn string(&self) -> String {
        match self {
            Statement::LetStatement { token, name, value } => {
               self.token_literal() + " " + name + " = "  + value + ";"
            },
            _ => String::from("")
        }
    }
}

