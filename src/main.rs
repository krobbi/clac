use std::{
    env,
    io::{self, Write},
};

/// Run Clac.
fn main() {
    let mut args = env::args().skip(1);

    match args.next() {
        None => run_repl(),
        Some(mut line) => {
            for arg in args {
                line.push(' ');
                line.push_str(&arg);
            }

            execute_line(&line);
        }
    }
}

/// Run Clac in REPL mode.
fn run_repl() {
    #[cfg(target_os = "windows")]
    const EXIT_SHORTCUT: &str = "Ctrl+Z";

    #[cfg(not(target_os = "windows"))]
    const EXIT_SHORTCUT: &str = "Ctrl+D";

    println!("Clac - Command-line calculator\nEnter `{EXIT_SHORTCUT}` to exit.\n");

    loop {
        print!("clac> ");

        if let Err(error) = io::stdout().flush() {
            panic!("failed to flush output: {error}");
        }

        let line = read_line();

        if line.is_empty() {
            break;
        }

        execute_line(line.trim());
        println!();
    }
}

/// Read a line of text from standard input.
fn read_line() -> String {
    let mut line = String::new();

    match io::stdin().read_line(&mut line) {
        Ok(_) => line,
        Err(error) => panic!("failed to read line: {error}"),
    }
}

/// Execute a line of text.
fn execute_line(line: &str) {
    println!("`{line}`");
}
