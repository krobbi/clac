use std::io::{self, Write};

/// Runs Clac.
fn main() {
    run_repl();
}

/// Runs Clac in REPL mode.
fn run_repl() {
    #[cfg(target_os = "windows")]
    const EXIT_SHORTCUT: &str = "Ctrl+Z";

    #[cfg(not(target_os = "windows"))]
    const EXIT_SHORTCUT: &str = "Ctrl+D";

    println!("Clac - command-line calculator\nEnter `{EXIT_SHORTCUT}` to exit.\n");

    let mut source = String::new();

    loop {
        print!("clac> ");
        io::stdout().flush().unwrap();

        source.clear();
        io::stdin().read_line(&mut source).unwrap();

        if source.is_empty() {
            break;
        }

        println!("'{}'\n", source.trim());
    }
}
