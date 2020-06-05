use crate::{
    ast::Expr,
    errors::ParseErr,
    tokens::{Operator, Token, TokenKind},
};

// 1 + 2 * 3

pub fn shunting_yard(tokens: Vec<Token>) -> Result<Expr, ParseErr> {
    let mut value_stack = vec![];
    let mut op_stack: Vec<Operator> = vec![];
    for Token { kind, span: _ } in tokens {
        match kind {
            TokenKind::Number(n) => value_stack.push(Expr::Num(n)),
            TokenKind::Op(op) => {
                while let Some(last_op) = op_stack.last() {
                    if last_op.prec() > op.prec()
                        || (last_op.prec() >= op.prec() && !last_op.is_right_assoc())
                    {
                        let right = value_stack.pop().unwrap();
                        let left = value_stack.pop().unwrap();
                        value_stack.push(Expr::BinaryOp(
                            op_stack.pop().unwrap(),
                            Box::new(left),
                            Box::new(right),
                        ))
                    } else {
                        break;
                    }
                }
                op_stack.push(op);
            }
        }
    }
    for op in op_stack.into_iter().rev() {
        let right = value_stack.pop().unwrap();
        let left = value_stack.pop().unwrap();
        value_stack.push(Expr::BinaryOp(op, Box::new(left), Box::new(right)));
    }
    Ok(value_stack.into_iter().next().unwrap())
}
