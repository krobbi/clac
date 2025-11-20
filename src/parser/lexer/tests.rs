use super::*;

/// Asserts that source code produces an expected stream of [`Token`]s.
macro_rules! assert_tokens {
    ($source:literal, [$($result:pat $(if $guard:expr)?),* $(,)?] $(,)?) => {
        let mut lexer = Lexer::new($source);

        $(
            assert!(matches!(lexer.bump(), $result $(if $guard)?));
        )*

        assert!(matches!(lexer.bump(), Ok(Token::Eof)));
    };
    ($source:literal, Ok[$($token:pat $(if $guard:expr)?),* $(,)?] $(,)?) => {
        assert_tokens!($source, [$(Ok($token) $(if $guard)?),*]);
    };
}

/// Tests that empty source code does not produce any [`Token`]s.
#[test]
fn empty_source_code_produces_no_tokens() {
    assert_tokens!("", Ok[]);
}

/// Tests that whitespace does not produce any [`Token`]s.
#[test]
fn whitespace_produces_no_tokens() {
    assert_tokens!(" \r\n\t ", Ok[]);
}

/// Tests that whitespace separates digraph [`Token`]s.
#[test]
fn whitespace_separates_digraphs() {
    assert_tokens!(
        "- >, ->",
        [
            Ok(Token::Minus),
            Err(LexError::UnexpectedChar('>')),
            Ok(Token::Comma),
            Ok(Token::RightArrow),
        ],
    );
}

/// Tests that non-ASCII [`char`]s are scanned.
#[test]
fn non_ascii_chars_are_scanned() {
    assert_tokens!(
        "(Café ☕!)(🦀💻🧮)",
        [
            Ok(Token::OpenParen),
            Ok(Token::Ident(n)) if n == "Caf",
            Err(LexError::UnexpectedChar('é')),
            Err(LexError::UnexpectedChar('☕')),
            Err(LexError::UnexpectedChar('!')),
            Ok(Token::CloseParen),
            Ok(Token::OpenParen),
            Err(LexError::UnexpectedChar('🦀')),
            Err(LexError::UnexpectedChar('💻')),
            Err(LexError::UnexpectedChar('🧮')),
            Ok(Token::CloseParen),
        ],
    );
}

/// Tests that source code produces trailing EOF [`Token`]s.
#[test]
fn trailing_eof_tokens_are_produced() {
    let mut lexer = Lexer::new("1 2 3");
    assert!(matches!(lexer.bump(), Ok(Token::Number(1.0))));
    assert!(matches!(lexer.bump(), Ok(Token::Number(2.0))));
    assert!(matches!(lexer.bump(), Ok(Token::Number(3.0))));

    for _ in 0..16 {
        assert!(matches!(lexer.bump(), Ok(Token::Eof)));
    }
}

/// Tests that all [`Token`]s can be produced.
#[test]
fn all_tokens_are_produced() {
    assert_tokens!(
        "-(1 + 2.5) * 3. / 4, 123.0, {life -> 42, _F00 == life()},",
        Ok[
            Token::Minus,
            Token::OpenParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.5),
            Token::CloseParen,
            Token::Star,
            Token::Number(3.0),
            Token::Slash,
            Token::Number(4.0),
            Token::Comma,

            Token::Number(123.0),
            Token::Comma,

            Token::OpenBrace,
            Token::Ident(n) if n == "life",
            Token::RightArrow,
            Token::Number(42.0),
            Token::Comma,

            Token::Ident(n) if n == "_F00",
            Token::Eq,
            Token::Eq,
            Token::Ident(n) if n == "life",
            Token::OpenParen,
            Token::CloseParen,
            Token::CloseBrace,
            Token::Comma,
        ],
    );
}

/// Tests that integer number [`Token`]s are produced.
#[test]
fn integers_are_produced() {
    assert_tokens!(
        "0, -1, 002, 300, 00400, 5_000, 0b1010, 0o10, 0xff,",
        Ok[
            Token::Number(0.0),
            Token::Comma,

            Token::Minus,
            Token::Number(1.0),
            Token::Comma,

            Token::Number(2.0),
            Token::Comma,

            Token::Number(300.0),
            Token::Comma,

            Token::Number(400.0),
            Token::Comma,

            Token::Number(5.0),
            Token::Ident(n) if n == "_000",
            Token::Comma,

            Token::Number(0.0),
            Token::Ident(n) if n == "b1010",
            Token::Comma,

            Token::Number(0.0),
            Token::Ident(n) if n == "o10",
            Token::Comma,

            Token::Number(0.0),
            Token::Ident(n) if n == "xff",
            Token::Comma,
        ],
    );
}

/// Tests that decimal number [`Token`]s are produced.
#[test]
fn decimals_are_produced() {
    assert_tokens!(
        "0.0, 1., -2.5, 00300.12500, 4.0625, .5, 0.03125, .,",
        [
            Ok(Token::Number(0.0)),
            Ok(Token::Comma),
            Ok(Token::Number(1.0)),
            Ok(Token::Comma),
            Ok(Token::Minus),
            Ok(Token::Number(2.5)),
            Ok(Token::Comma),
            Ok(Token::Number(300.125)),
            Ok(Token::Comma),
            Ok(Token::Number(4.0625)),
            Ok(Token::Comma),
            Err(LexError::UnexpectedChar('.')),
            Ok(Token::Number(5.0)),
            Ok(Token::Comma),
            Ok(Token::Number(0.03125)),
            Ok(Token::Comma),
            Err(LexError::UnexpectedChar('.')),
            Ok(Token::Comma),
        ],
    );
}

/// Tests that decimal number [`Token`]s are parsed accurately.
#[test]
fn decimals_are_accurate() {
    use std::f64::consts::PI;

    // Test pi as it is written in the standard library.
    assert_tokens!(
        "3.14159265358979323846264338327950288",
        Ok[Token::Number(PI)],
    );

    // Test pi with more decimal places than can be represented.
    assert_tokens!(
        "3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628035",
        Ok[Token::Number(PI)],
    );
}
