use std::fmt::{self, Display, Formatter};

/// Converts a type metavariable to a wildcard pattern.
macro_rules! wildcard {
    ($type:ty) => {
        _
    };
}

/// Defines the set of available [`Token`]s.
macro_rules! define_tokens {
    ([$((
        $name:ident $(($field:ty))?, $doc:literal, $description:literal $(,)?
    )),* $(,)?] $(,)?) => {
        #[doc = "A lexical element of source code."]
        #[derive(Debug)]
        pub enum Token {
            $(
                #[doc = $doc]
                $name $(
                    ($field)
                )?
            ),*
        }

        impl Token {
            #[doc = "Converts the `Token` to its [`TokenType`]."]
            pub fn as_type(&self) -> TokenType {
                match self {
                    $(
                        Self::$name $(
                            (wildcard!($field))
                        )? => TokenType::$name
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
            fn description(self) -> &'static str {
                match self {
                    $(
                        Self::$name => $description
                    ),*
                }
            }
        }
    };
}

define_tokens!([
    (Number(f64), "A number.", "a number"),
    (Ident(String), "An identifier.", "an identifier"),
    (OpenParen, "An opening parenthesis.", "an opening '('"),
    (CloseParen, "A closing parenthesis.", "a closing ')'"),
    (Comma, "A comma.", "','"),
    (Plus, "A plus sign.", "'+'"),
    (Minus, "A minus sign.", "'-'"),
    (Star, "An asterisk.", "'*'"),
    (Slash, "A forward slash.", "'/'"),
    (Eof, "An end of source code marker.", "end of file"),
]);

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => write!(f, "number '{value}'"),
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
