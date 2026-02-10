use std::fmt::{self, Display, Formatter};

use crate::{ast::Literal, symbols::Symbol};

/// Defines the set of [`Token`]s.
macro_rules! define_tokens {
    {$(($name:ident$(($field:ty))?, $doc:literal, $desc:literal)),* $(,)?} => {
        /// A lexical element of source code.
        #[derive(Debug)]
        pub enum Token {$(
            #[doc = $doc]
            $name$(($field))?
        ),*}

        impl Token {
            /// Returns the `Token`'s [`TokenType`].
            pub const fn token_type(&self) -> TokenType {
                match self {$(
                    Self::$name { .. } => TokenType::$name
                ),*}
            }
        }

        /// A [`Token`]'s type.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum TokenType {$(
            #[doc = $doc]
            $name
        ),*}

        impl TokenType {
            /// Returns a description of the `TokenType`.
            const fn description(self) -> &'static str {
                match self {$(
                    Self::$name => $desc
                ),*}
            }
        }
    };
}

define_tokens! {
    (Eof, "An end of source code marker.", "end of file"),
    (Literal(Literal), "A [`Literal`].", "a literal"),
    (Ident(Symbol), "An identifier.", "an identifier"),
    (OpenParen, "An opening parenthesis (`(`).", "an opening '('"),
    (CloseParen, "A closing parenthesis (`)`).", "a closing ')'"),
    (OpenBrace, "An opening brace (`{`).", "an opening '{'"),
    (CloseBrace, "A closing brace (`}`).", "a closing '}'"),
    (Comma, "A comma (`,`).", "','"),
    (Plus, "A plus sign (`+`).", "'+'"),
    (Minus, "A minus sign (`-`).", "'-'"),
    (MinusGreater, "A minus sign and greater than symbol (`->`).", "'->'"),
    (Star, "An asterisk (`*`).", "'*'"),
    (Slash, "A forward slash (`/`).", "'/'"),
    (Caret, "A caret (`^`).", "'^'"),
    (Equals, "An equals sign (`=`).", "'='"),
    (EqualsEquals, "A double equals sign (`==`).", "'=='"),
    (Bang, "An exclamation mark (`!`).", "'!'"),
    (BangEquals, "An exclamation mark and equals sign (`!=`).", "'!='"),
    (Less, "A less than symbol (`<`).", "'<'"),
    (LessEquals, "A less than symbol and equals sign (`<=`).", "'<='"),
    (Greater, "A greater than symbol (`>`).", "'>'"),
    (GreaterEquals, "A greater than symbol and equals sign (`>=`).", "'>='"),
    (AndAnd, "A double ampersand (`&&`).", "'&&'"),
    (PipePipe, "A double pipe (`||`).", "'||'"),
    (Question, "A question mark (`?`).", "'?'"),
    (Colon, "A colon (`:`).", "':'"),
}

impl Literal {
    /// Returns the name of the `Literal`'s type.
    const fn type_name(&self) -> &'static str {
        match self {
            Self::Number(_) => "number",
            Self::Bool(_) => "bool",
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => {
                let type_name = literal.type_name();
                write!(f, "{type_name} '{literal}'")
            }
            Self::Ident(symbol) => write!(f, "identifier '{symbol}'"),
            _ => Display::fmt(&self.token_type(), f),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}
