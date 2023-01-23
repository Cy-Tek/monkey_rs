#![feature(option_result_contains)]
#![feature(is_some_and)]

use std::io::{BufRead, stdin};
use crate::lexer::Lexer;
use crate::token::TokenType;

mod ast;
mod lexer;
mod token;
mod parser;

fn main() {
    let mut input = String::new();

    while let Ok(len) = stdin().lock().read_line(&mut input) {
        if len > 1 {
            let mut lexer = Lexer::new(input.clone());
            let mut token = lexer.next_token();

            while token.kind != TokenType::EOF {
                println!("{:#?}", token);
                token = lexer.next_token();
            }

            input.clear();
        } else {
            break;
        }
    }
}
