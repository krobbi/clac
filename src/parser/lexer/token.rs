use std::fmt::{self, Display, Formatter};

use crate::ast::Literal;

/// Converts a type metavariable to a wildcard pattern.
macro_rules! wildcard {
    ($type:ty) => {
        _
    };
}

/// Defines the set of available [`Token`]s.
macro_rules! define_tokens {
    {$(($name:ident $(($field:ty))?, $doc:literal, $description:literal)),* $(,)?} => {
        #[doc = "A lexical element of source code."]
        #[derive(Debug)]
        pub enum Token {
            $(
                #[doc = $doc]
                $name $(($field))?
            ),*
        }

        impl Token {
            #[doc = "Converts the `Token` to its [`TokenType`]."]
            pub const fn as_type(&self) -> TokenType {
                match self {
                    $(
                        Self::$name $((wildcard!($field)))? => TokenType::$name
                    ),*
                }
            }
        }

        #[doc = "A [`Token`]'s type."]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum TokenType {
            $(
                #[doc = $doc]
                $name
            ),*
        }

        impl TokenType {
            #[doc = "Returns a description of the `TokenType`."]
            const fn description(self) -> &'static str {
                match self {
                    $(
                        Self::$name => $description
                    ),*
                }
            }
        }
    };
}

define_tokens! {
    (Literal(Literal), "A [`Literal`].", "a literal"),
    (Ident(String), "An identifier.", "an identifier"),
    (OpenParen, "An opening parenthesis (`(`).", "an opening '('"),
    (CloseParen, "A closing parenthesis (`)`).", "a closing ')'"),
    (OpenBrace, "An opening brace (`{`).", "an opening '{'"),
    (CloseBrace, "A closing brace (`}`).", "a closing '}'"),
    (Comma, "A comma (`,`).", "','"),
    (Plus, "A plus sign (`+`).", "'+'"),
    (Minus, "A minus sign (`-`).", "'-'"),
    (Star, "An asterisk (`*`).", "'*'"),
    (Slash, "A forward slash (`/`).", "'/'"),
    (Caret, "A caret (`^`).", "'^'"),
    (Eq, "An equals sign (`=`).", "'='"),
    (EqEq, "A double equals sign (`==`).", "'=='"),
    (Bang, "An exclamation mark (`!`).", "'!'"),
    (BangEq, "An exclamation mark and equals sign (`!=`).", "'!='"),
    (Lt, "A less than symbol (`<`).", "'<'"),
    (LtEq, "A less than symbol and equals sign (`<=`).", "'<='"),
    (Gt, "A greater than symbol (`>`).", "'>'"),
    (GtEq, "A greater than symbol and equals sign (`>=`).", "'>='"),
    (AndAnd, "A double ampersand (`&&`).", "'&&'"),
    (PipePipe, "A double pipe (`||`).", "'||'"),
    (Question, "A question mark (`?`).", "'?'"),
    (Colon, "A colon (`:`).", "':'"),
    (RightArrow, "A right arrow (`->`).", "'->'"),
    (Eof, "An end of source code marker.", "end of file"),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => match literal {
                Literal::Number(value) => write!(f, "number '{value}'"),
                Literal::Bool(value) => write!(f, "bool '{value}'"),
            },
            Self::Ident(name) => write!(f, "identifier '{name}'"),
            _ => self.as_type().fmt(f),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}
