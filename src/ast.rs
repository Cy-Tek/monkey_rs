use std::{rc::Rc, any::Any};

use crate::token::Token;
use monkey_derive::{Expression, Statement};

pub trait Node {
    fn token_literal(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub trait Statement: Node {}
pub trait Expression: Node {}

impl<'a, T> From<T> for Box<dyn Node + 'a>
where
    T: Sized + Node + 'a,
{
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

impl<'a, T> From<T> for Box<dyn Expression + 'a>
where
    T: Sized + Expression + 'a,
{
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

impl<'a, T> From<T> for Box<dyn Statement + 'a>
where
    T: Sized + Statement + 'a,
{
    fn from(value: T) -> Self {
        Box::new(value)
    }
}


pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        self.statements
            .get(0)
            .map_or(String::new(), |statement| statement.token_literal())
    }

    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Statement, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Rc<dyn Expression>>,
}

#[derive(Expression, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: Token) -> Self {
        Self {
            value: token.literal.clone(),
            token,
        }
    }
}

#[cfg(test)]
mod let_statement;