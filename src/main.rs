use std::env;
mod ast;
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
    let ast_wrapped = p.parse();
    if ast_wrapped.is_none() {
        return;
    }
    let ast = ast_wrapped.unwrap();
    println!("{:#?}", ast);
    println!("{}", ast.to_string(0));
}
