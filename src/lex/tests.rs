use super::*;

/// Asserts that source code produces an expected stream of [`Token`]s.
macro_rules! assert_tokens {
    ($src:literal, [$($tok:pat $(if $guard:expr)?),* $(,)?]) => {
        let mut lexer = Lexer::new($src);
        $(assert!(matches!(lexer.next_token(), $tok $(if $guard)?));)*
        assert!(matches!(lexer.next_token(), Ok(Token::Eof)));
    };
    ($src:literal, Ok[$($tok:pat $(if $guard:expr)?),* $(,)?]) => {
        assert_tokens!($src, [$(Ok($tok) $(if $guard)?),*]);
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
        "- >, ->, = =, ==, ! =, !=, < =, <=, > =, >=, & &, &&, | |, ||,",
        [
            Ok(Token::Minus),
            Ok(Token::Greater),
            Ok(Token::Comma),
            Ok(Token::MinusGreater),
            Ok(Token::Comma),
            Ok(Token::Equals),
            Ok(Token::Equals),
            Ok(Token::Comma),
            Ok(Token::EqualsEquals),
            Ok(Token::Comma),
            Ok(Token::Bang),
            Ok(Token::Equals),
            Ok(Token::Comma),
            Ok(Token::BangEquals),
            Ok(Token::Comma),
            Ok(Token::Less),
            Ok(Token::Equals),
            Ok(Token::Comma),
            Ok(Token::LessEquals),
            Ok(Token::Comma),
            Ok(Token::Greater),
            Ok(Token::Equals),
            Ok(Token::Comma),
            Ok(Token::GreaterEquals),
            Ok(Token::Comma),
            Err(LexError(ErrorKind::BitwiseAnd)),
            Err(LexError(ErrorKind::BitwiseAnd)),
            Ok(Token::Comma),
            Ok(Token::AndAnd),
            Ok(Token::Comma),
            Err(LexError(ErrorKind::BitwiseOr)),
            Err(LexError(ErrorKind::BitwiseOr)),
            Ok(Token::Comma),
            Ok(Token::PipePipe),
            Ok(Token::Comma),
        ]
    );
}

/// Tests that non-ASCII [`char`]s are scanned.
#[test]
fn non_ascii_chars_are_scanned() {
    assert_tokens!(
        "(Café ☕!)(🦀💻🧮)",
        [
            Ok(Token::OpenParen),
            Ok(Token::Ident(s)) if s.to_string() == "Caf",
            Err(LexError(ErrorKind::UnexpectedChar('é'))),
            Err(LexError(ErrorKind::UnexpectedChar('☕'))),
            Ok(Token::Bang),
            Ok(Token::CloseParen),
            Ok(Token::OpenParen),
            Err(LexError(ErrorKind::UnexpectedChar('🦀'))),
            Err(LexError(ErrorKind::UnexpectedChar('💻'))),
            Err(LexError(ErrorKind::UnexpectedChar('🧮'))),
            Ok(Token::CloseParen),
        ]
    );
}

/// Tests that source code produces trailing EOF [`Token`]s.
#[test]
fn trailing_eof_tokens_are_produced() {
    let mut lexer = Lexer::new("1 2 3");
    assert!(matches!(
        lexer.next_token(),
        Ok(Token::Literal(Literal::Number(1.0))),
    ));

    assert!(matches!(
        lexer.next_token(),
        Ok(Token::Literal(Literal::Number(2.0))),
    ));

    assert!(matches!(
        lexer.next_token(),
        Ok(Token::Literal(Literal::Number(3.0))),
    ));

    for _ in 0..16 {
        assert!(matches!(lexer.next_token(), Ok(Token::Eof)));
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
            Token::EqualsEquals,
            Token::Bang,
            Token::OpenBrace,
            Token::Ident(s) if s.to_string() == "foo",
            Token::MinusGreater,
            Token::Ident(s) if s.to_string() == "_B4R",
            Token::Equals,
            Token::Ident(s) if s.to_string() == "baz",
            Token::Comma,
            Token::Literal(Literal::Bool(true)),
            Token::BangEquals,
            Token::Literal(Literal::Bool(false)),
            Token::CloseBrace,
            Token::Ident(s) if s.to_string() == "min",
            Token::LessEquals,
            Token::Ident(s) if s.to_string() == "mid",
            Token::Less,
            Token::Ident(s) if s.to_string() == "max",
            Token::Greater,
            Token::Literal(Literal::Number(2.0)),
            Token::GreaterEquals,
            Token::Literal(Literal::Number(1.0)),
        ]
    );

    assert_tokens!(
        "x ^ 2",
        Ok[
            Token::Ident(s) if s.to_string() == "x",
            Token::Caret,
            Token::Literal(Literal::Number(2.0)),
        ]
    );

    assert_tokens!(
        "foo && bar || baz",
        Ok[
            Token::Ident(s) if s.to_string() == "foo",
            Token::AndAnd,
            Token::Ident(s) if s.to_string() == "bar",
            Token::PipePipe,
            Token::Ident(s) if s.to_string() == "baz",
        ]
    );

    assert_tokens!(
        "abs(n) = n < 0 ? -n : n",
        Ok[
            Token::Ident(s) if s.to_string() == "abs",
            Token::OpenParen,
            Token::Ident(s) if s.to_string() == "n",
            Token::CloseParen,
            Token::Equals,
            Token::Ident(s) if s.to_string() == "n",
            Token::Less,
            Token::Literal(Literal::Number(0.0)),
            Token::Question,
            Token::Minus,
            Token::Ident(s) if s.to_string() == "n",
            Token::Colon,
            Token::Ident(s) if s.to_string() == "n",
        ]
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
            Token::Ident(s) if s.to_string() == "_000",
            Token::Comma,
            Token::Literal(Literal::Number(0.0)),
            Token::Ident(s) if s.to_string() == "b1010",
            Token::Comma,
            Token::Literal(Literal::Number(0.0)),
            Token::Ident(s) if s.to_string() == "o10",
            Token::Comma,
            Token::Literal(Literal::Number(0.0)),
            Token::Ident(s) if s.to_string() == "xff",
            Token::Comma,
        ]
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
            Err(LexError(ErrorKind::UnexpectedChar('.'))),
            Ok(Token::Literal(Literal::Number(5.0))),
            Ok(Token::Comma),
            Ok(Token::Literal(Literal::Number(0.03125))),
            Ok(Token::Comma),
            Err(LexError(ErrorKind::UnexpectedChar('.'))),
            Ok(Token::Comma),
        ]
    );
}

/// Tests that decimal number [`Token`]s are parsed accurately.
#[test]
fn decimal_tokens_are_accurate() {
    use std::f64::consts::PI;

    // Test pi as it is written in the standard library.
    assert_tokens!(
        "3.14159265358979323846264338327950288",
        Ok[Token::Literal(Literal::Number(PI))]
    );

    // Test pi with more decimal places than can be represented.
    assert_tokens!(
        "3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628035",
        Ok[Token::Literal(Literal::Number(PI))]
    );
}

/// Tests that keyword [`Token`]s are length-sensitive.
#[test]
fn keywords_are_length_sensitive() {
    assert_tokens!(
        "f, fals, false, false_, falsetto,",
        Ok[
            Token::Ident(s) if s.to_string() == "f",
            Token::Comma,
            Token::Ident(s) if s.to_string() == "fals",
            Token::Comma,
            Token::Literal(Literal::Bool(false)),
            Token::Comma,
            Token::Ident(s) if s.to_string() == "false_",
            Token::Comma,
            Token::Ident(s) if s.to_string() == "falsetto",
            Token::Comma,
        ]
    );

    assert_tokens!(
        "t, tru, true, true_, truest,",
        Ok[
            Token::Ident(s) if s.to_string() == "t",
            Token::Comma,
            Token::Ident(s) if s.to_string() == "tru",
            Token::Comma,
            Token::Literal(Literal::Bool(true)),
            Token::Comma,
            Token::Ident(s) if s.to_string() == "true_",
            Token::Comma,
            Token::Ident(s) if s.to_string() == "truest",
            Token::Comma,
        ]
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
            Ok(Token::Ident(s)) if s.to_string() == "False",
            Ok(Token::Comma),
            Ok(Token::Ident(s)) if s.to_string() == "FALSE",
            Ok(Token::Comma),
            Ok(Token::Ident(s)) if s.to_string() == "f",
            Err(LexError(ErrorKind::UnexpectedChar('á'))),
            Ok(Token::Ident(s)) if s.to_string() == "lse",
            Ok(Token::Comma),
        ]
    );

    assert_tokens!(
        "true, True, TRUE, trüe,",
        [
            Ok(Token::Literal(Literal::Bool(true))),
            Ok(Token::Comma),
            Ok(Token::Ident(s)) if s.to_string() == "True",
            Ok(Token::Comma),
            Ok(Token::Ident(s)) if s.to_string() == "TRUE",
            Ok(Token::Comma),
            Ok(Token::Ident(s)) if s.to_string() == "tr",
            Err(LexError(ErrorKind::UnexpectedChar('ü'))),
            Ok(Token::Ident(s)) if s.to_string() == "e",
            Ok(Token::Comma),
        ]
    );
}

/// Tests that [`Symbol`]s are reused for equal names and are case-sensitive.
#[test]
fn symbols_are_reused_and_case_sensitive() {
    let mut lexer = Lexer::new("foo foo FOO FOO bar");

    /// Returns the next [`Symbol`] from the [`Lexer`].
    macro_rules! next_symbol {
        () => {{
            let Ok(Token::Ident(symbol)) = lexer.next_token() else {
                unreachable!("token should be an identifier");
            };

            symbol
        }};
    }

    let lower_symbol = next_symbol!();
    assert_eq!(lower_symbol.to_string(), "foo");
    assert_eq!(lower_symbol, next_symbol!());

    let upper_symbol = next_symbol!();
    assert_eq!(upper_symbol.to_string(), "FOO");
    assert_eq!(upper_symbol, next_symbol!());

    assert_ne!(lower_symbol, upper_symbol);

    let other_symbol = next_symbol!();
    assert_eq!(other_symbol.to_string(), "bar");
    assert_ne!(other_symbol, lower_symbol);
    assert_ne!(other_symbol, upper_symbol);

    assert!(matches!(lexer.next_token(), Ok(Token::Eof)));
}
