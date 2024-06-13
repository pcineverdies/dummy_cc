# Grammar

## Grammar definition

`Goal -> Statements_List eof`

`Statements_List -> Statement | Statement Statements_List`

`Statement -> Variable_Declaration stop | If_Statement | For_Statement | While_Statement |  Flow_Statement | Assignment_Statement stop | stop | ε`

`Variable_Declaration -> Type identifier = Expr`

`Type -> char | int | bool | u8 | u16 | u32`

`If_Statement -> if ( Expr ) { Statements_List } Else_Statement`

`Else_Statement -> else { Statements_List } | ε`

`Assignment_Statement -> identifier = Expr`

`While_Statement -> while ( Expr ) { Statements_List }`

`For_Statement -> for ( Variable_Declaration stop Expr stop Assignment_Statement ) { Statements_List }`

`Flow_Statement -> return Expr stop | break  stop | continue stop | return stop`

`Expr -> Comparison | Comparison == Expr | Comparison != Expr`

`Comparison -> Term | Term < Comparison | Term > Comparison | Term <= Comparison | Term >= Comparison`

`Term -> Factor Term_Star`

`Term -> Factor | Factor + Term | Factor - Term | Factor or Term | Factor and Term | Factor ^ Term`

`Factor -> Unary | Unary / Factor | Unary * Factor | Unary % Factor`

`Unary -> Primary | + Primary | - Primary | ! Primary | ~ Primary`

`Primary -> ( Expr ) | numerical | identifier | true | false | char`

## Grammar restructured for implementation of Recursive Descent parser

`Goal -> Statements_List eof`

`Statements_List -> Statement Statement_List_Star`

`Statement_List_Star -> ε | Statements_List`

`Statement -> Variable_Declaration stop | If_Statement | For_Statement | While_Statement |  Flow_Statement | Assignment_Statement stop | stop | ε`

`Variable_Declaration -> Type identifier = Expr`
 
`Type -> char | int | bool | u8 | u16 | u32`

`If_Statement -> if ( Expr ) { Statements_List } Else_Statement`

`Assignment_Statement -> identifier = Expr`

`Expr -> Equality`

`Equality -> Comparison Equality_Star`

`Equality_Star -> == Comparison Equality_Star | != Comparison Equality_Star | ε`

`Comparison -> Term Comparison_Star`

`Comparison_Star -> > Term Comparison_Star | < Term Comparison_Star | >= Term Comparison_Star | <= Term Comparison_Star | ε`

`Term -> Factor Term_Star`

`Term_Star -> + Factor Term_Star | - Factor Term_Star | or Factor Term_Star | and Factor Term_Star | xor Factor Term_Star | ε`

`Factor -> Unary Factor_Star`

`Factor_Star -> / Unary Factor_Star | * Unary Factor_Star | % Unary Factor_Star | ε`

`Unary -> Primary | + Primary | - Primary | ! Primary | ~ Primary`

`Primary -> ( Expr ) | numerical | identifier | true | false | char`

`Else_Statement -> else { Statements_List } | ε`

`While_Statement -> while ( Expr ) { Statements_List }`

`For_Statement -> for ( Variable_Declaration stop Expr stop Assignment_Statement ) { Statements_List }`

`Flow_Statement -> return Expr stop | break  stop | continue stop | return stop`

## What is missing

- Function declaration
- Function call
- Array
- Pointers
- Pointers arithmetic
- Address operator
