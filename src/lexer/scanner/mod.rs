#[cfg(test)]
mod tests;

use std::str::Chars;

/// A structure that reads a stream of [`char`]s from source code and records
/// lexemes.
pub struct Scanner<'a> {
    /// The iterator for reading [`char`]s and string slices from source code.
    chars: Chars<'a>,

    /// The string slice between the start of the current lexeme and the end of
    /// source code.
    rest: &'a str,
}

impl<'a> Scanner<'a> {
    /// Creates a new `Scanner` from source code to be read.
    pub fn new(source: &'a str) -> Self {
        let chars = source.chars();
        let rest = source;
        Self { chars, rest }
    }

    /// Begins a new lexeme.
    pub fn begin_lexeme(&mut self) {
        self.rest = self.chars.as_str();
    }

    /// Returns the current lexeme.
    pub fn lexeme(&self) -> &'a str {
        let length = self.rest.len() - self.chars.as_str().len();
        &self.rest[..length]
    }

    /// Consumes the next [`char`] from source code. This function returns
    /// [`None`] if the scanner is at the end of source code.
    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Consumes the next [`char`] from source code if it is equal to an
    /// expected [`char`]. This function returns whether a [`char`] was
    /// consumed.
    pub fn eat(&mut self, expected: char) -> bool {
        if let Some(char) = self.peek()
            && char == expected
        {
            self.bump();
            true
        } else {
            false
        }
    }

    /// Repeatedly consumes the next [`char`] from source code while it matches
    /// a predicate function.
    pub fn eat_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(char) = self.peek()
            && predicate(char)
        {
            self.bump();
        }
    }

    /// Returns the next [`char`] without consuming it. This function returns
    /// [`None`] if the scanner is at the end of source code.
    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }
}
