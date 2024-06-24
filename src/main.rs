use std::env;
mod ast;
mod lexer;
mod lirgen;
mod parser;
use lexer::lexer_impl::Lexer;
use lirgen::lirgen_impl::Lirgen;
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
    if ast_wrapped.is_some() {
        // println!("{:#?}", &ast_wrapped.clone().unwrap());
        // println!("{}", &ast_wrapped.clone().unwrap().to_string(0));
    } else {
        return;
    }
    let mut i = Lirgen::new();
    let ir = i.linearize_ast(&ast_wrapped.unwrap());
    println!("{:#?}", ir);
    println!("{}", Lirgen::to_string(&ir));
}
