use crate::token;
use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut new = Self {
            input,
            position: 0,
            read_position: 0,
            ch: char::default(),
        };
        new.read_char();

        new
    }

    pub fn next_token(&mut self) -> Token {
        use TokenType::*;

        self.skip_whitespace();
        let token = match self.ch {
            '=' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::new(Eq, "==".into())
                }
                _ => Token::new(Assign, "=".into()),
            },
            ';' => Token::new(Semicolon, ";".into()),
            ',' => Token::new(Comma, ",".into()),
            '(' => Token::new(LParen, "(".into()),
            ')' => Token::new(RParen, ")".into()),
            '{' => Token::new(LBrace, "{".into()),
            '}' => Token::new(RBrace, "}".into()),
            '+' => Token::new(Plus, "+".into()),
            '-' => Token::new(Minus, '-'.into()),
            '!' => match self.peek_char() {
                '=' => {
                    self.read_char();
                    Token::new(NotEq, "!=".into())
                }
                _ => Token::new(Bang, "!".into()),
            },
            '/' => Token::new(Slash, '/'.into()),
            '*' => Token::new(Asterisk, "*".into()),
            '<' => Token::new(Lt, '<'.into()),
            '>' => Token::new(Gt, '>'.into()),
            '\0' => Token::new(EOF, "".into()),
            c if is_alphabetic_ident(c) => {
                let literal = self.read_identifier();
                return Token::new(token::lookup_ident(&literal), literal);
            }
            c if c.is_ascii_digit() => {
                let literal = self.read_number();
                return Token::new(Int, literal);
            }
            c => Token::new(Illegal, c.to_string()),
        };

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            char::default()
        } else {
            self.input.as_bytes()[self.read_position] as char
        };

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_alphabetic_ident(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        self.input.chars().nth(self.read_position).unwrap_or('\0')
    }
}

fn is_alphabetic_ident(ident: char) -> bool {
    match ident {
        'A'..='Z' | 'a'..='z' | '_' => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::TokenType;
    use table_test::table_test;

    #[test]
    fn next_token() {
        let input = "=+(){},;";
        let tests = vec![
            (TokenType::Assign, "="),
            (TokenType::Plus, "+"),
            (TokenType::LParen, "("),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::RBrace, "}"),
            (TokenType::Comma, ","),
            (TokenType::Semicolon, ";"),
        ];
        let mut lexer = Lexer::new(input.clone().into());
        let mut iter = input.chars();

        for (validator, e_token, e_literal) in table_test!(tests) {
            let actual = lexer.next_token();

            validator
                .when("next_token")
                .given(&format!("{:?}", iter.next()))
                .then(&format!("it should be {}", e_token))
                .assert_eq(e_token, actual.kind)
                .assert_eq(e_literal, &actual.kind.to_string());
        }
    }

    #[test]
    fn next_token_advanced() {
        let input = r###"
let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
  return true;
} else {
  return false;
}

10 == 10;
10 != 9;
"###;
        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::Gt, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Int, "5"),
            (TokenType::Lt, "<"),
            (TokenType::Int, "10"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Int, "10"),
            (TokenType::Eq, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "10"),
            (TokenType::NotEq, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];
        let mut lexer = Lexer::new(input.clone().into());
        for (validator, e_token, e_literal) in table_test!(tests) {
            let current_char = lexer.ch;
            let actual = lexer.next_token();

            validator
                .when("next_token")
                .given(&format!("{:?}", current_char))
                .then(&format!("it should be {}", e_token))
                .assert_eq(e_token, actual.kind)
                .assert_eq(e_literal, &actual.literal);
        }
    }
}
