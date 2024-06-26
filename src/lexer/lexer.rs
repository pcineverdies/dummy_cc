use crate::lexer::token::{Bracket, Keyword, Operator, Tk, Token};
use std::fs;
use std::fs::read_to_string;

/// struct Lexer
///
/// Strcture associated to the lexer
#[derive(Clone, Debug)]
pub struct Lexer {
    input_code: Vec<char>,
    current_index: usize,
    current_char: char,
    lexemes_list: Vec<Token>,
    current_line_number: u32,
    current_character_number: u32,
    errors_counter: u32,
    file_name: String,
    is_file: bool,
    current_first_character: u32,
}

impl Lexer {
    //! Lexer::new
    //!
    //! Create a new lexer given a file to tokenize
    //!
    //! @in [String]: Path to the file to open
    //! @in [bool]: The string might be the text directly (used for testing purposes)
    //! @return [Result<Lexer, Box<dyn std::error::Error + 'static>>]: Lexer if the file could be
    //! opened, an error otherwise
    pub fn new(file_name: String, is_file: bool) -> Result<Lexer, Box<dyn std::error::Error + 'static>> {
        let input_code = if is_file {
            let input_code_u8 = fs::read(file_name.clone())?;
            // Open file and read all the characters
            input_code_u8.iter().map(|b| *b as char).collect::<Vec<_>>()
        } else {
            file_name.chars().collect()
        };

        Ok(Lexer {
            // First character of the file
            current_char: input_code[0],
            // File
            input_code,
            // Index to 0
            current_index: 0,
            // New list of empty lexems tokenized
            lexemes_list: Vec::new(),
            // Current line number analyzed
            current_line_number: 1,
            // Current character number in the line
            current_character_number: 1,
            // Number of errors found
            errors_counter: 0,
            // Name of the file to handle
            file_name,
            // Handling a file?
            is_file,
            // First character number of the token under anaylsis
            current_first_character: 0,
        })
    }

    /// Lexer::tokenize
    ///
    /// Tokenize the opened file
    /// TODO: The lexer should be able to parse more than one file. For this reason, the tokenizer
    /// should be able to run on multiple files
    ///
    /// @return [Option<Vec<Token>>]: List of tokens from the lexed file, None if error is found
    pub fn tokenize(&mut self) -> Option<Vec<Token>> {
        // Until the file is not finished
        while self.current_index < self.input_code.len() {
            // Skip all the whitespaces
            self.skip_whitespaces();
            // Skip all the comments
            self.skip_comments();

            // Since a token is starting, we can mark the initial character
            self.current_first_character = self.current_character_number;
            // Get next token
            let token = self.get_next_token();

            // If the token is not valid, push an error
            if token.is_none() {
                self.lexemes_list.push(Token {
                    tk: Tk::ERROR,
                    line_number: self.current_line_number,
                    last_character: self.current_character_number,
                    first_character: self.current_first_character,
                });
            // Push the valid token
            } else {
                let token_unwrapped = token.unwrap();
                if token_unwrapped != Tk::EOF {
                    self.lexemes_list.push(Token {
                        tk: token_unwrapped,
                        line_number: self.current_line_number,
                        last_character: self.current_character_number,
                        first_character: self.current_first_character,
                    });
                }
            }

            // Advance index
            self.advance_index();
        }

        self.lexemes_list.push(Token {
            tk: Tk::EOF,
            line_number: self.current_line_number,
            last_character: self.current_character_number,
            first_character: self.current_first_character,
        });

        // Return the list of tokens
        if self.errors_counter == 0 {
            return Some(self.lexemes_list.clone());
        } else {
            return None;
        }
    }

    /// Lexer::get_next_token
    ///
    /// Get the next token to be handled
    ///
    /// @return [Option<Tk>]: Return either a token or None if invalid is found
    pub fn get_next_token(&mut self) -> Option<Tk> {
        // End of file token
        if self.current_char == '\0' {
            return Some(Tk::EOF);
        }

        // Semicolon token
        if self.current_char == ';' {
            return Some(Tk::Semicolon);
        }

        // Bracket token
        let bracket = Bracket::from(&self.current_char.to_string());
        if bracket.is_some() {
            return Some(Tk::Bracket(bracket.unwrap()));
        }

        // Operator token: operators with more than one characters always start with a token
        let operator = Operator::from(&self.current_char.to_string());
        if operator.is_some() {
            // == operator
            if self.current_char == '=' && self.get_char(self.current_index + 1) == '=' {
                self.advance_index();
                return Some(Tk::Operator(Operator::EqualCompare));
            }
            // != operator
            if self.current_char == '!' && self.get_char(self.current_index + 1) == '=' {
                self.advance_index();
                return Some(Tk::Operator(Operator::DiffCompare));
            }
            // <= operator
            if self.current_char == '<' && self.get_char(self.current_index + 1) == '=' {
                self.advance_index();
                return Some(Tk::Operator(Operator::LECompare));
            }
            // >= operator
            if self.current_char == '>' && self.get_char(self.current_index + 1) == '=' {
                self.advance_index();
                return Some(Tk::Operator(Operator::GECompare));
            }
            // >> operator
            if self.current_char == '>' && self.get_char(self.current_index + 1) == '>' {
                self.advance_index();
                return Some(Tk::Operator(Operator::RShift));
            }
            // << operator
            if self.current_char == '<' && self.get_char(self.current_index + 1) == '<' {
                self.advance_index();
                return Some(Tk::Operator(Operator::LShift));
            }

            return Some(Tk::Operator(operator.unwrap()));
        }

        // String token: handle the parsing of a string
        if self.current_char == '\"' {
            self.advance_index();
            let str = self.read_string();
            if str.is_some() {
                return Some(Tk::String(str.unwrap()));
            } else {
                self.lexer_error(format!("Line {}: String is not closed", self.current_line_number));
                return None;
            }
        }

        // Character token: handle the parsing of a single character
        if self.current_char == '\'' {
            self.advance_index();
            let chr = self.read_char();
            if chr.is_some() {
                return Some(Tk::Char(chr.unwrap()));
            } else {
                return None;
            }
        }

        // If the first character is alphabetic, then we have an identifier
        if self.current_char.is_alphabetic() || self.current_char == '_' {
            let str = self.read_identifier();

            // the identifier might be a keyword
            let keyword = Keyword::from(&str);
            if keyword.is_some() {
                return Some(Tk::Keyword(keyword.unwrap()));
            } else {
                return Some(Tk::Identifier(str));
            }
        }

        // If the first charactrer is a number, then we have a number
        if self.current_char.is_numeric() {
            let str = self.read_number();
            if str.is_some() {
                return Some(Tk::IntegerLiteral(str.unwrap()));
            } else {
                return None;
            }
        }

        // Invalid token
        self.lexer_error(format!("Invalid characrter found ",));
        return None;
    }

    /// Lexer::advance_index
    ///
    /// Consider next character
    fn advance_index(&mut self) {
        self.current_index += 1;
        self.current_character_number += 1;

        // If we have a \n, then the also increase the line number
        if self.current_char == '\n' {
            self.current_line_number += 1;
            self.current_character_number = 1;
        }

        // If we have covered the whole file, then we can return the EOF
        if self.current_index >= self.input_code.len() {
            self.current_char = '\0';
        // else we return the currenct character
        } else {
            self.current_char = self.get_char(self.current_index);
        }
    }

    /// Lexer::skip_whitespaces
    ///
    /// Skip the whitespaces until a character is found
    fn skip_whitespaces(&mut self) {
        loop {
            if self.current_char.is_whitespace() && self.current_char != '\0' {
                self.advance_index();
            } else {
                break;
            }
        }
    }

    /// Lexer::skip comments until a character is found
    ///
    /// Skip the whitespaces until a character is found
    fn skip_comments(&mut self) {
        // This must be handled through a loop as there might be more than one comment in a row
        loop {
            // A comment is found if the sequence // is found in the text
            if self.current_char == '/' && self.current_index + 1 < self.input_code.len() && self.get_char(self.current_index + 1) == '/' {
                // Go ahead until an end of line is found
                loop {
                    self.advance_index();
                    if self.current_char == '\n' {
                        self.skip_whitespaces();
                        break;
                    }
                }
            // Continue until all the whitespaces are skipped and a real character is ready
            } else {
                self.skip_whitespaces();
                break;
            }
        }
    }

    /// Lexer::get_char
    ///
    /// get the next character to handle
    ///
    /// @input index [usize]: index of the character to handle in the input string
    /// @return [char]: required character
    fn get_char(&mut self, index: usize) -> char {
        if index < self.input_code.len() {
            self.input_code[index]
        } else {
            '\0'
        }
    }

    /// Lexer::read_string
    ///
    /// When a quote is found, the next text is to be handled as a string: read until another quote
    /// is found. If a new character (or EOF) is found before, then we have an error while
    /// tokenizing it
    ///
    /// @return [Option<String>]: parsed string
    fn read_string(&mut self) -> Option<String> {
        let mut str = String::from("");
        let mut next_char = self.input_code[self.current_index];

        // Until and exit condition is found
        loop {
            str.push(next_char);
            self.advance_index();
            next_char = self.input_code[self.current_index];
            if next_char == '\"' {
                return Some(str);
            }
            if next_char == '\n' || next_char == '\n' {
                return None;
            }
        }
    }

    /// Lexer::read_identifier
    ///
    /// Read until a non alphabetic character is found, and mark it as identifer
    ///
    /// @return [String]: parsed identifier
    fn read_identifier(&mut self) -> String {
        let mut str = String::from("");
        str.push(self.input_code[self.current_index]);

        loop {
            let next_char = self.input_code[self.current_index + 1];
            if next_char.is_alphanumeric() || next_char == '_' {
                self.advance_index();
                str.push(next_char);
            } else {
                return str;
            }
        }
    }

    /// Lexer::read_char
    ///
    /// Read character between two quotations
    ///
    /// @return [Option<char>]: parse char
    fn read_char(&mut self) -> Option<char> {
        // Error if there is a quotation followed by a new line
        if self.current_char == '\n' {
            self.lexer_error("Can't parse character".to_string());
            return None;
        }
        let chr = self.current_char;
        self.advance_index();

        // Erorr if there is not the closing in the expected position
        if self.current_char != '\'' {
            self.lexer_error("Can't parse character".to_string());
            return None;
        }

        return Some(chr);
    }

    /// Lexer::read_number
    ///
    /// Read a number
    ///
    /// @return [Option<char>]: parse char
    fn read_number(&mut self) -> Option<u64> {
        let mut str = String::from("");
        str.push(self.input_code[self.current_index]);

        loop {
            let next_char = self.input_code[self.current_index + 1];
            // Also allow b and x to represent binary and hexadecimal radix in the standard C
            // format
            if next_char.is_alphanumeric() {
                self.advance_index();
                str.push(next_char);
            } else {
                break;
            }
        }

        // all the parsing could lead to errors

        // If the string has an 'x', parse it as hexadecimal
        if str.contains("x") {
            let without_prefix = str.trim_start_matches("0x");
            match u64::from_str_radix(without_prefix, 16) {
                Ok(parsed_int) => return Some(parsed_int),
                _ => self.lexer_error(format!("Can't parse hexadecimal number {}", str).to_string()),
            };
        // If the string has an 'b', parse it as binary
        } else if str.contains("b") {
            let without_prefix = str.trim_start_matches("0b");
            match u64::from_str_radix(without_prefix, 2) {
                Ok(parsed_int) => return Some(parsed_int),
                _ => self.lexer_error(format!("Can't parse binary number {}", str).to_string()),
            };
        // If the string starts with 0, parse it as octal
        } else if str.chars().nth(0).unwrap() == '0' && str.len() != 1 {
            let without_prefix = str.trim_start_matches("0");
            match u64::from_str_radix(without_prefix, 8) {
                Ok(parsed_int) => return Some(parsed_int),
                _ => self.lexer_error(format!("Can't parse octal number {}", str).to_string()),
            };
        // Parse the number as decimal in all the other cases
        } else {
            match u64::from_str_radix(&str, 10) {
                Ok(parsed_int) => return Some(parsed_int),
                _ => self.lexer_error(format!("Can't parse decimal number {}", str).to_string()),
            };
        }

        return None;
    }

    /// Lexer::lexer_error
    ///
    /// Print an error on the error stream, by appending the line number
    ///
    /// @input error_str [String]: error message to print
    fn lexer_error(&mut self, error_str: String) {
        self.errors_counter += 1;
        if !self.is_file {
            return;
        }

        let line_number = self.current_line_number;
        let character_number = self.current_character_number;
        let file_lines = self.read_lines(&self.file_name);

        eprint!("\x1b[34m{}:{}:{}: \x1b[0m", self.file_name, line_number, self.current_first_character);
        eprintln!("\x1b[91merror lexer: \x1b[34m{}\x1b[0m", error_str);

        eprint!("{}\t| {}\n\t| ", line_number, file_lines[line_number as usize - 1]);

        for i in 0..character_number {
            if i < self.current_first_character - 1 {
                eprint!(" ");
            } else if i == self.current_first_character - 1 {
                eprint!("\x1b[91m^\x1b[0m");
            } else {
                eprint!("\x1b[91m~\x1b[0m");
            }
        }
        eprintln!("");
    }

    fn read_lines(&self, filename: &str) -> Vec<String> {
        let mut result = Vec::new();

        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }

        result
    }
}
