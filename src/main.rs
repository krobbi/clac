mod lexer;
mod token;

use std::{
    env,
    io::{self, Write},
};

use lexer::Lexer;
use token::Token;

/// Runs Clac.
fn main() {
    let mut args = env::args().skip(1);

    match args.next() {
        None => run_repl(),
        Some(mut source) => {
            for arg in args {
                source.push(' ');
                source.push_str(&arg);
            }

            execute_source(&source);
        }
    }
}

/// Runs Clac in REPL mode.
fn run_repl() {
    #[cfg(target_os = "windows")]
    const EXIT_SHORTCUT: &str = "Ctrl+Z";

    #[cfg(not(target_os = "windows"))]
    const EXIT_SHORTCUT: &str = "Ctrl+D";

    println!("Clac - command-line calculator\nEnter `{EXIT_SHORTCUT}` to exit.");

    let mut source = String::new();

    loop {
        print!("\nclac> ");
        io::stdout().flush().unwrap();

        source.clear();
        io::stdin().read_line(&mut source).unwrap();

        if source.is_empty() {
            break;
        }

        execute_source(&source);
    }

    println!("\nReceived `{EXIT_SHORTCUT}`, exiting...");
}

/// Executes Clac source code.
fn execute_source(source: &str) {
    let mut lexer = Lexer::new(source);

    loop {
        match lexer.next() {
            Ok(Token::Eof) => break,
            Ok(t) => println!("{t:?}"),
            Err(e) => eprintln!("Syntax error: {e}"),
        }
    }
}
