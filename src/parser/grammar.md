```
Expression ->    Logical_expression
            |    Unary_expression = Expression
```

```
Logical_expression ->   Equality_expression Logical_expression_star
```

```
Logical_expression_star -> and_op Logical_expression   
                         | or_op Logical_expression
                         | xor_op Logical_expression
                         | and Logical_expression
                         | or Logical_expression
                         | ε
```

```
Equality_expression ->  Relational_expression Equality_expression_star
```

```
Equality_expression_star -> == Equality_expression
                          | != Equality_expression
                          | ε
```

```
Relational_expression ->    Shift_expression Relational_expression_star
 
Relational_expression_star ->   > Relational_expression
                            |   < Relational_expression
                            |   >= Relational_expression
                            |   <= Relational_expression
                            |   ε
```

```
Shift_expression -> Additive_expression Shift_expression_star
```

```
Shift_expression_star ->    << Shift_expression
                       |    >> Shift_expression
                       |    ε
```

```
Additive_expression -> Multiplicative_expression Additive_expression_star
```

```
Additive_expression_star -> + Additive_expression 
                          | - Additive_expression
                          | ε
```

```
Multiplicative_expression -> Cast_expression Multiplicative_expression_star
```

```
Multiplicative_expression_star ->   * Multiplicative_expression
                                |   / Multiplicative_expression
                                |   % Multiplicative_expression
                                |   ε
```

```
Cast_expression ->  ( Pointer_type ) Cast_expression
                 |  Unary_expression   
```

```
Pointer_type -> {const} Type_native {*}*
```

```
Type_native -> u8
             | u16
             | u32
             | i8
             | i16
             | i32
             | void 
```

```
Unary_expression -> Postfix_expression
                  | + Unary_expression
                  | - Unary_expression
                  | ! Unary_expression
                  | & Unary_expression
                  | * Unary_expression
```

```
Postfix_expression ->   Primary_expression  Postfix_operator
```

```
Postfix_operator -> [ Expression ] Postfix_operator
                  | ( Expression_list ) Postfix_operator
                  | ( ) Postfix_operator
                  | ε
```

```
Primary_expression ->   identifier
                    |   number
                    |   char
                    |   ( Expression )
```

```
Expression_list ->  Expression Expression_list_star
```

```
Compound_statement ->  { Statement_list }
```

```
Statement_list -> Statement {Statement}*
```

```
Expression_statement -> Optional_expression ;
```

```
Optional_expression ->  Expression 
                     |  ε
```

```
Selection_statement ->  if ( Expression ) Compound_statement Else_statement
```

```
Else_statement ->   ε
                |   else Compound_statement
```

```
Iteration_statement ->  while ( Expression ) Compound_statement
                     |  for ( Optional_expression ; Optional_expression ; Optional_expression ) Compound_statement
```

```
Jump_statement ->   return Optional_expression ;
                |   break ;
                |   continue ;
```

```
Translation_unit -> External_declaration_list eof
```

```
External_declaration_list -> Declaration {Declaration}*
```

```
Declaration ->  Type_declaration identifier ;
             |  Type_declaration identifier =  Expression ;
             |  Type_declaration identifier ( Parameter_list ) ;
             |  Type_declaration identifier ( Parameter_list ) Compound_statement
             |  Type_declaration identifier [ Expression ] ;
```

```
Type_declaration -> Optional_const Pointer_type
```

```
Optional_const ->   const 
                |   ε
```

```
Parameter_list ->   {Type_declaration identifier {, Type_declaration identifier}* }
```
