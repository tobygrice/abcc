use crate::abcc::error::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    IntConstant(String),
    Keyword(Keyword),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
    EndOfFile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    Int,
    Void,
    Return,
}

pub fn lex(src: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();

    Ok(tokens)
}
