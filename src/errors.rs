use crate::tokens::Span;
#[derive(Debug)]
pub struct ParseErr {
    pub span: Span,
    pub reason: ErrReason,
}
#[derive(Debug)]
pub enum ErrReason {
    NumOverflow { max_size: &'static str },
    NumUnderflow { min_size: &'static str },
    UnexpectedChar(char),
}
