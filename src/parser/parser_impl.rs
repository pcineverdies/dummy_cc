use crate::ast::ast_impl::AstNode;
use crate::lexer::lexer_impl::{Bracket, Keyword, Operator, Tk, Token};
use std::{fs::read_to_string, process::exit};

macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Parser {
    token_list: Vec<Token>,
    current_position: usize,
    depth: u32,
    errors_counter: u32,
    file_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ParserResult {
    Match(AstNode),
    Unmatch,
    Fail,
}

use ParserResult::*;

impl Parser {
    //! Parser::new
    //!
    //! Initiate the parser with a vector of tokens generated by the lexer
    //!
    //! @input token_list [Vec<Token>]: List of input tokens
    //! @input file_name [String]: Name of the file under analysis
    //! @return [Parser]: Generated parser
    pub fn new(token_list: Vec<Token>, file_name: String) -> Parser {
        Parser {
            token_list,
            current_position: 0,
            depth: 0,
            errors_counter: 0,
            file_name,
        }
    }

    /// Parser::get_current
    ///
    /// Get the current token under exam
    fn get_current(&self) -> Tk {
        return self.token_list[self.current_position].clone().tk;
    }

    /// Parser::get_current_token
    ///
    /// Get the full current token under exam
    fn get_current_token(&mut self) -> Token {
        return self.token_list[self.current_position].clone();
    }

    /// Parser::advance
    ///
    /// Advance to next token
    fn advance(&mut self) {
        debug_println!("Consuming {:?}", self.get_current());
        self.current_position += 1;
        if self.current_position as usize >= self.token_list.len() {
            eprintln!(
                "\x1b[91mFailed parsing with {} errors\x1b[0m",
                self.errors_counter
            );
            exit(1);
        }
    }

    pub fn parse(&mut self) -> Option<AstNode> {
        debug_println!("-> parse");

        match self.statement_list() {
            Match(node) => {
                if self.get_current() != Tk::EOF {
                    self.parser_error("EOF");
                    return None;
                } else {
                    return Some(node);
                }
            }
            _ => {
                eprintln!(
                    "\x1b[91mFailed parsing with {} errors\x1b[0m",
                    self.errors_counter
                );
                return None;
            }
        }
    }

    fn statement_list(&mut self) -> ParserResult {
        debug_println!("-> statement_list");
        let mut statements_vec: Vec<AstNode> = Vec::new();
        match self.statement() {
            Match(statement_node) => {
                if statement_node != AstNode::AstNull {
                    statements_vec.push(statement_node.clone());
                }
                match self.statement_list_star() {
                    Unmatch => return Match(AstNode::new_ast_statements(&statements_vec)),
                    Fail => return Fail,
                    Match(node) => match node {
                        AstNode::AstStatements(mut list) => {
                            statements_vec.append(&mut list);
                            return Match(AstNode::new_ast_statements(&statements_vec));
                        }
                        _ => {
                            return Fail;
                        }
                    },
                }
            }
            Unmatch => {
                return Match(AstNode::new_ast_statements(&statements_vec));
            }
            _ => {
                while self.get_current() != Tk::Semicolon
                    && self.get_current() != Tk::Bracket(Bracket::RCurly)
                {
                    debug_println!("!! Skipping token due to error");
                    self.advance();
                }
                self.advance();
                return self.statement_list();
            }
        }
    }

    fn statement_list_star(&mut self) -> ParserResult {
        debug_println!("-> statement_list_star");
        match self.get_current() {
            Tk::Bracket(Bracket::RCurly) => return Unmatch,
            Tk::EOF => return Unmatch,
            _ => return self.statement_list(),
        }
    }

    fn statement(&mut self) -> ParserResult {
        debug_println!("-> statement");
        match self.variable_declaration() {
            Match(node) => {
                if self.get_current() != Tk::Semicolon {
                    return self.parser_error(";");
                }
                self.advance();
                return Match(node);
            }
            Fail => return Fail,
            _ => {}
        }

        match self.for_statement() {
            Match(node) => return Match(node),
            Fail => return Fail,
            _ => {}
        }

        match self.flow_statement() {
            Match(node) => return Match(node),
            Fail => return Fail,
            _ => {}
        }

        match self.if_statement() {
            Match(node) => return Match(node),
            Fail => return Fail,
            _ => {}
        }

        match self.while_statement() {
            Match(node) => return Match(node),
            Fail => return Fail,
            _ => {}
        }

        match self.assignment_statement() {
            Match(node) => return Match(node),
            Fail => return Fail,
            _ => {}
        }

        match self.get_current() {
            Tk::Semicolon => {
                self.advance();
                return Match(AstNode::new_null());
            }
            Tk::Identifier(_) => return Unmatch,
            Tk::Bracket(Bracket::RCurly) => return Unmatch,
            Tk::EOF => return Unmatch,
            Tk::Keyword(Keyword::If) => return Unmatch,
            Tk::Keyword(Keyword::For) => return Unmatch,
            Tk::Keyword(Keyword::Return) => return Unmatch,
            Tk::Keyword(Keyword::Continue) => return Unmatch,
            Tk::Keyword(Keyword::Break) => return Unmatch,
            Tk::Keyword(Keyword::Char) => return Unmatch,
            Tk::Keyword(Keyword::Int) => return Unmatch,
            Tk::Keyword(Keyword::Bool) => return Unmatch,
            Tk::Keyword(Keyword::U8) => return Unmatch,
            Tk::Keyword(Keyword::U16) => return Unmatch,
            Tk::Keyword(Keyword::U32) => return Unmatch,
            Tk::Keyword(Keyword::While) => return Unmatch,
            _ => {
                return self.parser_error("");
            }
        }
    }

    fn flow_statement(&mut self) -> ParserResult {
        match self.get_current() {
            Tk::Keyword(Keyword::Continue) | Tk::Keyword(Keyword::Break) => {
                let token = self.get_current_token();
                self.advance();
                if self.get_current() != Tk::Semicolon {
                    return self.parser_error(";");
                }
                self.advance();
                return Match(AstNode::new_ast_flow(&token, &AstNode::new_null()));
            }
            Tk::Keyword(Keyword::Return) => {
                let token = self.get_current_token();
                self.advance();
                if self.get_current() == Tk::Semicolon {
                    self.advance();
                    return Match(AstNode::new_ast_flow(&token, &AstNode::new_null()));
                }
                match self.expr() {
                    Match(node) => {
                        if self.get_current() != Tk::Semicolon {
                            return self.parser_error(";");
                        }
                        self.advance();
                        return Match(AstNode::new_ast_flow(&token, &node));
                    }
                    _ => return Fail,
                }
            }
            _ => return Unmatch,
        }
    }

    fn for_statement(&mut self) -> ParserResult {
        debug_println!("-> for_statement");
        if self.get_current() != Tk::Keyword(Keyword::For) {
            return Unmatch;
        }
        self.advance();
        if self.get_current() != Tk::Bracket(Bracket::LBracket) {
            return self.parser_error("(");
        }
        self.advance();
        return match self.variable_declaration() {
            Match(decl_node) => {
                if self.get_current() != Tk::Semicolon {
                    return self.parser_error(";");
                }
                self.advance();
                match self.expr() {
                    Match(expr_node) => {
                        if self.get_current() != Tk::Semicolon {
                            self.parser_error(";");
                        }
                        self.advance();
                        match self.assignment_statement() {
                            Match(ass_node) => {
                                if self.get_current() != Tk::Bracket(Bracket::RBracket) {
                                    self.parser_error(")");
                                }
                                self.advance();
                                if self.get_current() != Tk::Bracket(Bracket::LCurly) {
                                    self.parser_error("{");
                                }
                                self.advance();
                                match self.statement_list() {
                                    Match(list_node) => {
                                        if self.get_current() != Tk::Bracket(Bracket::RCurly) {
                                            self.parser_error("}");
                                        }
                                        self.advance();
                                        Match(AstNode::new_ast_for(
                                            &decl_node, &expr_node, &ass_node, &list_node,
                                        ))
                                    }
                                    _ => Fail,
                                }
                            }
                            _ => Fail,
                        }
                    }
                    _ => Fail,
                }
            }
            _ => Fail,
        };
    }

    fn variable_declaration(&mut self) -> ParserResult {
        debug_println!("-> variable_declaration");
        if self.get_current() != Tk::Keyword(Keyword::Char)
            && self.get_current() != Tk::Keyword(Keyword::Bool)
            && self.get_current() != Tk::Keyword(Keyword::Int)
            && self.get_current() != Tk::Keyword(Keyword::U8)
            && self.get_current() != Tk::Keyword(Keyword::U16)
            && self.get_current() != Tk::Keyword(Keyword::U32)
        {
            return Unmatch;
        }
        let token_type = self.get_current_token();
        self.advance();
        match self.get_current() {
            Tk::Identifier(_) => {
                let id_node = AstNode::new_ast_identifer(&self.get_current_token());
                self.advance();
                if self.get_current() != Tk::Operator(Operator::Assign) {
                    return self.parser_error("=");
                }
                self.advance();
                match self.expr() {
                    Match(expr_node) => {
                        return Match(AstNode::new_ast_decl(&token_type, &id_node, &expr_node));
                    }
                    _ => return Fail,
                }
            }
            _ => return self.parser_error("identifier"),
        }
    }

    fn if_statement(&mut self) -> ParserResult {
        debug_println!("-> if_statement");
        if self.get_current() != Tk::Keyword(Keyword::If) {
            return Unmatch;
        }
        self.advance();
        if self.get_current() != Tk::Bracket(Bracket::LBracket) {
            return self.parser_error("(");
        }
        self.advance();
        match self.expr() {
            Match(expr_node) => {
                if self.get_current() != Tk::Bracket(Bracket::RBracket) {
                    return self.parser_error(")");
                }
                self.advance();
                if self.get_current() != Tk::Bracket(Bracket::LCurly) {
                    return self.parser_error("{");
                }
                self.advance();
                match self.statement_list() {
                    Match(if_list_node) => {
                        if self.get_current() != Tk::Bracket(Bracket::RCurly) {
                            return self.parser_error("}");
                        }
                        self.advance();
                        match self.else_statement() {
                            Match(else_list_node) => {
                                return Match(AstNode::new_ast_if(
                                    &expr_node,
                                    &if_list_node,
                                    &else_list_node,
                                ));
                            }
                            Unmatch => {
                                return Match(AstNode::new_ast_if(
                                    &expr_node,
                                    &if_list_node,
                                    &AstNode::new_null(),
                                ));
                            }
                            Fail => return Fail,
                        }
                    }
                    _ => return Fail,
                }
            }
            _ => return Fail,
        }
    }

    fn while_statement(&mut self) -> ParserResult {
        debug_println!("-> while_statement");
        if self.get_current() != Tk::Keyword(Keyword::While) {
            return Unmatch;
        }
        self.advance();
        if self.get_current() != Tk::Bracket(Bracket::LBracket) {
            return self.parser_error("(");
        }
        self.advance();
        match self.expr() {
            Match(node_expr) => {
                if self.get_current() != Tk::Bracket(Bracket::RBracket) {
                    return self.parser_error(")");
                }
                self.advance();
                if self.get_current() != Tk::Bracket(Bracket::LCurly) {
                    return self.parser_error("{");
                }
                self.advance();
                match self.statement_list() {
                    Match(node) => {
                        if self.get_current() != Tk::Bracket(Bracket::RCurly) {
                            return self.parser_error("}");
                        }
                        self.advance();
                        return Match(AstNode::new_ast_while(&node_expr, &node));
                    }
                    _ => Fail,
                }
            }
            _ => return Fail,
        }
    }

    fn else_statement(&mut self) -> ParserResult {
        if self.get_current() == Tk::Keyword(Keyword::Else) {
            self.advance();
            if self.get_current() != Tk::Bracket(Bracket::LCurly) {
                return self.parser_error("{");
            }
            self.advance();
            match self.statement_list() {
                Match(node) => {
                    if self.get_current() != Tk::Bracket(Bracket::RCurly) {
                        return self.parser_error("}");
                    }
                    self.advance();
                    return Match(node);
                }
                _ => return Fail,
            }
        } else {
            match self.get_current() {
                Tk::Identifier(_) => return Unmatch,
                Tk::EOF => return Unmatch,
                Tk::Bracket(Bracket::RCurly) => return Unmatch,
                Tk::Semicolon => return Unmatch,
                Tk::Keyword(Keyword::For) => return Unmatch,
                Tk::Keyword(Keyword::While) => return Unmatch,
                Tk::Keyword(Keyword::Return) => return Unmatch,
                Tk::Keyword(Keyword::Break) => return Unmatch,
                Tk::Keyword(Keyword::Continue) => return Unmatch,
                Tk::Keyword(Keyword::Char) => return Unmatch,
                Tk::Keyword(Keyword::Int) => return Unmatch,
                Tk::Keyword(Keyword::Bool) => return Unmatch,
                Tk::Keyword(Keyword::U8) => return Unmatch,
                Tk::Keyword(Keyword::U16) => return Unmatch,
                Tk::Keyword(Keyword::U32) => return Unmatch,
                Tk::Keyword(Keyword::If) => return Unmatch,
                _ => {
                    return self.parser_error("");
                }
            }
        }
    }

    fn assignment_statement(&mut self) -> ParserResult {
        debug_println!("-> assignment_statement");
        match self.get_current() {
            Tk::Identifier(_) => {
                let id_node = AstNode::new_ast_identifer(&self.get_current_token());
                self.advance();
                if self.get_current() != Tk::Operator(Operator::Assign) {
                    return self.parser_error("=");
                }
                self.advance();
                let expr_result = self.expr();
                return match expr_result {
                    Match(node) => return Match(AstNode::new_ast_assignment(&id_node, &node)),
                    _ => Fail,
                };
            }
            _ => return Unmatch,
        }
    }

    fn expr(&mut self) -> ParserResult {
        debug_println!("-> expr");

        let mut op_stack: Vec<Token> = Vec::new();
        let mut node_stack: Vec<AstNode> = Vec::new();

        match self.comparison() {
            Match(node) => {
                node_stack.push(node);
                while self.get_current() == Tk::Operator(Operator::EqualCompare)
                    || self.get_current() == Tk::Operator(Operator::DiffCompare)
                {
                    op_stack.push(self.get_current_token());
                    self.advance();
                    match self.comparison() {
                        Match(node) => node_stack.push(node),
                        _ => return Fail,
                    }
                }
                let mut final_node = node_stack.remove(0);
                while node_stack.len() != 0 {
                    let op = op_stack.remove(0);
                    let new_operand = node_stack.remove(0);
                    final_node = AstNode::new_ast_binary(&op, &final_node, &new_operand);
                }
                if !self.is_expression_unmatch() {
                    return Fail;
                }
                Match(final_node)
            }
            _ => Fail,
        }
    }

    fn comparison(&mut self) -> ParserResult {
        debug_println!("-> comparison");

        let mut op_stack: Vec<Token> = Vec::new();
        let mut node_stack: Vec<AstNode> = Vec::new();

        match self.term() {
            Match(node) => {
                node_stack.push(node);
                while self.get_current() == Tk::Operator(Operator::LTCompare)
                    || self.get_current() == Tk::Operator(Operator::GTCompare)
                    || self.get_current() == Tk::Operator(Operator::LECompare)
                    || self.get_current() == Tk::Operator(Operator::GECompare)
                {
                    op_stack.push(self.get_current_token());
                    self.advance();
                    match self.term() {
                        Match(node) => node_stack.push(node),
                        _ => return Fail,
                    }
                }
                let mut final_node = node_stack.remove(0);
                while node_stack.len() != 0 {
                    let op = op_stack.remove(0);
                    let new_operand = node_stack.remove(0);
                    final_node = AstNode::new_ast_binary(&op, &final_node, &new_operand);
                }
                if !self.is_expression_unmatch() {
                    return Fail;
                }
                Match(final_node)
            }
            _ => Fail,
        }
    }

    fn term(&mut self) -> ParserResult {
        debug_println!("-> term");

        let mut op_stack: Vec<Token> = Vec::new();
        let mut node_stack: Vec<AstNode> = Vec::new();

        match self.factor() {
            Match(node) => {
                node_stack.push(node);
                while self.get_current() == Tk::Operator(Operator::Plus)
                    || self.get_current() == Tk::Operator(Operator::Minus)
                    || self.get_current() == Tk::Operator(Operator::Xor)
                    || self.get_current() == Tk::Operator(Operator::And)
                    || self.get_current() == Tk::Operator(Operator::Or)
                {
                    op_stack.push(self.get_current_token());
                    self.advance();
                    match self.factor() {
                        Match(node) => node_stack.push(node),
                        _ => return Fail,
                    }
                }
                let mut final_node = node_stack.remove(0);
                while node_stack.len() != 0 {
                    let op = op_stack.remove(0);
                    let new_operand = node_stack.remove(0);
                    final_node = AstNode::new_ast_binary(&op, &final_node, &new_operand);
                }
                if !self.is_expression_unmatch() {
                    return Fail;
                }
                Match(final_node)
            }
            _ => Fail,
        }
    }

    fn factor(&mut self) -> ParserResult {
        debug_println!("-> factor");

        let mut op_stack: Vec<Token> = Vec::new();
        let mut node_stack: Vec<AstNode> = Vec::new();

        match self.unary() {
            Match(node) => {
                node_stack.push(node);
                while self.get_current() == Tk::Operator(Operator::Module)
                    || self.get_current() == Tk::Operator(Operator::Slash)
                    || self.get_current() == Tk::Operator(Operator::Module)
                    || self.get_current() == Tk::Operator(Operator::Asterisk)
                {
                    op_stack.push(self.get_current_token());
                    self.advance();
                    match self.unary() {
                        Match(node) => node_stack.push(node),
                        _ => return Fail,
                    }
                }
                let mut final_node = node_stack.remove(0);
                while node_stack.len() != 0 {
                    let op = op_stack.remove(0);
                    let new_operand = node_stack.remove(0);
                    final_node = AstNode::new_ast_binary(&op, &final_node, &new_operand);
                }
                if !self.is_expression_unmatch() {
                    return Fail;
                }
                Match(final_node)
            }
            _ => Fail,
        }
    }

    fn unary(&mut self) -> ParserResult {
        debug_println!("-> unary");
        if self.get_current() == Tk::Operator(Operator::Plus)
            || self.get_current() == Tk::Operator(Operator::Minus)
            || self.get_current() == Tk::Operator(Operator::Not)
            || self.get_current() == Tk::Operator(Operator::Complement)
        {
            let token = self.get_current_token();
            self.advance();
            return match self.primary() {
                Match(node) => return Match(AstNode::new_ast_unary(&token, &node)),
                _ => Fail,
            };
        }
        return self.primary();
    }

    fn primary(&mut self) -> ParserResult {
        debug_println!("-> primary");
        match self.get_current() {
            Tk::IntegerLiteral(_) => {
                let token = self.get_current_token();
                self.advance();
                return Match(AstNode::new_ast_numerical(&token));
            }
            Tk::Identifier(_) => {
                let token = self.get_current_token();
                self.advance();
                return Match(AstNode::new_ast_identifer(&token));
            }
            Tk::Char(_) => {
                let token = self.get_current_token();
                self.advance();
                return Match(AstNode::new_ast_character(&token));
            }
            Tk::Keyword(Keyword::True) | Tk::Keyword(Keyword::False) => {
                let token = self.get_current_token();
                self.advance();
                return Match(AstNode::new_ast_boolean(&token));
            }
            Tk::Bracket(Bracket::LBracket) => {
                self.advance();
                match self.expr() {
                    Match(node) => {
                        if self.get_current() != Tk::Bracket(Bracket::RBracket) {
                            return self.parser_error("right bracket");
                        }
                        self.advance();
                        return Match(node);
                    }
                    _ => {
                        return Fail;
                    }
                }
            }
            _ => {
                return self.parser_error("");
            }
        }
    }

    fn parser_error(&mut self, expected: &str) -> ParserResult {
        self.errors_counter += 1;
        let line_number = self.token_list[self.current_position].line_number;
        let character_number = self.token_list[self.current_position].character_number;
        let file_lines = self.read_lines(&self.file_name);

        eprint!(
            "\x1b[34m{}:{}:{}: \x1b[0m",
            self.file_name, line_number, character_number
        );

        if expected.len() != 0 {
            eprintln!(
                "\x1b[91merror parser: \x1b[0mexpected `\x1b[34m{}\x1b[0m`, found `\x1b[34m{:?}\x1b[0m`",
                expected,
                self.get_current()
            );
        } else {
            eprintln!(
                "\x1b[91merror parser: \x1b[0munexpected token `\x1b[34m{:?}\x1b[0m`",
                self.get_current()
            );
        }
        if self.errors_counter > 1 {
            eprintln!("(this error might be a propagation of the previous ones)");
        }
        if line_number as usize > file_lines.len() {
            return Fail;
        }

        eprint!(
            "{}\t| {}\n\t| ",
            line_number,
            file_lines[line_number as usize - 1]
        );

        if character_number > 3 {
            for _ in 0..character_number - 4 {
                eprint!(" ");
            }
        }

        for _ in 0..3.min(character_number - 1) {
            eprint!("\x1b[91m~\x1b[0m");
        }

        eprintln!("\x1b[91m^~~~\x1b[0m");

        return Fail;
    }

    fn read_lines(&self, filename: &str) -> Vec<String> {
        let mut result = Vec::new();

        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }

        result
    }

    fn is_expression_unmatch(&self) -> bool {
        if self.get_current() == Tk::Bracket(Bracket::RBracket)
            || self.get_current() == Tk::Semicolon
            || self.get_current() == Tk::Operator(Operator::EqualCompare)
            || self.get_current() == Tk::Operator(Operator::DiffCompare)
            || self.get_current() == Tk::Operator(Operator::LTCompare)
            || self.get_current() == Tk::Operator(Operator::GTCompare)
            || self.get_current() == Tk::Operator(Operator::LECompare)
            || self.get_current() == Tk::Operator(Operator::GECompare)
            || self.get_current() == Tk::Operator(Operator::Xor)
            || self.get_current() == Tk::Operator(Operator::Or)
            || self.get_current() == Tk::Operator(Operator::And)
            || self.get_current() == Tk::Operator(Operator::Minus)
            || self.get_current() == Tk::Operator(Operator::Plus)
            || self.get_current() == Tk::Operator(Operator::Slash)
            || self.get_current() == Tk::Operator(Operator::Asterisk)
        {
            return true;
        }

        return false;
    }
}
