use std::{iter::Peekable, num::IntErrorKind, str::Chars};

use crate::{
    errors::{ErrReason, Error},
    tokens::{Operator, Token, TokenKind},
};

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            position: 0,
        }
    }
    pub fn tokenize(mut self) -> Result<Vec<Token>, Error> {
        let mut result_tokens = vec![];
        loop {
            match self.next() {
                Some(c) if c.is_ascii_digit() => {
                    result_tokens.push(self.number(c)?);
                }
                Some(c) if c.is_whitespace() => (),
                Some(c @ ('+' | '-' | '*' | '/' | '^' | '(' | ')')) => result_tokens.push(Token {
                    span: self.position..self.position + 1,
                    kind: TokenKind::Op(Operator::from(c)),
                }),
                Some(c) => {
                    return Err(Error {
                        span: self.position..self.position + 1,
                        reason: ErrReason::UnexpectedChar(c),
                    })
                }
                None => break,
            }
        }
        result_tokens.push(Token {
            span: std::usize::MAX..std::usize::MAX,
            kind: TokenKind::Eof,
        });
        Ok(result_tokens)
    }
    fn number(&mut self, first_char: char) -> Result<Token, Error> {
        let mut num = first_char.to_string();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num.push(self.next().unwrap())
            } else {
                break;
            }
        }
        let num_len = num.len();
        let span = self.position - num_len..self.position;
        match num.parse::<i32>() {
            Ok(num) => Ok(Token {
                span,
                kind: TokenKind::Number(num),
            }),
            Err(e) => match e.kind() {
                IntErrorKind::Overflow => Err(Error {
                    span,
                    reason: ErrReason::NumOverflow {
                        max_size: "2_147_483_647",
                    },
                }),
                IntErrorKind::Underflow => Err(Error {
                    span,
                    reason: ErrReason::NumUnderflow {
                        min_size: "-2_147_483_648",
                    },
                }),
                _ => unreachable!(),
            },
        }
    }
    fn next(&mut self) -> Option<char> {
        self.position += 1;
        self.source.next()
    }
    fn peek(&mut self) -> Option<&'_ char> {
        self.source.peek()
    }
}
