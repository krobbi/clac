use super::*;

/// Asserts that an expected [`ParseError`] is produced from source code.
macro_rules! assert_error {
    ($source:literal, $error:pat $(,)?) => {
        // Can't use `Result::expect_err` because `Ast` does not implement
        // `Debug`.
        let Err(error) = parse_source($source) else {
            panic!("source code should be invalid");
        };

        assert!(matches!(error, $error));
    };
}

/// Tests that empty source code is parsed as an empty AST.
#[test]
fn empty_source() {
    assert_ast("", "(a:)");
}

/// Tests that blocks can be empty.
#[test]
fn empty_blocks() {
    assert_ast("{}", "(a: (b:))");
}

/// Tests that parenthesized expressions can be nested.
#[test]
fn nested_parens() {
    assert_ast("0, (1), ((2))", "(a: 0 (p: 1) (p: (p: 2)))");
}

/// Tests that blocks can be nested.
#[test]
fn nested_blocks() {
    assert_ast("0, {1}, {{2}}", "(a: 0 (b: 1) (b: (b: 2)))");
}

/// Tests that parenthesized expressions and blocks can be nested inside each
/// other.
#[test]
fn mixed_nesting() {
    assert_ast("{({})}", "(a: (b: (p: (b:))))");
}

/// Tests operator associativity.
#[test]
fn associativity() {
    assert_ast("---1", "(a: (- (- (- 1))))");
    assert_ast("1 + 2 + 3", "(a: (+ (+ 1 2) 3))");
    assert_ast("4 - 5 - 6", "(a: (- (- 4 5) 6))");
    assert_ast("7 * 8 * 9", "(a: (* (* 7 8) 9))");
    assert_ast("a / b / c", "(a: (/ (/ a b) c))");
    assert_ast("f(1)(2)(3)", "(a: (((f 1) 2) 3))");
}

/// Tests operator precedence levels.
#[test]
fn precedence_levels() {
    // The precedence of `+` is equal to `-`.
    assert_ast("1 + 2 - 3", "(a: (- (+ 1 2) 3))");
    assert_ast("1 - 2 + 3", "(a: (+ (- 1 2) 3))");

    // The precedence of `*` is equal to `/`.
    assert_ast("1 * 2 / 3", "(a: (/ (* 1 2) 3))");
    assert_ast("1 / 2 * 3", "(a: (* (/ 1 2) 3))");

    // The precedence of `*` and `-` is higher than `+` and `-`.
    assert_ast("1 + 2 * 3", "(a: (+ 1 (* 2 3)))");
    assert_ast("1 + 2 * 3 + 4", "(a: (+ (+ 1 (* 2 3)) 4))");

    // Precedence can be overridden with parentheses.
    assert_ast("(1 + 2) * 3", "(a: (* (p: (+ 1 2)) 3))");
}

/// Tests the precedence of the unary negation operator.
#[test]
fn negation_precedence() {
    assert_ast("-1 * x", "(a: (* (- 1) x))");
    assert_ast("1 -1", "(a: (- 1 1))");
    assert_ast("1, -1", "(a: 1 (- 1))");
    assert_ast("-f(x)", "(a: (- (f x)))");
    assert_ast("-f(x)(y)", "(a: (- ((f x) y)))");
}

/// Tests that assignments are parsed as expected.
#[test]
fn assignments() {
    assert_ast("n = 10", "(a: (= n 10))");
    assert_ast("f(x) = x * x", "(a: (= (f x) (* x x)))");

    // Nonsensical assignments may be parsed, but should be checked later.
    assert_ast("1 + x = 2", "(a: (= (+ 1 x) 2))");
}

/// Tests that blocks may contain statements.
#[test]
fn block_stmts() {
    assert_ast("1 + {x = 2, x}", "(a: (+ 1 (b: (= x 2) x)))");
}

/// Tests that commas between statements are optional and may be trailing.
#[test]
fn sequence_commas() {
    assert_error!(", 1", ParseError::ExpectedExpr(Token::Comma));
    assert_ast("1 2 3", "(a: 1 2 3)");
    assert_ast("1 2 3,", "(a: 1 2 3)");
    assert_ast("1, 2, 3", "(a: 1 2 3)");
    assert_ast("1, 2, 3,", "(a: 1 2 3)");
    assert_ast("{1 2 3}", "(a: (b: 1 2 3))");
    assert_ast("{1 2 3,}", "(a: (b: 1 2 3))");
    assert_ast("{1, 2, 3}", "(a: (b: 1 2 3))");
    assert_ast("{1, 2, 3,},", "(a: (b: 1 2 3))");
}

/// Tests that commas between expressions are required, but may optionally be
/// trailing.
#[test]
fn tuple_commas() {
    assert_ast("f()", "(a: (f))");
    assert_error!("f(,)", ParseError::ExpectedExpr(Token::Comma));
    assert_ast("f(1)", "(a: (f 1))");
    assert_ast("f(1,)", "(a: (f 1))");
    assert_error!(
        "f(1 2)",
        ParseError::UnexpectedToken(TokenType::CloseParen, Token::Number(2.0))
    );

    assert_ast("f(1, 2)", "(a: (f 1 2))");
    assert_ast("f(1, 2,)", "(a: (f 1 2))");
}

/// Tests that unexpected character [`LexError`]s are encapsulated in
/// [`ParseError`]s.
#[test]
fn unexpected_chars() {
    assert_error!("foo + $bar", ParseError::Lex(LexError::UnexpectedChar('$')));
}

/// Tests that leading plus signs are not supported.
#[test]
fn plus_signs() {
    assert_error!("+1", ParseError::ExpectedExpr(Token::Plus));
}

/// Tests that tuple values are not supported.
#[test]
fn tuples() {
    assert_error!("()", ParseError::TupleValue);
    assert_error!("(,)", ParseError::ExpectedExpr(Token::Comma));
    assert_error!("(1,)", ParseError::TupleValue);
    assert_error!("(x, y)", ParseError::TupleValue);
    assert_error!("(u, v,)", ParseError::TupleValue);
    assert_error!("(r, g, b)", ParseError::TupleValue);
    assert_error!("(h, s, v,)", ParseError::TupleValue);
}

/// Tests that assignment cannot be used as an expression.
#[test]
fn assignment_exprs() {
    assert_error!("x = y = 0", ParseError::ChainedAssignment);
    assert_error!(
        "1 + (x = 2)",
        ParseError::UnexpectedToken(TokenType::CloseParen, Token::Eq)
    );
}

/// Asserts that an expected [`Ast`] is parsed from source code.
fn assert_ast(source: &str, expected: &str) {
    let ast = parse_source(source).expect("source code should be valid");
    assert_eq!(ast.to_string(), expected);
}
