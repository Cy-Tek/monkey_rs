use crate::ast::{Identifier, LetStatement, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser {
    lexer: Lexer,

    cur_token: Option<Token>,
    peek_token: Option<Token>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: None,
            peek_token: None,
        };

        // We need to prime the current token and peek token
        p.read_token();
        p.read_token();

        p
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while !self.current_token_is(TokenType::EOF) {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement.into());
            }

            self.read_token()
        }

        program
    }

    fn read_token(&mut self) {
        self.cur_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse_statement(&mut self) -> Option<impl Statement> {
        match self.cur_token.as_ref()?.kind {
            TokenType::Let => self.parse_let_statement(),
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        let current = self.cur_token.take()?;

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let ident = self.cur_token.take()?;
        let name = Identifier::new(ident);

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }


        while !self.current_token_is(TokenType::EOF) {
            self.read_token();
        }

        Some(LetStatement {
            token: current,
            name,
            value: None,
        })
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.as_ref().is_some_and(|t| t.kind == token_type)
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.current_token_is(token_type) {
            self.read_token();
            return true;
        }

        false
    }
}

#[cfg(test)]
mod test {}
