use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Token {
    pub kind: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token: TokenType, literal: String) -> Self {
        Self {
            literal,
            kind: token,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TokenType {
    Illegal,
    EOF,

    Ident,
    Int,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Illegal => write!(f, "ILLEGAL"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::Ident => write!(f, "IDENT"),
            TokenType::Int => write!(f, "INT"),
            TokenType::Assign => write!(f, "="),
            TokenType::Plus => write!(f, "+"),
            TokenType::Comma => write!(f, ","),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::LParen => write!(f, "("),
            TokenType::RParen => write!(f, ")"),
            TokenType::LBrace => write!(f, "{{"),
            TokenType::RBrace => write!(f, "}}"),
            TokenType::Function => write!(f, "FUNCTION"),
            TokenType::Let => write!(f, "LET"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Bang => write!(f, "!"),
            TokenType::Asterisk => write!(f, "*"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Lt => write!(f, "<"),
            TokenType::Gt => write!(f, ">"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::If => write!(f, "IF"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::Eq => write!(f, "=="),
            TokenType::NotEq => write!(f, "!="),
        }
    }
}

static KEYWORDS: Lazy<HashMap<String, TokenType>> = Lazy::new(|| {
    let mut hash = HashMap::new();
    hash.insert("fn".to_string(), TokenType::Function);
    hash.insert("let".to_string(), TokenType::Let);
    hash.insert("true".to_owned(), TokenType::True);
    hash.insert("false".to_owned(), TokenType::False);
    hash.insert("if".to_owned(), TokenType::If);
    hash.insert("else".to_owned(), TokenType::Else);
    hash.insert("return".to_owned(), TokenType::Return);
    hash
});

pub(crate) fn lookup_ident(ident: &str) -> TokenType {
    *KEYWORDS.get(ident).unwrap_or(&TokenType::Ident)
}
