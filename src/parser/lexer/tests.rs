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
fn whitespace_separates_digraph_tokens() {
    assert_tokens!(
        "- >, ->, = =, ==, ! =, !=, < =, <=, > =, >=",
        Ok[
            Token::Minus,
            Token::Gt,
            Token::Comma,
            Token::RightArrow,
            Token::Comma,
            Token::Eq,
            Token::Eq,
            Token::Comma,
            Token::EqEq,
            Token::Comma,
            Token::Bang,
            Token::Eq,
            Token::Comma,
            Token::BangEq,
            Token::Comma,
            Token::Lt,
            Token::Eq,
            Token::Comma,
            Token::LtEq,
            Token::Comma,
            Token::Gt,
            Token::Eq,
            Token::Comma,
            Token::GtEq,
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
            Ok(Token::Bang),
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
    assert!(matches!(
        lexer.bump(),
        Ok(Token::Literal(Literal::Number(1.0))),
    ));

    assert!(matches!(
        lexer.bump(),
        Ok(Token::Literal(Literal::Number(2.0))),
    ));

    assert!(matches!(
        lexer.bump(),
        Ok(Token::Literal(Literal::Number(3.0))),
    ));

    for _ in 0..16 {
        assert!(matches!(lexer.bump(), Ok(Token::Eof)));
    }
}

/// Tests that all [`Token`]s can be produced.
#[test]
fn all_tokens_are_produced() {
    assert_tokens!(
        "-(1 + 2.5) * 3. / 4 == !{foo -> _B4R = baz, true != false} min <= mid < max > 2 >= 1",
        Ok[
            Token::Minus,
            Token::OpenParen,
            Token::Literal(Literal::Number(1.0)),
            Token::Plus,
            Token::Literal(Literal::Number(2.5)),
            Token::CloseParen,
            Token::Star,
            Token::Literal(Literal::Number(3.0)),
            Token::Slash,
            Token::Literal(Literal::Number(4.0)),
            Token::EqEq,
            Token::Bang,
            Token::OpenBrace,
            Token::Ident(n) if n == "foo",
            Token::RightArrow,
            Token::Ident(n) if n == "_B4R",
            Token::Eq,
            Token::Ident(n) if n == "baz",
            Token::Comma,
            Token::Literal(Literal::Bool(true)),
            Token::BangEq,
            Token::Literal(Literal::Bool(false)),
            Token::CloseBrace,
            Token::Ident(n) if n == "min",
            Token::LtEq,
            Token::Ident(n) if n == "mid",
            Token::Lt,
            Token::Ident(n) if n == "max",
            Token::Gt,
            Token::Literal(Literal::Number(2.0)),
            Token::GtEq,
            Token::Literal(Literal::Number(1.0)),
        ],
    );

    assert_tokens!(
        "abs(n) = n < 0 ? -n : n",
        Ok[
            Token::Ident(n) if n == "abs",
            Token::OpenParen,
            Token::Ident(n) if n == "n",
            Token::CloseParen,
            Token::Eq,
            Token::Ident(n) if n == "n",
            Token::Lt,
            Token::Literal(Literal::Number(0.0)),
            Token::Question,
            Token::Minus,
            Token::Ident(n) if n == "n",
            Token::Colon,
            Token::Ident(n) if n == "n",
        ],
    );
}

/// Tests that integer number [`Token`]s are produced.
#[test]
fn integers_tokens_are_produced() {
    assert_tokens!(
        "0, -1, 002, 300, 00400, 5_000, 0b1010, 0o10, 0xff,",
        Ok[
            Token::Literal(Literal::Number(0.0)),
            Token::Comma,
            Token::Minus,
            Token::Literal(Literal::Number(1.0)),
            Token::Comma,
            Token::Literal(Literal::Number(2.0)),
            Token::Comma,
            Token::Literal(Literal::Number(300.0)),
            Token::Comma,
            Token::Literal(Literal::Number(400.0)),
            Token::Comma,
            Token::Literal(Literal::Number(5.0)),
            Token::Ident(n) if n == "_000",
            Token::Comma,
            Token::Literal(Literal::Number(0.0)),
            Token::Ident(n) if n == "b1010",
            Token::Comma,
            Token::Literal(Literal::Number(0.0)),
            Token::Ident(n) if n == "o10",
            Token::Comma,
            Token::Literal(Literal::Number(0.0)),
            Token::Ident(n) if n == "xff",
            Token::Comma,
        ],
    );
}

/// Tests that decimal number [`Token`]s are produced.
#[test]
fn decimal_tokens_are_produced() {
    assert_tokens!(
        "0.0, 1., -2.5, 00300.12500, 4.0625, .5, 0.03125, .,",
        [
            Ok(Token::Literal(Literal::Number(0.0))),
            Ok(Token::Comma),
            Ok(Token::Literal(Literal::Number(1.0))),
            Ok(Token::Comma),
            Ok(Token::Minus),
            Ok(Token::Literal(Literal::Number(2.5))),
            Ok(Token::Comma),
            Ok(Token::Literal(Literal::Number(300.125))),
            Ok(Token::Comma),
            Ok(Token::Literal(Literal::Number(4.0625))),
            Ok(Token::Comma),
            Err(LexError::UnexpectedChar('.')),
            Ok(Token::Literal(Literal::Number(5.0))),
            Ok(Token::Comma),
            Ok(Token::Literal(Literal::Number(0.03125))),
            Ok(Token::Comma),
            Err(LexError::UnexpectedChar('.')),
            Ok(Token::Comma),
        ],
    );
}

/// Tests that decimal number [`Token`]s are parsed accurately.
#[test]
fn decimal_tokens_are_accurate() {
    use std::f64::consts::PI;

    // Test pi as it is written in the standard library.
    assert_tokens!(
        "3.14159265358979323846264338327950288",
        Ok[Token::Literal(Literal::Number(PI))],
    );

    // Test pi with more decimal places than can be represented.
    assert_tokens!(
        "3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628035",
        Ok[Token::Literal(Literal::Number(PI))],
    );
}

/// Tests that keyword [`Token`]s are length-sensitive.
#[test]
fn keywords_are_length_sensitive() {
    assert_tokens!(
        "f, fals, false, false_, falsetto,",
        Ok[
            Token::Ident(n) if n == "f",
            Token::Comma,
            Token::Ident(n) if n == "fals",
            Token::Comma,
            Token::Literal(Literal::Bool(false)),
            Token::Comma,
            Token::Ident(n) if n == "false_",
            Token::Comma,
            Token::Ident(n) if n == "falsetto",
            Token::Comma,
        ],
    );

    assert_tokens!(
        "t, tru, true, true_, truest,",
        Ok[
            Token::Ident(n) if n == "t",
            Token::Comma,
            Token::Ident(n) if n == "tru",
            Token::Comma,
            Token::Literal(Literal::Bool(true)),
            Token::Comma,
            Token::Ident(n) if n == "true_",
            Token::Comma,
            Token::Ident(n) if n == "truest",
            Token::Comma,
        ],
    );
}

/// Tests that keyword [`Token`]s are case-sensitive.
#[test]
fn keywords_are_case_sensitive() {
    assert_tokens!(
        "false, False, FALSE, fálse,",
        [
            Ok(Token::Literal(Literal::Bool(false))),
            Ok(Token::Comma),
            Ok(Token::Ident(n)) if n == "False",
            Ok(Token::Comma),
            Ok(Token::Ident(n)) if n == "FALSE",
            Ok(Token::Comma),
            Ok(Token::Ident(n)) if n == "f",
            Err(LexError::UnexpectedChar('á')),
            Ok(Token::Ident(n)) if n == "lse",
            Ok(Token::Comma),
        ],
    );

    assert_tokens!(
        "true, True, TRUE, trüe,",
        [
            Ok(Token::Literal(Literal::Bool(true))),
            Ok(Token::Comma),
            Ok(Token::Ident(n)) if n == "True",
            Ok(Token::Comma),
            Ok(Token::Ident(n)) if n == "TRUE",
            Ok(Token::Comma),
            Ok(Token::Ident(n)) if n == "tr",
            Err(LexError::UnexpectedChar('ü')),
            Ok(Token::Ident(n)) if n == "e",
            Ok(Token::Comma),
        ],
    );
}
