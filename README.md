# dummy_cc
Dummy C Compiler for Learning Purposes 

## Structure

### [Lexer](./src/lexer/)
The file [tokens.md](./src/lexer/tokens.md) contains the definition of the tokens currently implemented in the parser. 

### [Parser](./src/parser/)
The file [grammar.md](./src/parser/grammar.md) contains the grammar of the C-like language implemented. 
While generating the AST, many conditions are checked for the correctness of the program. 
In particular, no automatic type conversion is performed (casting must always be explicit).

