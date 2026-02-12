use super::{lexer::LexError, *};

/// Asserts that an expected [`ErrorKind`] is produced from source code.
macro_rules! assert_error {
    ($src:literal, $err:pat $(if $guard:expr)?) => {
        let error_kind = *parse_source($src).expect_err("test source should be invalid").0;
        assert!(matches!(error_kind, $err $(if $guard)?));
    };
}

/// Tests that empty source code is parsed as an empty [`Ast`].
#[test]
fn empty_source_code_is_parsed() {
    assert_ast("", "(a:)");
}

/// Tests that assignments are parsed.
#[test]
fn assignments_are_parsed() {
    assert_ast("n = 10", "(a: (= n 10))");
    assert_ast("f(x) = x * x", "(a: (= (f x) (* x x)))");
}

/// Tests that assignments are not [`Expr`]s.
#[test]
fn assignments_are_not_expressions() {
    assert_error!("x = y = 0", ErrorKind::ChainedAssignment);
    assert_error!(
        "1 + (x = 2)",
        ErrorKind::UnexpectedToken(TokenType::CloseParen, Token::Equals)
    );
}

/// Tests that non-identifier bindings are not checked by the [`Parser`].
#[test]
fn non_identifier_bindings_are_unchecked() {
    assert_ast("1 + x = 2", "(a: (= (+ 1 x) 2))");
    assert_ast("3(4 + 5) = 6", "(a: (= (3 (+ 4 5)) 6))");
    assert_ast("(7, 8) -> 9", "(a: (-> 7 8 9))");
}

/// Tests that empty blocks are parsed.
#[test]
fn empty_blocks_are_parsed() {
    assert_ast("{}", "(a: (b:))");
}

/// Tests that blocks can contain [`Stmt`]s.
#[test]
fn blocks_can_contain_statements() {
    assert_ast("1 + {x = 2, x}", "(a: (+ 1 (b: (= x 2) x)))");
}

/// Tests that blocks can be nested.
#[test]
fn blocks_can_be_nested() {
    assert_ast("0, {1}, {{2}}", "(a: 0 (b: 1) (b: (b: 2)))");
}

/// Tests that commas between [`Stmt`]s are optional and may be trailing.
#[test]
fn sequence_commas_are_optional() {
    assert_error!(", 1", ErrorKind::ExpectedExpr(Token::Comma));
    assert_ast("1 2 3", "(a: 1 2 3)");
    assert_ast("1 2 3,", "(a: 1 2 3)");
    assert_ast("1, 2, 3", "(a: 1 2 3)");
    assert_ast("1, 2, 3,", "(a: 1 2 3)");
    assert_error!("{, 1}", ErrorKind::ExpectedExpr(Token::Comma));
    assert_ast("{1 2 3}", "(a: (b: 1 2 3))");
    assert_ast("{1 2 3,}", "(a: (b: 1 2 3))");
    assert_ast("{1, 2, 3}", "(a: (b: 1 2 3))");
    assert_ast("{1, 2, 3,},", "(a: (b: 1 2 3))");
}

/// Tests that parenthesized [`Expr`]s and tuples are parsed.
#[test]
fn parens_are_parsed() {
    assert_ast("()", "(a: (t:))");
    assert_error!("(,)", ErrorKind::ExpectedExpr(Token::Comma));
    assert_ast("(1)", "(a: (p: 1))");
    assert_ast("(2,)", "(a: (t: 2))");
    assert_ast("(x, y)", "(a: (t: x y))");
    assert_error!(
        "(z w)",
        ErrorKind::UnexpectedToken(TokenType::CloseParen, Token::Ident(s)) if s.to_string() == "w"
    );

    assert_ast("(u, v,)", "(a: (t: u v))");
    assert_ast("(r, g, b)", "(a: (t: r g b))");
    assert_ast("(h, s, v,)", "(a: (t: h s v))");
}

/// Tests that parenthesized [`Expr`]s and tuples can be nested.
#[test]
fn parens_can_be_nested() {
    assert_ast("0, (1), ((2))", "(a: 0 (p: 1) (p: (p: 2)))");
    assert_ast("((1, 2), (3,), ())", "(a: (t: (t: 1 2) (t: 3) (t:)))");
}

/// Tests that parenthesized [`Expr`]s, tuples, and blocks can be nested inside
/// each other.
#[test]
fn parens_and_blocks_can_be_nested() {
    assert_ast("{({})}", "(a: (b: (p: (b:))))");
    assert_ast("{({}, {})}", "(a: (b: (t: (b:) (b:))))");
    assert_ast("(())", "(a: (p: (t:)))");
    assert_ast("((1), (2))", "(a: (t: (p: 1) (p: 2)))");
}

/// Tests that functions are parsed.
#[test]
fn functions_are_parsed() {
    assert_ast("() -> 1", "(a: (-> 1))");
    assert_ast("(x) -> 2", "(a: (-> x 2))");
    assert_ast("(y,) -> 3", "(a: (-> y 3))");
    assert_ast("z -> 4", "(a: (-> z 4))");
    assert_ast("(a, b) -> c", "(a: (-> a b c))");
    assert_error!(
        "(d e) -> f",
        ErrorKind::UnexpectedToken(TokenType::CloseParen, Token::Ident(s)) if s.to_string() == "e"
    );

    assert_ast("(g, h,) -> i", "(a: (-> g h i))");
}

/// Tests that empty function parameters are not parsed.
#[test]
fn empty_function_parameters_are_not_parsed() {
    assert_error!("-> 3.14", ErrorKind::ExpectedExpr(Token::MinusGreater));
}

/// Tests that separating commas are required between call arguments.
#[test]
fn call_arguments_require_separating_commas() {
    assert_ast("f()", "(a: (f))");
    assert_ast("f(1)", "(a: (f 1))");
    assert_error!(
        "f(1 2)",
        ErrorKind::UnexpectedToken(TokenType::CloseParen, Token::Literal(Literal::Number(2.0)))
    );

    assert_ast("f(1, 2)", "(a: (f 1 2))");
}

/// Tests that trailing commas are allowed after call arguments.
#[test]
fn call_arguments_allow_trailing_commas() {
    assert_error!("f(,)", ErrorKind::ExpectedExpr(Token::Comma));
    assert_ast("f(1,)", "(a: (f 1))");
    assert_ast("f(1, 2,)", "(a: (f 1 2))");
}

/// Tests that mismatched types are not checked by the [`Parser`].
#[test]
fn mismatched_types_are_unchecked() {
    assert_ast("false + 1", "(a: (+ false 1))");
    assert_ast("-true", "(a: (- true))");
}

/// Tests that comparisons cannot be chained.
#[test]
fn comparisons_cannot_be_chained() {
    // Chained comparisons are not supported for forward compatibility with
    // expressions like `min <= value <= max`.
    assert_error!("1 == x == y", ErrorKind::ChainedComparison);
    assert_error!("x == y != z", ErrorKind::ChainedComparison);
    assert_error!("1 != 2 == y", ErrorKind::ChainedComparison);
    assert_error!("1 != 2 != 3", ErrorKind::ChainedComparison);
    assert_error!("1 < 2 < 3", ErrorKind::ChainedComparison);
    assert_error!("1 < 2 <= 3", ErrorKind::ChainedComparison);
    assert_error!("1 <= 2 < 3", ErrorKind::ChainedComparison);
    assert_error!("1 <= 2 <= 3", ErrorKind::ChainedComparison);
    assert_error!("1 > 2 > 3", ErrorKind::ChainedComparison);
    assert_error!("1 > 2 >= 3", ErrorKind::ChainedComparison);
    assert_error!("1 >= 2 > 3", ErrorKind::ChainedComparison);
    assert_error!("1 >= 2 >= 3", ErrorKind::ChainedComparison);
    assert_error!("x == y < 10", ErrorKind::ChainedComparison);

    // Comparisons cannot be chained by mixing precedence levels.
    assert_error!("1 + 2 == 3 - 0 == 4", ErrorKind::ChainedComparison);
    assert_error!("1 * 2 != 0 / 3 == 4 * 0", ErrorKind::ChainedComparison);
    assert_error!("1 + 2 >= 3 * 0 < 4", ErrorKind::ChainedComparison);
    assert_error!("1 + 2 <= 3 / 1 > 0.5", ErrorKind::ChainedComparison);
    assert_error!("!!true == !false == !!!false", ErrorKind::ChainedComparison);
    assert_error!("foo() == bar() == baz()", ErrorKind::ChainedComparison);
}

/// Tests that comparisons can be chained with groupings.
#[test]
fn comparisons_can_be_chained_with_groupings() {
    assert_ast("(a == b) == c", "(a: (== (p: (== a b)) c))");
    assert_ast("(1 < 2) == true", "(a: (== (p: (< 1 2)) true))");
    assert_ast("(1 <= 2) == true", "(a: (== (p: (<= 1 2)) true))");
    assert_ast("(1 > 2) != true", "(a: (!= (p: (> 1 2)) true))");
    assert_ast("(1 >= 2) != true", "(a: (!= (p: (>= 1 2)) true))");

    assert_ast(
        "(a == b, c == d) == e",
        "(a: (== (t: (== a b) (== c d)) e))",
    );

    assert_ast(
        "{a == b, c == d} == e",
        "(a: (== (b: (== a b) (== c d)) e))",
    );

    assert_ast(
        "((a == b) == (c == d)) == e",
        "(a: (== (p: (== (p: (== a b)) (p: (== c d)))) e))",
    );
}

/// Tests that leading plus signs are not parsed.
#[test]
fn leading_plus_signs_are_not_parsed() {
    assert_error!("+1", ErrorKind::ExpectedExpr(Token::Plus));
}

/// Tests that operators have the expected associativity.
#[test]
fn operators_have_expected_associativity() {
    assert_ast("---1", "(a: (- (- (- 1))))");
    assert_ast("!!!true", "(a: (! (! (! true))))");
    assert_ast("1 + 2 + 3", "(a: (+ (+ 1 2) 3))");
    assert_ast("4 - 5 - 6", "(a: (- (- 4 5) 6))");
    assert_ast("x - y + 1", "(a: (+ (- x y) 1))");
    assert_ast("7 * 8 * 9", "(a: (* (* 7 8) 9))");
    assert_ast("a / b / c", "(a: (/ (/ a b) c))");
    assert_ast("1 ^ 2 ^ 3", "(a: (^ 1 (^ 2 3)))");
    assert_ast("a && b && c", "(a: (&& (&& a b) c))");
    assert_ast("a || b || c", "(a: (|| (|| a b) c))");
    assert_ast("f(1)(2)(3)", "(a: (((f 1) 2) 3))");
    assert_ast("x -> y -> z", "(a: (-> x (-> y z)))");
    assert_ast("c ? t : c2 ? t2 : e2", "(a: (? c t (? c2 t2 e2)))");
}

/// Tests that unary operators have the expected precedence levels.
#[test]
fn unary_operators_have_expected_precedence_levels() {
    // The precedence levels of all unary operators are equal.
    assert_ast("-!true", "(a: (- (! true)))");
    assert_ast("!-1", "(a: (! (- 1)))");

    // The `-` operator is both unary and binary.
    assert_ast("1 -1", "(a: (- 1 1))");
    assert_ast("1, -1", "(a: 1 (- 1))");
    assert_ast("-1 - 1", "(a: (- (- 1) 1))");

    // Unary operators have a high precedence, but have a lower precedence than
    // function calls.
    assert_ast("-1 * x", "(a: (* (- 1) x))");
    assert_ast("-f(x)", "(a: (- (f x)))");
    assert_ast("-f(x)(y)", "(a: (- ((f x) y)))");
    assert_ast("-x -> y", "(a: (-> (- x) y))");
    assert_ast("-(x) -> y", "(a: (-> (- (p: x)) y))");

    // Unary operators have a lower precedence than exponentiation.
    assert_ast("-x^2", "(a: (- (^ x 2)))");

    // Unary operators are allowed in exponents.
    assert_ast("-1 ^ -2 ^ -3", "(a: (- (^ 1 (- (^ 2 (- 3))))))");
}

/// Tests that binary operators have the expected precedence levels.
#[test]
fn binary_operators_have_expected_precedence_levels() {
    // Functions have the lowest precedence.
    assert_ast(
        "true || x -> x || false",
        "(a: (-> (|| true x) (|| x false)))",
    );

    // The precedence of `||` is lower than `&&`.
    assert_ast("a || b && c || d", "(a: (|| (|| a (&& b c)) d))");
    assert_ast("a && b || c && d", "(a: (|| (&& a b) (&& c d)))");

    // The precedence of `&&` is lower than comparison operators.
    assert_ast("a && b == c && d", "(a: (&& (&& a (== b c)) d))");
    assert_ast("a == b && c == d", "(a: (&& (== a b) (== c d)))");

    // The precedence of comparison operators are equal and lower than `+` and
    // `-`.
    assert_ast("1 + 2 == 3 - 4", "(a: (== (+ 1 2) (- 3 4)))");
    assert_ast("1 + 2 != 3 - 4", "(a: (!= (+ 1 2) (- 3 4)))");
    assert_ast("1 + 2 < 3 - 4", "(a: (< (+ 1 2) (- 3 4)))");
    assert_ast("1 + 2 <= 3 - 4", "(a: (<= (+ 1 2) (- 3 4)))");
    assert_ast("1 + 2 > 3 - 4", "(a: (> (+ 1 2) (- 3 4)))");
    assert_ast("1 + 2 >= 3 - 4", "(a: (>= (+ 1 2) (- 3 4)))");

    // The precedence of `+` is equal to `-`.
    assert_ast("1 + 2 - 3", "(a: (- (+ 1 2) 3))");
    assert_ast("1 - 2 + 3", "(a: (+ (- 1 2) 3))");

    // The precedence of `*` is equal to `/`.
    assert_ast("1 * 2 / 3", "(a: (/ (* 1 2) 3))");
    assert_ast("1 / 2 * 3", "(a: (* (/ 1 2) 3))");

    // The precedence of `*` and `/` is higher than `+` and `-`.
    assert_ast("1 + 2 * 3", "(a: (+ 1 (* 2 3)))");
    assert_ast("1 + 2 * 3 + 4", "(a: (+ (+ 1 (* 2 3)) 4))");

    // The precedence of `^` is higher than `*` and `/`.
    assert_ast("1 * 2 ^ 3", "(a: (* 1 (^ 2 3)))");
    assert_ast("1 * 2 ^ 3 * 4", "(a: (* (* 1 (^ 2 3)) 4))");

    // The precedence of `^` is lower than function calls.
    assert_ast("f() ^ 2", "(a: (^ (f) 2))");
    assert_ast("n ^ f()", "(a: (^ n (f)))");

    // Precedence can be overridden with parentheses.
    assert_ast("(1 + 2) * 3", "(a: (* (p: (+ 1 2)) 3))");
    assert_ast(
        "1 + (x -> x - 2)(10)",
        "(a: (+ 1 ((p: (-> x (- x 2))) 10)))",
    );
}

/// Tests that the ternary conditional [`Expr`] has the expected precedence
/// level.
#[test]
fn ternary_conditional_has_expected_precedence_level() {
    // The middle of ternary conditionals has minimum precedence.
    assert_ast("c ? x -> 2 * x : e", "(a: (? c (-> x (* 2 x)) e))");
    assert_ast("c ? c2 ? t2 : e2 : e", "(a: (? c (? c2 t2 e2) e))");

    // The precedence of ternary conditionals is equal to functions.
    assert_ast("x -> x ? 1 : 0", "(a: (-> x (? x 1 0)))");
    assert_ast("x ? 1 : y -> z", "(a: (? x 1 (-> y z)))");
}

/// Tests that [`LexError`]s are caught and encapsulated as [`ErrorKind`]s.
#[test]
fn lex_errors_are_caught() {
    assert_error!(
        "foo + $bar",
        ErrorKind::Lexing(LexError::UnexpectedChar('$'))
    );

    assert_error!("foo & bar", ErrorKind::Lexing(LexError::BitwiseAnd));
    assert_error!("foo | bar", ErrorKind::Lexing(LexError::BitwiseOr));
}

/// Asserts that an expected [`Ast`] is parsed from source code.
fn assert_ast(source: &str, expected: &str) {
    let ast = parse_source(source).expect("source code should be valid");
    assert_eq!(ast.to_string(), expected);
}
