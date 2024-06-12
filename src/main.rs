use std::env;
mod lexer;
mod parser;
use lexer::lexer_impl::Lexer;
use parser::parser_impl::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = String::from(&args[1]);
    let mut l = Lexer::new(file_name.clone(), true).unwrap();
    let tokens = l.tokenize();
    if tokens.is_none() {
        return;
    }
    let mut p = Parser::new(tokens.unwrap(), file_name.clone());
    p.parse();
}
