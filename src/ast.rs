use crate::tokens::{Operator, Span};

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub span: Span,
}

impl<T> Into<Node<Box<T>>> for Node<T> {
    fn into(self) -> Node<Box<T>> {
        Node {
            value: Box::new(self.value),
            span: self.span,
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Num(i32),
    BinaryOp(Operator, Node<Box<Expr>>, Node<Box<Expr>>),
}
