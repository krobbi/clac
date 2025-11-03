mod ast;
mod clac_error;
mod compiler;
mod hir;
mod interpreter;
mod ir;
mod parser;
mod resolver;

use std::{
    env,
    io::{self, Write as _},
};

use self::{clac_error::ClacError, interpreter::Globals};

/// Runs Clac.
fn main() {
    let mut globals = Globals::new();
    let mut args = env::args().skip(1);

    match args.next() {
        None => run_repl(&mut globals),
        Some(mut source) => {
            for arg in args {
                source.push(' ');
                source.push_str(&arg);
            }

            execute_source(&source, &mut globals);
        }
    }
}

/// Runs Clac in REPL mode with [`Globals`].
fn run_repl(globals: &mut Globals) {
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

        execute_source(&source, globals);
    }

    println!("\nReceived [{EXIT_SHORTCUT}], exiting...");
}

/// Executes source code with [`Globals`].
fn execute_source(source: &str, globals: &mut Globals) {
    if let Err(error) = try_execute_source(source, globals) {
        eprintln!("{error}");
    }
}

/// Executes source code with [`Globals`]. This function returns a [`ClacError`]
/// if the source code could not be executed.
fn try_execute_source(source: &str, globals: &mut Globals) -> Result<(), ClacError> {
    let ast = parser::parse_source(source)?;
    let hir = resolver::resolve_ast(&ast, globals.names().iter())?;
    println!("{hir:#?}");
    let ir = compiler::compile_hir(&hir);
    println!("{ir}");
    interpreter::interpret_ir(&ir, globals)?;
    Ok(())
}
