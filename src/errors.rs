use crate::tokens::{Span, TokenKind};
#[derive(Debug)]
pub struct Error {
    pub span: Span,
    pub reason: ErrReason,
}
#[derive(Debug)]
pub enum ErrReason {
    NumOverflow { max_size: &'static str },
    NumUnderflow { min_size: &'static str },
    UnexpectedChar(char),
    ExpectedOperator { found: TokenKind },
    ExpectedNumber { found: TokenKind },
    UnclosedParen,
    DivByZero,
    PowerByNegative(i32),
}
