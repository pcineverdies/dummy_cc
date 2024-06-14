Translation_unit -> External_declaration_list eof

External_declaration_list -> Declaration External_declaration_list_star

External_declaration_list_star ->   ε
                                |   Translation_unit

Declaration ->  Type_declaration identifier stop
             |  Type_declaration identifier =  Expression stop
             |  Type_declaration identifier ( Parameter_list ) stop
             |  Type_declaration identifier ( Parameter_list ) Compound_statement
             |  Type_declaration identifier Array_declaration stop

Type_declaration -> Optional_const Pointer_type

Optional_const ->   const 
                |   ε

Pointer_type -> Type Pointer

Pointer -> * Pointer
         | ε

Parameter_list ->   Type_declaration identifier Parameter_list_star

Parameter_list_star ->  , Parameter_list
                     |  ε

Type -> u8
      | u16
      | u32
      | i8
      | i16
      | i32
      | void 

Array_declaration ->    [ Logical_expression ] Array_declaration_star

Array_declaration_star ->   Array_declaration
                        |   ε

Compound_statement ->  { Statement_list }

Statement_list -> Statement Statement_list_star

Statement_list_star ->  Statement_list
                     |  ε

Statement ->    Expression_statement
           |    Declaration
           |    Compound_statement
           |    Selection_statement
           |    Iteration_statement
           |    Jump_statement

Expression_statement -> Optional_expression stop

Expression ->    Logical_expression
            |    Unary_expression = Expression

Selection_statement ->  if ( Expression ) Compound_statement
                     |  if ( Expression ) Compound_statement Else_statement

Else_statement ->   ε
                |   else Compound_statement

Iteration_statement ->  while ( Expression ) Compound_statement
                     |  for ( Optional_expression stop Optional_expression stop Optional_expression ) Compound_statement

Optional_expression ->  Expression 
                     |  ε

Logical_expression ->   Equality_expression Logical_expression_star

Logical_expression_star -> & Logical_expression   
                         | | Logical_expression
                         | ^ Logical_expression
                         | ε

Equality_expression ->  Relational_expression Equality_expression_star

Equality_expression_star -> == Equality_expression
                          | != Equality_expression
                          | ε

Relational_expression ->    Shift_expression Relational_expression_star
 
Relational_expression_star ->   > Relational_expression
                            |   < Relational_expression
                            |   >= Relational_expression
                            |   <= Relational_expression
                            |   ε

Shift_expression -> Additive_expression Shift_expression_star

Shift_expression_star ->    << Shift_expression
                       |    >> Shift_expression
                       |    ε

Additive_expression -> Multiplicative_expression Additive_expression_star

Additive_expression_star -> + Additive_expression 
                          | - Additive_expression
                          | ε


Multiplicative_expression -> Cast_expression Multiplicative_expression_star

Multiplicative_expression_star ->   * Multiplicative_expression
                                |   / Multiplicative_expression
                                |   % Multiplicative_expression
                                |   ε

Cast_expression ->  ( Pointer_type ) Cast_expression
                 |  Unary_expression   

Unary_expression -> Postfix_expression
                  | + Unary_expression
                  | - Unary_expression
                  | ! Unary_expression
                  | & Unary_expression
                  | * Unary_expression

Postfix_expression ->   Primary_expression  Postfix_operator

Postfix_operator -> [ Expression ] Postfix_operator
                  | ( Expression_list ) Postfix_operator
                  | ( ) Postfix_operator
                  | ε

Expression_list ->  Expression Expression_list_star

Expression_list_star -> , Expression_list 
                      | ε

Primary_expression ->   identifier
                    |   number
                    |   char
                    |   ( Expression )

Jump_statement ->   return Optional_expression ''
                |   break
