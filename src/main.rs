mod ast;
mod parser;
mod runtime;

use std::{
    env,
    io::{self, Write},
};

use runtime::Runtime;

/// Runs Clac.
fn main() {
    let mut runtime = Runtime::new();
    let mut args = env::args().skip(1);

    match args.next() {
        None => run_repl(&mut runtime),
        Some(mut source) => {
            for arg in args {
                source.push(' ');
                source.push_str(&arg);
            }

            execute_source(&source, &mut runtime);
        }
    }
}

/// Runs Clac in REPL mode with a runtime environment.
fn run_repl(runtime: &mut Runtime) {
    #[cfg(target_os = "windows")]
    const EXIT_SHORTCUT: &str = "Ctrl+Z";

    #[cfg(not(target_os = "windows"))]
    const EXIT_SHORTCUT: &str = "Ctrl+D";

    println!("Clac - command-line calculator\nEnter [{EXIT_SHORTCUT}] to exit.");

    let mut source = String::new();

    loop {
        print!("\nclac> ");
        io::stdout().flush().unwrap();

        source.clear();
        io::stdin().read_line(&mut source).unwrap();

        if source.is_empty() {
            break;
        }

        execute_source(&source, runtime);
    }

    println!("\nReceived [{EXIT_SHORTCUT}], exiting...");
}

/// Executes Clac source code with a runtime environment.
fn execute_source(source: &str, runtime: &mut Runtime) {
    match parser::parse_source(source) {
        Ok(program) => {
            for expr in program {
                if let Err(error) = runtime.execute_expr(expr) {
                    eprintln!("Runtime error: {error}");
                    return;
                }
            }
        }
        Err(error) => eprintln!("Syntax error: {error}"),
    }
}
