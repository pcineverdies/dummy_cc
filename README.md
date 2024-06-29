# dummy_cc
Dummy C Compiler for Learning Purposes 

## Structure

### [Lexer](./src/lexer/)
The file [tokens.md](./src/lexer/tokens.md) contains the definition of the tokens currently implemented in the parser. 

### [Parser](./src/parser/)
The file [grammar.md](./src/parser/grammar.md) contains the grammar of the C-like language implemented. 
While generating the AST, many conditions are checked for the correctness of the program. 
In particular, no automatic type conversion is performed (casting must always be explicit).

### [Linear Ir](./src/lirgen/)
The file [LIR.md](./src/lirgen/lir.md) contains the grammar the linear intermediate representation employed in the middle end.
It is a stack-based lir in SSA format (although without phi functions). 
It can be used to obtain a CFG of the code and implement some optimizations. 
While the project does not focus on optimizing the result, some simple local techinques are employed.

### [Optimizations](./src/optimizer/)
The file [Optimization.md](./src/optimizer/optimization.md) describes the techinques adopted for different level of optimizations.
Some of these techniques are implemented directly during the LIR construction, thus they can be found in [lirgen.rs](./src/lirgen/lirgen.rs).

### [Backend](./src/backend)

## Build

```bash
cargo build -r
```

## Usage

The program in on a single file. It has to follow the syntax described in [grammar.md](./src/parser/grammar.md). 
Relevant messages are shown in case of errors, with references to the input file.

```
Usage: dummy_cc [OPTIONS] --file-name <FILE_NAME>

Options:
  -f, --file-name <FILE_NAME>  Path of the file to compile
  -o, --opt <OPT>              Required level of optimization [default: 0]
      --print-ast...           Show result of parsing
      --print-lir...           Show result of lirgen
  -h, --help                   Print help
  -V, --version                Print version
```

Available levels of optimization are `0`, `1` and `2`.
Using option `--print-ast` you can see a printed version of the ast, highlithing the order in which expressions are evaluated.
Using option `--print-lir` you can see a printed version of intermediate representation after it has been optimized.

## Resources

- [Engineering a Compiler, Second Edition, Cooper & Torcson](https://books.google.it/books/about/Engineering_a_Compiler.html?id=xcJrEAAAQBAJ&source=kp_book_description&redir_esc=y). 
This was the main source of the project, as I build the compiler while studying it.
- [Crafting Interpreters, Chapter 2, Nystrom](https://craftinginterpreters.com/).
This was foundamental to obtain a working recursive descent parser.
- [r/compilers](https://www.reddit.com/r/Compilers/).
- [r/ProgrammingLanguages](https://www.reddit.com/r/ProgrammingLanguages/).
- [carbon-ir](https://github.com/RobbeDGreef/carbon-ir). As inspiration for my IR.
- Many online materail. Unfortunately I have not kept track of all of them. Here are some:
    - [Coping with nontermination: some thoughts on stopping loops](https://outerproduct.net/boring/2023-02-11_term-loop.html).
    - [A gentle introuduction to LLVM IR](https://mcyoung.xyz/2023/08/01/llvm-ir/).
    - [Lectures on optimization from CS143 @Stanford](https://web.stanford.edu/class/archive/cs/cs143/cs143.1128/).
