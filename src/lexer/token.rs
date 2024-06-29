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
