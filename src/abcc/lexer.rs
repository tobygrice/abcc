use crate::abcc::error::{Error, Result};
use std::collections::HashMap;

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
    Lexer::new(src).lex()
}

struct Lexer<'src> {
    src: &'src str,
    pos: usize,
    line: u32,
    column: u32,
    keywords: HashMap<&'static str, Keyword>,
    single_byte_tokens: HashMap<u8, Token>,
}

impl<'src> Lexer<'src> {
    fn new(src: &'src str) -> Self {
        Self {
            src,
            pos: 0,
            line: 1,
            column: 1,
            keywords: HashMap::from([
                ("int", Keyword::Int),
                ("return", Keyword::Return),
                ("void", Keyword::Void),
            ]),
            single_byte_tokens: HashMap::from([
                (b'(', Token::OpenParen),
                (b')', Token::CloseParen),
                (b'{', Token::OpenBrace),
                (b'}', Token::CloseBrace),
                (b';', Token::Semicolon),
            ]),
        }
    }

    fn lex(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while let Some(byte) = self.peek_byte() {
            if byte.is_ascii_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if byte.is_ascii_alphabetic() || byte == b'_' {
                tokens.push(self.lex_identifier_or_keyword());
                continue;
            }

            if byte.is_ascii_digit() {
                tokens.push(self.lex_constant()?);
                continue;
            }

            if let Some(token) = self.single_byte_tokens.get(&byte).cloned() {
                self.advance_byte();
                tokens.push(token);
            } else {
                return Err(self.invalid_token());
            }
        }

        tokens.push(Token::EndOfFile);

        Ok(tokens)
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let start = self.pos;

        self.consume_word_bytes();

        let identifier = &self.src[start..self.pos];
        self.keywords
            .get(identifier)
            .copied()
            .map(Token::Keyword)
            .unwrap_or_else(|| Token::Identifier(identifier.to_owned()))
    }

    fn lex_constant(&mut self) -> Result<Token> {
        let start = self.pos;
        let line = self.line;
        let column = self.column;

        while let Some(byte) = self.peek_byte() {
            if !byte.is_ascii_digit() {
                break;
            }

            self.advance_byte();
        }

        match self.peek_byte() {
            Some(byte) if is_word_byte(byte) => {
                self.consume_word_bytes();
                Err(Error::InvalidToken {
                    token: self.src[start..self.pos].to_owned(),
                    line,
                    column,
                })
            }
            _ => Ok(Token::IntConstant(self.src[start..self.pos].to_owned())),
        }
    }

    fn consume_word_bytes(&mut self) {
        while let Some(byte) = self.peek_byte() {
            if !is_word_byte(byte) {
                break;
            }

            self.advance_byte();
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(byte) = self.peek_byte() {
            match byte {
                b'\n' => {
                    self.pos += 1;
                    self.line += 1;
                    self.column = 1;
                }
                b'\r' => {
                    self.pos += 1;
                    if self.peek_byte() == Some(b'\n') {
                        self.pos += 1;
                    }
                    self.line += 1;
                    self.column = 1;
                }
                byte if byte.is_ascii_whitespace() => self.advance_byte(),
                _ => break,
            }
        }
    }

    fn invalid_token(&self) -> Error {
        Error::InvalidToken {
            token: self
                .src
                .get(self.pos..)
                .and_then(|rest| rest.chars().next())
                .map(|ch| ch.to_string())
                .unwrap_or_default(),
            line: self.line,
            column: self.column,
        }
    }

    fn advance_byte(&mut self) {
        self.pos += 1;
        self.column += 1;
    }

    fn peek_byte(&self) -> Option<u8> {
        self.src.as_bytes().get(self.pos).copied()
    }
}

fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_basic_function() {
        assert_eq!(
            lex("int main(void) {\n    return 2;\n}").unwrap(),
            vec![
                Token::Keyword(Keyword::Int),
                Token::Identifier("main".to_owned()),
                Token::OpenParen,
                Token::Keyword(Keyword::Void),
                Token::CloseParen,
                Token::OpenBrace,
                Token::Keyword(Keyword::Return),
                Token::IntConstant("2".to_owned()),
                Token::Semicolon,
                Token::CloseBrace,
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn treats_keyword_prefixes_as_identifiers() {
        assert_eq!(
            lex("integer voided return_value _x x1").unwrap(),
            vec![
                Token::Identifier("integer".to_owned()),
                Token::Identifier("voided".to_owned()),
                Token::Identifier("return_value".to_owned()),
                Token::Identifier("_x".to_owned()),
                Token::Identifier("x1".to_owned()),
                Token::EndOfFile,
            ]
        );
    }

    #[test]
    fn rejects_constants_without_word_boundary() {
        match lex("123abc").unwrap_err() {
            Error::InvalidToken {
                token,
                line,
                column,
            } => {
                assert_eq!(token, "123abc");
                assert_eq!(line, 1);
                assert_eq!(column, 1);
            }
            error => panic!("unexpected error: {error}"),
        }
    }
}
