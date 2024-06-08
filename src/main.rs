use std::env;
mod lexer;
use lexer::lexer_impl::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut l = Lexer::new(String::from(&args[1]), true).unwrap();
    let _ = l.tokenize();
}
