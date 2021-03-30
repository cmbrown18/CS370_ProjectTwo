use std::io::{Error, ErrorKind};
use std::process::{Command, Stdio};
use std::{fs::File, io::Write};
use std::{fs::OpenOptions, io::Read};

/// Handles redirection
///
/// # Arguments
///
/// * `redirector` - A string representing the redirect operation to perform, if any
/// * `command` - A slice of strings representing a command and its arguments
/// * `process` - An `Option` representing a read-to-execute Command to be modified/executed/returned
///
/// # Return value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
pub fn redirect(
    redirector: &str,
    command: &[String],
    process: Option<Command>,
) -> Result<Option<Command>, Error> {
    match redirector {
        // ---- Append redirection ----
        ">>" => handle_append_redirect(command, process),

        // ---- stderr redirection ----
        "2>" => handle_stderr_redirect(command, process),

        // ---- stdout and stderr redirection ----
        "&>" => handle_stdout_stderr_redirect(command, process),

        // ---- Stdout redirection ----
        ">" | "1>" => handle_stdout_redirect(command, process),

        // ---- Stdin redirection ----
        "<" => handle_stdin_redirect(command, process),
        // ---- pipe in between processes ----
        "|" => handle_pipe(command, process),
        _ => {
            // TODO
            // This is not a redication operator but a command we want to execute, so set up a ready-to-execute
            // Command with any command line arguments it should have and return it.
            // Replace this error with correct code
            return Err(Error::new(ErrorKind::Other, "Redirect not finished"));
        }
    }
}

/// Redirects standard output from this ready-to-execute Command to the file with the specified name.
/// Data is appended to the file instead of truncating existing file.
///
/// # Arguments
///
/// * `tokens` - A vector of strings corresponding to the command/operator and its arguments
/// * `process` - The current ready-to-execute Command to be redirected
///
/// # Return Value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
fn handle_append_redirect(
    tokens: &[String],
    process: Option<Command>,
) -> Result<Option<Command>, Error> {
    //TODO
    // Write code here to cause the standard output of this ready-to-run command
    // to be sent to a file with the specified name. If the file already exists, its
    // contents should be appended when this (soon-to-be process) writes to the file
    let mut pathway = String::new();
    for i in 0..tokens.len() {
        if tokens[i] == ">>" {
            pathway = tokens[i + 1].to_string();
        }
    }
    let file_name = OpenOptions::new()
        .create(true)
        .append(true)
        .open(pathway)
        .expect("Failed to open");
    let mut proc = process.unwrap();
    proc.stdout(file_name);
    Ok(Some(proc))
    //Err(Error::new(ErrorKind::Other, "append redirect not finished"))
}

/// Redirects standard error from this ready-to-execute Command to the file with the specified name.
///
/// # Arguments
///
/// * `tokens` - A vector of strings corresponding to the command/operator and its arguments
/// * `process` - The current ready-to-execute Command to be redirected
///
/// # Return Value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
fn handle_stderr_redirect(
    tokens: &[String],
    process: Option<Command>,
) -> Result<Option<Command>, Error> {
    //TODO
    // Write code here to cause the standard error of this ready-to-run command
    // to be sent to a file with the specified name. If the file already exists, its
    // contents should be truncated before this (soon-to-be process) writes to the file.
    let mut pathway = String::new();
    for i in 0..tokens.len() {
        if tokens[i] == ">>" {
            pathway = tokens[i + 1].to_string();
        }
    }
    let file_name = OpenOptions::new()
        .create(true)
        .truncate(true)
        .open(pathway)
        .expect("Failed to open");
    let mut proc = process.unwrap();
    proc.stderr(file_name);
    Ok(Some(proc))
    //Err(Error::new(ErrorKind::Other, "stderr redirect not finished"))
}

/// Redirects stdout and stderr from this ready-to-execute Command to the file with the specified name.
///
/// # Arguments
///
/// * `tokens` - A vector of strings corresponding to the command/operator and its arguments
/// * `process` - The current ready-to-execute Command to be redirected
///
/// # Return Value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
fn handle_stdout_stderr_redirect(
    tokens: &[String],
    process: Option<Command>,
) -> Result<Option<Command>, Error> {
    //TODO
    // Write code here to cause the standard output and standard error of this ready-to-run command
    // to be sent to a file with the specified name. If the file already exists, its
    // contents should be truncated before this (soon-to-be process) writes to the file.

    let mut pathway = String::new();
    for i in 0..tokens.len() {
        if tokens[i] == "&>" {
            pathway = tokens[i + 1].to_string();
        }
    }
    let file_name = OpenOptions::new()
        .create(true)
        .truncate(true)
        .open(pathway)
        .expect("Failed to open");
    let mut proc = process.unwrap();
    proc.stdout(file_name);
    Ok(Some(proc))
    //Err(Error::new(
    //    ErrorKind::Other,
    //    "stdout_stderr redirect not finished",
    //))
}

/// Redirects standard output from this ready-to-execute Command to a file with the specified name.
///
/// # Arguments
///
/// * `tokens` - A vector of strings corresponding to the command and its arguments
/// * `process` - The current ready-to-execute Command to be redirected
///
/// # Return Value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
fn handle_stdout_redirect(
    tokens: &[String],
    process: Option<Command>,
) -> Result<Option<Command>, Error> {
    //TODO
    // Write code here to cause the standard output of this ready-to-run command
    // to be sent to a file with the specified name. If the file already exists, its
    // contents should be truncated before this (soon-to-be process) writes to the file.
    let mut pathway = String::new();
    for i in 0..tokens.len() {
        if tokens[i] == ">>" {
            pathway = tokens[i + 1].to_string();
        }
    }
    let file_name = OpenOptions::new()
        .create(true)
        .truncate(true)
        .open(pathway)
        .expect("Failed to open");
    let mut proc = process.unwrap();
    proc.stdout(file_name);
    Ok(Some(proc))
    //Err(Error::new(ErrorKind::Other, "stdout redirect not finished"))
}

/// Redirects standard input to this ready-to-execute Command from the file with the specified name.
///
/// # Arguments
///
/// * `tokens` - A vector of strings corresponding to the command/operator and its arguments
/// * `process` - The current ready-to-execute Command to be redirected
///
/// # Return Value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
fn handle_stdin_redirect(
    tokens: &[String],
    process: Option<Command>,
) -> Result<Option<Command>, Error> {
    //TODO
    // Write code here to cause the standard input of this ready-to-run command
    // to read from a file with the specified name

    let mut pathway = String::new();
    for i in 0..tokens.len() {
        if tokens[i] == ">>" {
            pathway = tokens[i + 1].to_string();
        }
    }
    let file_name = OpenOptions::new()
        .create(true)
        .read(true)
        .open(pathway)
        .expect("Failed to open");
    let mut proc = process.unwrap();
    proc.stdin(file_name);
    Ok(Some(proc))

    //Err(Error::new(ErrorKind::Other, "stdin redirect not finished"))
}

/// Partial impementation of a pipe between two processes.
///
/// # Arguments
///
/// * `commands` - A vector of strings corresponding to a command/operator and its arguments
/// * `process` - A ready to run Command whose output should be set up to be piped into a new ready-to-run-command
///    process is the the left hand side process in a `LHS process | RHS process`
///
/// # Return value
///
/// A `Result` with an `Option` containing a ready-to-execute `Command`
fn handle_pipe(commands: &[String], process: Option<Command>) -> Result<Option<Command>, Error> {
    // TODO
    // We do not see a RHS process there is nothing to do

    //TODO
    // Write code here to handle a pipe between processes -- a unidirectional data stream
    // that can be used for interprocess communication.

    // Write code here to run and connect the left hand side process to a new ready-to-execute Command
    Err(Error::new(ErrorKind::Other, "pipe redirect not finished"))
}
