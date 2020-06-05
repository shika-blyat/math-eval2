use crate::tokens::Operator;

// 1 + 2 * 3

// 1 + (2 * 3)

// (1 + (2 * 3))

/*
    +
   / \
  1   *
     / \
    2   3
*/

#[derive(Debug)]
pub enum Expr {
    Num(i32),
    BinaryOp(Operator, Box<Expr>, Box<Expr>),
}
