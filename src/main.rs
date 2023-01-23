#![feature(option_result_contains)]
#![feature(is_some_and)]

use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io::{stdin, BufRead};

mod ast;
mod lexer;
mod parser;
mod token;
mod utils;

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
