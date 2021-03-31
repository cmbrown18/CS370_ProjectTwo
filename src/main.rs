/// Agora is using Rust, version 1.45.0
pub mod builtin;
pub mod history;
pub mod parser;
pub mod redirect;
pub mod utils;
use crate::builtin::builtin;
use crate::history::History;
use crate::utils::{execute, parse_line, prompt_and_read};
use ctrlc::set_handler;
use std::cell::RefCell;

///
/// main.rs
///
/// An implementation of a simple UNIX shell.  This program supports:
///    - Running processes
///    - Redirecting standard output (>)
///    - Redirecting standard input (<)
///    - Appending standard output to a file (>>)
///    - Redirecting both standard output and standard input (&>)
///    - Creating process pipelines (p1 | p2 | ...)
///    - Interrupting a running process (e.g., ctrl-C)
///    - A built-in version of the 'ls' command
///    - A built-in version of the 'rm' command
///    - A built-in version of the 'touch' command
///    - A built-in version of the 'chdir' command
///    - A built-in version of the 'pwd' command
///    - A built-in 'history' list
///
/// Among the many things it does _NOT_ support are:
///    - Environment variables
///    - Appending standard error to a file (2>>)
///    - Backgrounding processes (p1&)
///    - Unconditionally chaining processes (p1;p2)
///    - Conditionally chaining processes (p1 && p2 or p1 || p2)
///    - re-executing history commands
///                                                                                                                     
/// Keep in mind that this program was written to be easily understood/modified for educational
/// purposes.  The author makes no claim that this is the "best" way to solve this problem.

fn main() {
    let mut history = History::new();
    set_handler(handler).expect("Failed to set signal handler");

    loop {
        let tokens = prompt_and_read().unwrap_or(Vec::new());

        // Check if user want to run a builtin or not
        if let Ok(false) = builtin(&tokens, &mut history) {
            if let Ok(Some(mut child)) = parse_line(&tokens, None) {
                if let Err(e) = execute(&mut child) {
                    eprintln!("Error: Could not execute process.\n{}", e);
                }
            } else {
                // Reasons this will execute:
                // * User entered only whitespace
                // * CTRL+C or CTRL+D was pressed in parent process
                // * One of the redirect functions was last to return
                //      ie: cat input.txt << file.txt
                // * Pipe encountered an error and returned early
                // * An error occurred parsing the line
                //      Specifically, writing to stdout/stderr in parse_line
            }
        } else {
            // Reasons this will execute:
            // * A builtin function returned an error
            // * A builtin function returned true
        }
    }
}

fn handler() {
    println!("exit to quit out of the shell");
}
