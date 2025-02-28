mod expr;
mod lexer;
mod parser;
mod token;

use std::{
    env,
    io::{self, Write},
};

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

    println!("Clac - Command-line calculator\nEnter `{EXIT_SHORTCUT}` to exit.\n");

    loop {
        print!("clac> ");
        io::stdout().flush().unwrap();
        let source = read_line();

        if source.is_empty() {
            break;
        }

        execute_source(&source);
        println!();
    }
}

/// Reads a line of text from standard input.
fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

/// Executes statement source code.
fn execute_source(source: &str) {
    match parser::parse_source(source) {
        Ok(expr) => println!("{}", expr.evaluate()),
        Err(error) => eprintln!("Syntax error: {error}"),
    }
}
