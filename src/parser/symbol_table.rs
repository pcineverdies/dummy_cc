use edit_distance;

use crate::{
    ast::ast_impl::{AstNode, AstNodeWrapper, TypeWrapper},
    lexer::lexer_impl::Tk,
};

/// Declaration
///
/// Element of the symbol table
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Declaration {
    pub name: String,                // Adopted identifier
    pub is_function: bool,           // Is it a function
    pub return_type: TypeWrapper,    // Return type for functions, types for variables
    pub arguments: Vec<TypeWrapper>, // Types of arguments
}

// Symbol table
//
// Stores the symbols in the current scope
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolTable {
    matrix: Vec<Vec<Declaration>>, // Currest global symbol table
    to_add: Vec<Declaration>,      // Elements to be added to the next scope. This is used to add
                                   // to the scope the arguments of a function
}

impl SymbolTable {
    /// SymbolTable::new
    ///
    /// Create a new symbol table containing the global scope only
    ///
    /// @return [SymbolTable]: created objects
    pub fn new() -> SymbolTable {
        let mut res = SymbolTable {
            matrix: Vec::new(),
            to_add: Vec::new(),
        };
        res.add_scope();
        res
    }

    /// SymbolTable::add_scope
    ///
    /// Add new scope to the symbol table. This is always valid. Add to the scope all the elements
    /// which are currently in the `to_add` list
    pub fn add_scope(&mut self) {
        self.matrix.push(Vec::new());
        let index = self.matrix.len() - 1;
        self.matrix[index].append(&mut self.to_add);
    }

    /// SymbolTable::remove_scope
    ///
    /// Remove the current scope from the symbol table. This is valid whenever the current scope is
    /// not the global one
    pub fn remove_scope(&mut self) {
        if self.matrix.len() != 1 {
            self.matrix.pop();
        } else {
            panic!("Cannot remove global scope");
        }
    }

    /// SymbolTable::add_to_next_scope
    ///
    /// Add symbol to the list of elements to be added to the next scope. This can be done if the
    /// symbol was not already declared.
    ///
    /// @in id [&Declaration]: Declaration to add
    /// @return [Option<()>]: Return Some(()) if the declaration was added succesfully, None if the
    /// identifier was already declared
    pub fn add_to_next_scope(&mut self, id: &Declaration) -> Option<()> {
        let res_search = self.search_definition(&id.name);
        if res_search.is_ok() {
            return None;
        }
        self.to_add.push(id.clone());
        return Some(());
    }

    /// SymbolTable::add_definition
    ///
    /// Add symbol to the current scope. This can be done if the symbol was not already declared
    ///
    /// @in id [&Declaration]: Declaration to add
    /// @return [Option<()>]: Return Some(()) if the declaration was added succesfully, None if the
    /// identifier was already declared
    pub fn add_definition(&mut self, id: &Declaration) -> Option<()> {
        let res_search = self.search_definition(&id.name);
        if res_search.is_ok() {
            return None;
        }
        let index = self.matrix.len() - 1;
        self.matrix[index].push(id.clone());
        return Some(());
    }

    /// SymbolTable::search_definition
    ///
    /// Search for the received symbol in the global table
    ///
    /// @in id [&String]: identifier to check
    /// @return [Result<Declaration, String>]: if the symbol was found, it returns its declaration.
    /// Otherwise, it returns the most similar available symbol, using the Levenshtein distance
    /// function
    pub fn search_definition(&self, id: &String) -> Result<Declaration, String> {
        for v in &self.matrix {
            for i in v {
                if i.name.eq(id) {
                    return Ok(i.clone());
                }
            }
        }

        let mut closer_string = "";
        let mut closer_distance = 1000;
        for v in &self.matrix {
            for i in v {
                let d = edit_distance::edit_distance(i.name.as_str(), id.as_str());
                // Change symbol if distance is smaller than previous smallest one
                if d < closer_distance {
                    closer_string = &i.name;
                    closer_distance = d;
                }
            }
        }
        return Err(closer_string.to_string().clone());
    }

    /// SymbolTable::check_procedure
    ///
    /// Check whether the procedure invoked is valid
    ///
    /// @in primary [&AstNodeWrapper]: primary node, left side of the procedure postfix operator
    /// @in args [&Vec<AstNodeWrapper>]: List of arguments
    /// @return [Result<Declaration, (AstNodeWrapper, String)>]: If something was wrong, returns the node
    /// which caused the error together with an error message. Otherwise it returns the declaration
    /// of the function
    pub fn check_procedure(&self, primary: &AstNodeWrapper, args: &Vec<AstNodeWrapper>) -> Result<Declaration, (AstNodeWrapper, String, String)> {
        // Primary must be an indentifier
        let mut identifier = "";
        if let AstNode::PrimaryNode(n) = &primary.node {
            if let Tk::Identifier(id) = &n.tk {
                identifier = id;
            }
        }
        if identifier == "" {
            return Err((primary.clone(), String::from("function identifier"), String::from("expression")));
        }

        // As an identifier, it must be a function
        let decl = self.search_definition(&identifier.to_string()).unwrap();
        if !decl.is_function {
            return Err((primary.clone(), String::from("function identifier"), String::from("variable identifier")));
        }

        // Number of arguments must be appropriate
        if decl.arguments.len() != args.len() {
            return Err((
                primary.clone(),
                String::from(format!("{} arguments", decl.arguments.len())),
                String::from(format!("{}", args.len())),
            ));
        }

        // Type of arguments must match
        for i in 0..decl.arguments.len() {
            if !TypeWrapper::are_compatible(&decl.arguments[i], &args[i].type_ref) {
                return Err((
                    args[i].clone(),
                    String::from(format!("type {}", decl.arguments[i].to_string())),
                    String::from(format!("type {}", args[i].type_ref.to_string())),
                ));
            }
        }

        return Ok(decl.clone());
    }
}

impl Default for Declaration {
    /// SymbolTable::default
    ///
    /// Creates a new default Declaration
    fn default() -> Declaration {
        Declaration {
            name: String::from(""),
            is_function: false,
            return_type: TypeWrapper { ..Default::default() },
            arguments: vec![],
        }
    }
}
