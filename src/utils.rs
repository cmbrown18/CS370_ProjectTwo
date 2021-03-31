use crate::parser::parse;
use crate::redirect::redirect;
use std::io::{stdin, stdout, Error, Write};
use std::process::id;
use std::process::Command;

/// A simple wrapper that displays a prompt and reads a line of input from the user.
///
/// # Return value
///
/// A vector of strings corresponding to the data entered into the command line
pub fn prompt_and_read() -> Option<Vec<String>> {
    print!("({}) $ ", id());
    stdout().flush().expect("Error flushing stdout");
    let mut buffer = String::new();

    match stdin().read_line(&mut buffer) {
        Ok(_) => {
            let tokens = parse(buffer);
            return Some(tokens);
        }
        Err(e) => {
            eprintln!("Error taking user input\n{}", e);
            return None;
        }
    };
}

/// Executes the process and displays output to stdout and stderr
///
/// # Arguments
///
/// * `process` - A `Command` to be executed
//fn execute(process: &mut Command, sender: Sender<i32>) {
pub fn execute(process: &mut Command) -> Result<(), Error> {
    let child = process.spawn().expect("Failed to spawn");
    let mut child2 = child;
    child2.wait().expect("Failed to run command");
    println!("Child {} exited with status 0", child2.id());
    Ok(())
}
/// Recursively parses the line of user input
///
/// # Arguments
///
/// * `tokens` - A slice of strings representing a command and its arguments
/// * `process` - An `Option` representing a `Command` to be modified/executed/returned
///
/// # Return value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
pub fn parse_line(tokens: &[String], process: Option<Command>) -> Result<Option<Command>, Error> {
    // NOTE: This method with not parse lines correctly until redirect() works

    // Base case of recursion; no tokens left to parse
    if tokens.is_empty() {
        return Ok(process);
    }

    // Check if there is a redirect token available
    let mut redirector = "";
    let mut tokens = tokens;

    if is_special(&tokens[0]) {
        redirector = &tokens[0];
        tokens = &tokens[1..];
    }

    // If there is no process to redirect, no partial output to redirect,
    // and the first character is a redirector, then an error has occurred
    if process.is_none() && redirector != "" {
        eprintln!("Error: Expected program, found {}", redirector);
        return Ok(None);
    }

    // Split the tokens by the first special character, or the end of the token
    // list if no special character is present
    let splitter_index = tokens
        .iter()
        .position(|x| is_special(&x))
        .unwrap_or(tokens.len());
    let (command, leftover) = tokens.split_at(splitter_index);

    // Obtain a new process by redirecting
    let new_process = redirect(redirector, command, process)?;

    // Recursively return to parse the rest of the line
    return parse_line(leftover, new_process);
}

/// Determines if the current slice is a special token.
///
/// # Arguments
///
/// * `token` - A token to parse.
///
/// # Return value
///
/// True if the specified token is "special" (i.e., is an operator like >, >>, |, <, etc); false
/// otherwise.
fn is_special(token: &str) -> bool {
    if token.len() == 1
        && (token.find("<") == Some(0)
            || token.find(">") == Some(0)
            || token.find("!") == Some(0)
            || token.find("|") == Some(0))
        || token.len() == 2 && token.rfind(">") == Some(1)
    {
        return true;
    }
    false
}
