pub trait TokenLiteral {
    fn token_literal(&self) -> String;
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl TokenLiteral for Program {
    fn token_literal(&self) -> String {
        self.statements
            .get(0)
            .map_or(String::new(), |statement| statement.token_literal())
    }
}

pub enum Statement {
    Let,
}

impl TokenLiteral for Statement {
    fn token_literal(&self) -> String {
        String::new()
    }
}

pub enum Expression {}
