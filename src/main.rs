use std::env;

/// Runs Clac.
fn main() {
    let mut args = env::args().skip(1);

    match args.next() {
        None => todo!("REPL mode"),
        Some(mut source) => {
            for arg in args {
                source.push(' ');
                source.push_str(&arg);
            }

            execute_source(&source);
        }
    }
}

/// Executes source code.
fn execute_source(source: &str) {
    println!("\"{}\"", source.escape_default());
}
