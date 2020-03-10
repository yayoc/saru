use crate::token::Token;

trait Node {
    fn token_literal() -> String;
    fn string() -> String;
}

struct Statement {}

struct Expression {}

impl Expression {
    fn expression_node() {}
}

struct PrefixExpression {
    token: Token,
    operator: String,
    right: Expression,
}

struct Program {
    statements: Vec<Statement>,
}

struct Identifier {
    token: Token,
    value: String,
}

struct LetStatement {
    token: Token,
    name: Identifier,
    value: Expression,
}
