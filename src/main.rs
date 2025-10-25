mod lexer;

use std::{
    env,
    io::{self, Write as _},
};

use crate::lexer::{Lexer, Token};

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

    println!("Clac - Command line calculator\nEnter [{EXIT_SHORTCUT}] to exit.");
    let mut source = String::new();

    loop {
        print!("\nclac> ");
        io::stdout()
            .flush()
            .expect("flushing stdout should not fail");

        source.clear();

        if let Err(error) = io::stdin().read_line(&mut source) {
            eprintln!("\nCould not read line: {error}");
            continue;
        }

        if source.is_empty() {
            break;
        }

        execute_source(&source);
    }

    println!("\nReceived [{EXIT_SHORTCUT}], exiting...");
}

/// Executes source code.
fn execute_source(source: &str) {
    let mut lexer = Lexer::new(source);

    loop {
        match lexer.bump() {
            Ok(Token::Eof) => {
                println!("--- END ---");
                break;
            }
            Ok(token) => println!("{token:?}"),
            Err(error) => eprintln!("{error}"),
        }
    }
}
