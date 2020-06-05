#![feature(int_error_matching)]
#![feature(or_patterns)]

mod ast;
mod errors;
mod lexer;
mod shunting_yard;
mod tokens;

use std::{
    convert::TryInto,
    io::{self, Write},
};

use ast::{Expr, Node};
use errors::{ErrReason, Error};
use lexer::Lexer;
use shunting_yard::shunting_yard;
use tokens::Operator;

fn eval_ast(expr: &Expr) -> Result<i32, Error> {
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
            let left = eval_ast(left)?;
            let right = eval_ast(right)?;
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

fn eval(s: &str) -> Result<i32, Error> {
    let lexer = Lexer::new(s);
    let tokens = lexer.tokenize();
    eval_ast(&shunting_yard(tokens?)?)
}
fn main() {
    let mut buffer = String::with_capacity(1064);
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        let buffer_s = buffer.as_str().trim();
        if buffer_s.len() != 0 {
            match buffer_s {
                "quit" | "exit" => break,
                s => {
                    match eval(s) {
                        Ok(v) => println!("{}", v),
                        Err(e) => eprintln!("{:#?}", e),
                    }
                    buffer.clear()
                }
            }
        }
    }
}
