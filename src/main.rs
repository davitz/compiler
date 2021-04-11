#[macro_use] extern crate lazy_static;
extern crate regex;


use std::env;
mod lexer;
use lexer::*;

fn main() {
    
    
    let args: Vec<String> = env::args().collect();

    let code = &args[1];

    let tokens: Vec<Token> = lex(code);

    println!("Tokens: {0:?}", tokens);
}
