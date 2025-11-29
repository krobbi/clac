mod ast;
mod cfg;
mod clac_error;
mod compiler;
mod decl_table;
mod hir;
mod interpreter;
mod parser;
mod resolver;

use std::{
    env,
    io::{self, Write as _},
};

use self::{clac_error::ClacError, decl_table::DeclTable};

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
            eprintln!("Could not read line: {error}");
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
    if let Err(error) = try_execute_source(source) {
        eprintln!("Error: {error}");
    }
}

/// Executes source code. This function returns a [`ClacError`] if the source
/// code could not be executed.
fn try_execute_source(source: &str) -> Result<(), ClacError> {
    let ast = parser::parse_source(source)?;
    let mut decls = DeclTable::new();
    let hir = resolver::resolve_ast(&ast, &mut decls)?;
    let cfg = compiler::compile_hir(&hir, &decls);
    interpreter::interpret_cfg(&cfg);
    Ok(())
}
