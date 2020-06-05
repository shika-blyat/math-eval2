use crate::{
    ast::{Expr, Node},
    errors::{ErrReason, Error},
    tokens::{Operator, Token, TokenKind},
};

pub fn shunting_yard(tokens: Vec<Token>) -> Result<Expr, Error> {
    let mut value_stack = vec![];
    let mut op_stack: Vec<Node<Operator>> = vec![];
    let mut expect_operator = false;
    for Token { kind, span } in tokens {
        match kind {
            TokenKind::Number(n) => {
                if expect_operator {
                    return Err(Error {
                        reason: ErrReason::ExpectedOperator { found: kind },
                        span,
                    });
                }
                value_stack.push(Node {
                    span,
                    value: Expr::Num(n),
                });
                expect_operator = true;
            }
            TokenKind::Op(op) => {
                if !expect_operator && op != Operator::LParen {
                    return Err(Error {
                        reason: ErrReason::ExpectedNumber { found: kind },
                        span,
                    });
                }
                expect_operator = op == Operator::RParen;
                if op != Operator::LParen {
                    while let Some(last_op) = op_stack.last() {
                        let last_op = last_op.value;
                        if (last_op.prec() > op.prec()
                            || (last_op.prec() >= op.prec() && !last_op.is_right_assoc())
                            || op == Operator::RParen)
                            && last_op != Operator::LParen
                        {
                            let right = value_stack.pop().unwrap();
                            let left = value_stack.pop().unwrap();
                            let span = left.span.start..right.span.end;
                            let expr = Expr::BinaryOp(
                                op_stack.pop().unwrap().value,
                                left.into(),
                                right.into(),
                            );
                            let node = Node { value: expr, span };
                            value_stack.push(node)
                        } else {
                            if last_op == Operator::LParen {
                                op_stack.pop();
                            }
                            break;
                        }
                    }
                }
                if op != Operator::RParen {
                    op_stack.push(Node { value: op, span });
                }
            }
            TokenKind::Eof => (),
        }
    }

    if !expect_operator {
        return Err(Error {
            reason: ErrReason::ExpectedNumber {
                found: TokenKind::Eof,
            },
            span: std::usize::MAX..std::usize::MAX,
        });
    }
    for Node { value: op, span } in op_stack.into_iter().rev() {
        if op == Operator::LParen {
            return Err(Error {
                reason: ErrReason::UnclosedParen,
                span,
            });
        }
        let right = value_stack.pop().unwrap();
        let left = value_stack.pop().unwrap();
        let span = left.span.start..right.span.end;
        let expr = Expr::BinaryOp(op, left.into(), right.into());
        let node = Node { value: expr, span };
        value_stack.push(node)
    }
    Ok(value_stack.into_iter().next().unwrap().value)
}
