mod ast;
mod backend;
mod lexer;
mod lirgen;
mod optimizer;
mod parser;
use backend::codegen_riscv::Codegen as cg_riscv;
use clap::Parser as ClapParser;
use lexer::lexer::Lexer;
use lirgen::lirgen::Lirgen;
use optimizer::optimizer::Optimizer;
use parser::parser::Parser;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path of the file to compile
    #[arg(short, long)]
    file_name: String,

    /// Required level of optimization
    #[arg(short, long, default_value_t = 0, value_parser = clap::value_parser!(u32).range(0..=2))]
    opt: u32,

    /// Show result of parsing
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    print_ast: bool,

    /// Show result of lirgen
    #[arg(long, action = clap::ArgAction::SetTrue, default_value_t = false)]
    print_lir: bool,

    /// Target architecture
    #[arg(short, long, default_value_t = format!("riscv"), value_parser = ["riscv"])]
    arch: String,
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
    let ast = ast_wrapped.unwrap();

    if args.print_ast {
        println!("{}", ast.to_string(0));
    }

    let mut i = Lirgen::new(args.opt);
    let mut ir = i.linearize_ast(&ast);

    if args.opt > 1 {
        let mut opt = Optimizer::new(args.opt);
        ir = opt.optimize(ir);
    }

    if args.print_lir {
        println!("{}", ir.to_string());
    }

    let _code = match args.arch.as_str() {
        "riscv" => {
            let mut codegen = cg_riscv::new();
            codegen.generate_code(&ir);
        }
        _ => panic!("Unsupported architecture: {}", args.arch),
    };
}
