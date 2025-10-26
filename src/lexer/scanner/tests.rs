use super::*;

/// Tests that no [`char`]s are scanned from empty source code.
#[test]
fn empty_source() {
    let mut scanner = Scanner::new("");
    assert!(scanner.rest.is_empty());
    assert!(scanner.peek().is_none());
    assert!(scanner.lexeme().is_empty());
    assert!(!scanner.eat('\0'));
    assert!(scanner.bump().is_none());
}

/// Tests that lexemes have expected boundaries, including non-ASCII [`char`]s.
#[test]
fn lexemes() {
    const SOURCE: &str = "{áéíóú}[☕🧀🍗]";

    let mut scanner = Scanner::new(SOURCE);
    assert!(scanner.lexeme().is_empty());
    assert_eq!(scanner.bump(), Some('{'));
    assert_eq!(scanner.lexeme(), "{");

    scanner.begin_lexeme();
    assert!(scanner.lexeme().is_empty());
    scanner.eat_while(char::is_alphabetic);
    assert_eq!(scanner.lexeme(), "áéíóú");

    scanner.begin_lexeme();
    assert!(scanner.lexeme().is_empty());
    assert_eq!(scanner.bump(), Some('}'));
    assert_eq!(scanner.bump(), Some('['));
    assert_eq!(scanner.lexeme(), "}[");

    scanner.begin_lexeme();
    assert!(scanner.lexeme().is_empty());
    assert_eq!(scanner.bump(), Some('☕'));
    assert_eq!(scanner.bump(), Some('🧀'));
    assert_eq!(scanner.bump(), Some('🍗'));
    assert_eq!(scanner.lexeme(), "☕🧀🍗");

    scanner.begin_lexeme();
    assert!(scanner.lexeme().is_empty());
    assert_eq!(scanner.bump(), Some(']'));
    assert_eq!(scanner.lexeme(), "]");

    assert!(scanner.bump().is_none());
    assert!(!scanner.eat('\0'));
    assert_eq!(scanner.rest, scanner.lexeme());
}

/// Tests that [`Scanner::eat_while`] terminates at EOF.
#[test]
fn eat_while_terminates() {
    const SOURCE: &str = "0123456789";

    let mut scanner = Scanner::new(SOURCE);
    assert!(scanner.lexeme().is_empty());
    scanner.eat_while(is_char_not_eof);
    assert_eq!(scanner.lexeme(), SOURCE);
    assert!(scanner.peek().is_none());
}

/// A [`char`] predicate function that always returns `true`.
fn is_char_not_eof(char: char) -> bool {
    use std::hint::black_box;

    black_box(char) == black_box(char)
}
