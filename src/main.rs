#![feature(int_error_matching)]
#![feature(or_patterns)]

mod ast;
mod errors;
mod lexer;
mod shunting_yard;
mod tokens;

use lexer::Lexer;
use shunting_yard::shunting_yard;
use tokens::{Token, TokenKind};

fn main() {
    let lexer = Lexer::new("1 + 2^3^4 - 5");

    let tokens = lexer.tokenize();
    println!("{:#?}", tokens);
    let ast = shunting_yard(tokens.unwrap());
    println!("{:#?}", ast);
}
