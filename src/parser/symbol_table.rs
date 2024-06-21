use edit_distance;

use crate::{
    ast::ast_impl::{AstNode, AstNodeWrapper, TypeNative, TypeWrapper},
    lexer::lexer_impl::Tk,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Declaration {
    pub name: String,
    pub is_function: bool,
    pub return_type: TypeWrapper,
    pub arguments: Vec<TypeWrapper>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolTable {
    matrix: Vec<Vec<Declaration>>,
    to_add: Vec<Declaration>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let mut res = SymbolTable {
            matrix: Vec::new(),
            to_add: Vec::new(),
        };
        res.add_scope();
        res
    }

    pub fn add_scope(&mut self) {
        self.matrix.push(Vec::new());
        let index = self.matrix.len() - 1;
        self.matrix[index].append(&mut self.to_add);
    }

    pub fn remove_scope(&mut self) {
        if self.matrix.len() != 1 {
            self.matrix.pop();
        } else {
            panic!("Cannot remove global scope");
        }
    }

    pub fn add_to_next_scope(&mut self, id: &Declaration) -> Option<()> {
        let res_search = self.search_definition(&id.name);
        if res_search.is_ok() {
            return None;
        }
        self.to_add.push(id.clone());
        return Some(());
    }

    pub fn add_definition(&mut self, id: &Declaration) -> Option<()> {
        let res_search = self.search_definition(&id.name);
        if res_search.is_ok() {
            return None;
        }
        let index = self.matrix.len() - 1;
        self.matrix[index].push(id.clone());
        return Some(());
    }

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
                if d < closer_distance {
                    closer_string = &i.name;
                    closer_distance = d;
                }
            }
        }
        return Err(closer_string.to_string().clone());
    }

    pub fn check_procedure(
        &self,
        primary: &AstNodeWrapper,
        args: &Vec<AstNodeWrapper>,
    ) -> Result<(), (AstNodeWrapper, String, String)> {
        // Primary must be an indentifier
        let mut identifier = "";
        if let AstNode::PrimaryNode(n) = &primary.node {
            if let Tk::Identifier(id) = &n.tk {
                identifier = id;
            }
        }
        if identifier == "" {
            return Err((
                primary.clone(),
                String::from("fynction identifier"),
                String::from("expression"),
            ));
        }

        // As an identifier, it must be a function
        let decl = self.search_definition(&identifier.to_string()).unwrap();
        if !decl.is_function {
            return Err((
                primary.clone(),
                String::from("function identifier"),
                String::from("variable identifier"),
            ));
        }

        // If function has no arguments, args must be empty
        if decl.arguments.len() != args.len() {
            return Err((
                primary.clone(),
                String::from(format!("{} arguments", decl.arguments.len())),
                String::from(format!("{}", args.len())),
            ));
        }

        for i in 0..decl.arguments.len() {
            if !TypeWrapper::are_compatible(&decl.arguments[i], &args[i].type_ref) {
                return Err((
                    args[i].clone(),
                    String::from(format!("type {}", decl.arguments[i].to_string())),
                    String::from(format!("type {}", args[i].type_ref.to_string())),
                ));
            }
        }
        // Each argument must match
        return Ok(());
    }
}

impl Default for Declaration {
    fn default() -> Declaration {
        Declaration {
            name: String::from(""),
            is_function: false,
            return_type: TypeWrapper {
                ..Default::default()
            },
            arguments: vec![],
        }
    }
}