mod ast;
mod lexer;
mod lirgen;
mod optimizer;
mod parser;
use clap::Parser as ClapParser;
use lexer::lexer_impl::Lexer;
use lirgen::lirgen_impl::Lirgen;
// use optimizer::optimizer_impl::Optimizer;
use parser::parser_impl::Parser;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path of the file to compile
    #[arg(short, long)]
    file_name: String,

    /// Required level of optimization
    #[arg(short, long, default_value_t = 0, value_parser = clap::value_parser!(u32).range(0..=2))]
    opt: u32,
}

fn main() {
    let args = Cli::parse();

    let mut l = Lexer::new(args.file_name.clone(), true).unwrap();
    let tokens = l.tokenize();
    if tokens.is_none() {
        return;
    }
    let mut p = Parser::new(tokens.unwrap(), args.file_name.clone());
    let ast_wrapped = p.parse();

    if ast_wrapped.is_none() {
        return;
    }

    let mut i = Lirgen::new(args.opt);
    let ir = i.linearize_ast(&ast_wrapped.unwrap());
    println!("{}", ir.to_string());
    // let opt = Optimizer::new();
}
