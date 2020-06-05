use std::ops::Range;

pub type Span = Range<usize>;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Number(i32),
    Op(Operator),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}
impl Operator {
    pub fn prec(&self) -> u8 {
        match self {
            Operator::Add | Operator::Sub => 5,
            Operator::Mul | Operator::Div => 10,
            Operator::Pow => 15,
        }
    }
    pub fn is_right_assoc(&self) -> bool {
        self == &Self::Pow
    }
}
impl From<char> for Operator {
    fn from(c: char) -> Operator {
        match c {
            '+' => Operator::Add,
            '-' => Operator::Sub,
            '*' => Operator::Mul,
            '/' => Operator::Div,
            '^' => Operator::Pow,
            _ => panic!("Cannot build an operator from `{}`", c),
        }
    }
}
