use std::env;
mod lexer;
mod parser;
use lexer::lexer_impl::Lexer;
use parser::parser_impl::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut l = Lexer::new(String::from(&args[1]), true).unwrap();
    let tokens = l.tokenize();
    println!("{:?}", tokens);
    let mut p = Parser::new(tokens);
    p.parse();
}
