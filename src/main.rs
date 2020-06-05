#![feature(int_error_matching)]
#![feature(or_patterns)]

mod ast;
mod errors;
mod lexer;
mod shunting_yard;
mod tokens;

use std::convert::TryInto;

use ast::{Expr, Node};
use errors::{ErrReason, Error};
use lexer::Lexer;
use shunting_yard::shunting_yard;
use tokens::Operator;

fn eval(expr: &Expr) -> Result<i32, Error> {
    Ok(match expr {
        Expr::Num(n) => *n,
        Expr::BinaryOp(
            op,
            Node {
                value: left,
                span: left_span,
            },
            Node {
                value: right,
                span: right_span,
            },
        ) => {
            let left = eval(&left)?;
            let right = eval(&right)?;
            match op {
                Operator::Add => left + right,
                Operator::Sub => left - right,
                Operator::Div => {
                    if right != 0 {
                        left / right
                    } else {
                        return Err(Error {
                            reason: ErrReason::DivByZero,
                            span: left_span.start..right_span.end,
                        });
                    }
                }
                Operator::Mul => left * right,
                Operator::Pow => left.pow(right.try_into().map_err(|_| Error {
                    reason: ErrReason::PowerByNegative(right),
                    span: right_span.clone(),
                })?),
                _ => unreachable!(),
            }
        }
    })
}

fn main() {
    let lexer = Lexer::new("1 / 5 + 1");
    let tokens = lexer.tokenize();
    println!("{:#?}", tokens);
    let ast = shunting_yard(tokens.unwrap());
    println!("{:#?}", ast);
    println!("{:#?}", eval(&ast.unwrap()));
}
