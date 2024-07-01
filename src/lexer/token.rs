use std::fmt;

/// enum Keyword
///
/// Enum associated to the keywords in the language
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Keyword {
    Return,
    If,
    Else,
    While,
    For,
    And,
    Or,
    True,
    Const,
    False,
    Continue,
    Break,
    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
    Void,
}

impl Keyword {
    //! Keyword::from
    //!
    //! @in [&str]: string to parse
    //! @return [Option<Keyword>]: given a string, either return the enum value associated to it,
    //! or None if it was not recognized
    pub fn from(input: &str) -> Option<Keyword> {
        match input {
            "return" => Some(Keyword::Return),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "for" => Some(Keyword::For),
            "and" => Some(Keyword::And),
            "or" => Some(Keyword::Or),
            "true" => Some(Keyword::True),
            "const" => Some(Keyword::Const),
            "false" => Some(Keyword::False),
            "continue" => Some(Keyword::Continue),
            "break" => Some(Keyword::Break),
            "u8" => Some(Keyword::U8),
            "u16" => Some(Keyword::U16),
            "u32" | "int" => Some(Keyword::U32),
            "i8" | "char" => Some(Keyword::I8),
            "i16" => Some(Keyword::I16),
            "i32" => Some(Keyword::I32),
            "void" => Some(Keyword::Void),
            _ => None,
        }
    }
}

/// enum Bracket
///
/// Enum associated to the brackets in the language
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Bracket {
    LCurly,
    RCurly,
    LSquare,
    RSquare,
    LBracket,
    RBracket,
}

impl Bracket {
    //! Bracket::from
    //!
    //! @in [&str]: string to parse
    //! @return [Option<Bracket>]: given a string, either return the enum value associated to it,
    //! or None if it was not recognized
    pub fn from(input: &str) -> Option<Bracket> {
        match input {
            "{" => Some(Bracket::LCurly),
            "(" => Some(Bracket::LBracket),
            "[" => Some(Bracket::LSquare),
            "}" => Some(Bracket::RCurly),
            ")" => Some(Bracket::RBracket),
            "]" => Some(Bracket::RSquare),
            _ => None,
        }
    }
}

/// enum Operator
///
/// Enum associated to the operators in the language
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Assign,
    EqualCompare,
    DiffCompare,
    LTCompare,
    GTCompare,
    LECompare,
    GECompare,
    Minus,
    Plus,
    Asterisk,
    Slash,
    XorOp,
    AndOp,
    OrOp,
    Not,
    Complement,
    Module,
    LShift,
    RShift,
    Comma,
}

impl Operator {
    //! Operator::from
    //!
    //! @in [&str]: string to parse
    //! @return [Option<Bracket>]: given a string, either return the enum value associated to it,
    //! or None if it was not recognized
    pub fn from(input: &str) -> Option<Operator> {
        match input {
            "=" => Some(Operator::Assign),
            "," => Some(Operator::Comma),
            "==" => Some(Operator::EqualCompare),
            "!=" => Some(Operator::DiffCompare),
            "<" => Some(Operator::LTCompare),
            ">" => Some(Operator::GTCompare),
            "<=" => Some(Operator::LECompare),
            ">=" => Some(Operator::GECompare),
            "-" => Some(Operator::Minus),
            "+" => Some(Operator::Plus),
            "*" => Some(Operator::Asterisk),
            "/" => Some(Operator::Slash),
            "^" => Some(Operator::XorOp),
            "&" => Some(Operator::AndOp),
            "~" => Some(Operator::Complement),
            "!" => Some(Operator::Not),
            "|" => Some(Operator::OrOp),
            "%" => Some(Operator::Module),
            "<<" => Some(Operator::LShift),
            ">>" => Some(Operator::RShift),
            _ => None,
        }
    }
}

/// enum Tk
///
/// Enum associated to the tokens in the language
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tk {
    Bracket(Bracket),
    Keyword(Keyword),
    Semicolon,
    Operator(Operator),
    Identifier(String),
    IntegerLiteral(u64),
    String(String),
    Char(char),
    EOF,
    ERROR,
}

/// enum Tk
///
/// struct associated to the tokens in the language, which will be sent to the parse
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub tk: Tk,
    pub line_number: u32,
    pub last_character: u32,
    pub first_character: u32,
}

impl Tk {
    /// Tk::get_identifier
    ///
    /// Return the identifier contained in a token. Error if the token is not of type Identifier
    ///
    /// @return [String]: result of the extraction
    pub fn get_identifier(&self) -> String {
        if let Tk::Identifier(s) = &self {
            return s.clone();
        }
        panic!("Cannot extract identifier from non-identifier token: {:#?}", self);
    }

    /// Tk::get_operator
    ///
    /// Return the operator contained in a token. Error if the token is not of type operator
    ///
    /// @return [String]: result of the extraction
    pub fn get_operator(&self) -> Operator {
        if let Tk::Operator(o) = &self {
            return o.clone();
        }
        panic!("Cannot extract operator from non-identifier token: {:#?}", self);
    }

    /// Tk::to_string
    ///
    /// Convert to string
    ///
    /// @return [String]: conversion
    pub fn to_string(&self) -> String {
        return match self {
            Tk::Bracket(br) => match br {
                Bracket::LCurly => "{".to_string(),
                Bracket::RCurly => "}".to_string(),
                Bracket::LSquare => "[".to_string(),
                Bracket::RSquare => "]".to_string(),
                Bracket::LBracket => "(".to_string(),
                Bracket::RBracket => ")".to_string(),
            },
            Tk::Keyword(kw) => match kw {
                Keyword::Const => "const".to_string(),
                Keyword::Void => "void".to_string(),
                Keyword::Return => "return".to_string(),
                Keyword::If => "if".to_string(),
                Keyword::Else => "else".to_string(),
                Keyword::While => "while".to_string(),
                Keyword::For => "for".to_string(),
                Keyword::And => "and".to_string(),
                Keyword::Or => "or".to_string(),
                Keyword::True => "true".to_string(),
                Keyword::False => "false".to_string(),
                Keyword::Continue => "continue".to_string(),
                Keyword::Break => "break".to_string(),
                Keyword::U8 => "u8".to_string(),
                Keyword::U16 => "u16".to_string(),
                Keyword::U32 => "u32".to_string(),
                Keyword::I8 => "i8".to_string(),
                Keyword::I16 => "i16".to_string(),
                Keyword::I32 => "i32".to_string(),
            },
            Tk::Operator(operator) => match operator {
                Operator::Assign => "=".to_string(),
                Operator::Comma => ",".to_string(),
                Operator::EqualCompare => "==".to_string(),
                Operator::DiffCompare => "!=".to_string(),
                Operator::LTCompare => "<".to_string(),
                Operator::GTCompare => ">".to_string(),
                Operator::LECompare => "<=".to_string(),
                Operator::GECompare => ">=".to_string(),
                Operator::Minus => "-".to_string(),
                Operator::Plus => "+".to_string(),
                Operator::Asterisk => "*".to_string(),
                Operator::Slash => "/".to_string(),
                Operator::XorOp => "^".to_string(),
                Operator::AndOp => "&".to_string(),
                Operator::Complement => "~".to_string(),
                Operator::Not => "!".to_string(),
                Operator::OrOp => "|".to_string(),
                Operator::Module => "%".to_string(),
                Operator::LShift => "<<".to_string(),
                Operator::RShift => ">>".to_string(),
            },
            Tk::Semicolon => ";".to_string(),
            Tk::Identifier(id) => id.to_string(),
            Tk::IntegerLiteral(num) => num.to_string(),
            Tk::String(str) => format!("\"{}\"", str).to_string(),
            Tk::Char(chr) => format!("\'{}\'", chr).to_string(),
            Tk::EOF => "EOF".to_string(),
            Tk::ERROR => "ERROR".to_string(),
        };
    }

    pub fn is_type(&self) -> bool {
        match self {
            Tk::Keyword(Keyword::U8)
            | Tk::Keyword(Keyword::I8)
            | Tk::Keyword(Keyword::U16)
            | Tk::Keyword(Keyword::I16)
            | Tk::Keyword(Keyword::U32)
            | Tk::Keyword(Keyword::I32)
            | Tk::Keyword(Keyword::Void) => {
                return true;
            }
            _ => return false,
        }
    }
}

impl fmt::Display for Tk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string().as_str())
    }
}
