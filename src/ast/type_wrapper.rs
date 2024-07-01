use crate::lexer::token::{Keyword, Tk, Token};

/// TypeNative
///
/// Possible list of native types in the language
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TypeNative {
    U32,
    U16,
    U8,
    I32,
    I16,
    I8,
    Void,
    #[default]
    Null,
}

/// TypeWrapper
///
/// Wrapper for a type native in order to inclue all the information related to the type: number of
/// pointers and constant value
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeWrapper {
    pub type_native: TypeNative,
    pub pointer: u32,
    pub constant: bool,
}

impl TypeNative {
    /// TypeNative::from_token
    ///
    /// Create a type native object starting from a valid token
    ///
    /// @in tk[&Token] Token to use
    /// @return [TypeNative] Result
    pub fn from_token(tk: &Token) -> TypeNative {
        match &tk.tk {
            Tk::Keyword(k) => match k {
                Keyword::U8 => return TypeNative::U8,
                Keyword::U16 => return TypeNative::U16,
                Keyword::U32 => return TypeNative::U32,
                Keyword::I8 => return TypeNative::I8,
                Keyword::I16 => return TypeNative::I16,
                Keyword::I32 => return TypeNative::I32,
                Keyword::Void => return TypeNative::Void,
                _ => panic!("Cannot create type from non-type keyword"),
            },
            _ => {
                panic!("Cannot create type from token which is not Keyword")
            }
        }
    }
}

impl TypeWrapper {
    /// TypeWrapper::to_string
    ///
    /// Print the type with the const keyword, type and pointers
    ///
    /// @return [String] Result
    pub fn to_string(&self) -> String {
        let mut result = String::from("");

        if self.constant {
            result += "const ";
        }

        match &self.type_native {
            TypeNative::U32 => result += "u32",
            TypeNative::U16 => result += "u16",
            TypeNative::U8 => result += "u8",
            TypeNative::I32 => result += "i32",
            TypeNative::I16 => result += "i16",
            TypeNative::I8 => result += "i8",
            TypeNative::Void => result += "void",
            TypeNative::Null => result += "null",
        };

        for _ in 0..self.pointer {
            result += "*";
        }

        result
    }

    /// TypeWrapper::get_size
    ///
    /// Get size in bytes of a give type
    ///
    /// @return [u32] size
    pub fn get_size(&self) -> u32 {
        if self.pointer != 0 {
            return 4;
        }
        match &self.type_native {
            TypeNative::U8 | TypeNative::I8 => 1,
            TypeNative::U16 | TypeNative::I16 => 2,
            TypeNative::U32 | TypeNative::I32 => 4,
            _ => panic!("Cannot get size of non-sized type"),
        }
    }

    /// TypeWrapper::are_compatible
    ///
    /// Check whether two types are compatible or not. This function can be expanded in order to
    /// allow automatic casting
    ///
    /// @in a [&TypeWrapper]: first type
    /// @in b [&TypeWrapper]: second type
    /// @return [bool] result of the comparison
    pub fn are_compatible(a: &TypeWrapper, b: &TypeWrapper) -> bool {
        // Both type and number of pointers must be identical
        if a.pointer != b.pointer || a.type_native != b.type_native {
            return false;
        }
        return true;
    }
}

impl Default for TypeWrapper {
    /// TypeWrapper::default
    ///
    /// Creates a new default TypeWrapper
    fn default() -> TypeWrapper {
        TypeWrapper {
            constant: false,
            type_native: TypeNative::Null,
            pointer: 0,
        }
    }
}
