use std::fs::OpenOptions;
use std::io::Error;
use std::process::{Command, Stdio};

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
            let mut new_process = Command::new(command[0].to_string());
            for i in 1..command.len() {
                new_process.arg(&command[i].to_string());
            }
            Ok(Some(new_process))
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
    let mut pathway = String::new();
    pathway.push_str(&tokens[0]);
    let file_name = OpenOptions::new()
        .create(true)
        .append(true)
        .open(pathway)
        .expect("Failed to open");
    let mut proc = process.unwrap();
    proc.stdout(file_name);
    Ok(Some(proc))
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
    let mut pathway = String::new();
    pathway.push_str(&tokens[0].to_string());
    let file_name = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(pathway)
        .expect("Failed to open");
    let mut proc = process.expect("Failed to unwrap");
    proc.stderr(file_name);
    Ok(Some(proc))
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
    let mut pathway = String::new();
    pathway.push_str(&tokens[0].to_string());
    let file_name = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&pathway)
        .expect("Failed to open");
    let file_two = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&pathway)
        .expect("Failed to open");

    let mut proc = process.expect("Failed to unwrap");
    proc.stdout(file_name);
    proc.stderr(file_two);
    Ok(Some(proc))
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
    let mut pathway = String::new();
    pathway.push_str(&tokens[0].to_string());
    let file_name = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(pathway)
        .expect("Failed to open");

    let mut proc = process.expect("Failed to unwrap");
    proc.stdout(file_name);
    Ok(Some(proc))
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
    let mut pathway = String::new();
    pathway.push_str(&tokens[0].to_string());
    let file_name = OpenOptions::new()
        .read(true)
        .open(pathway)
        .expect("Failed to open");
    let mut proc = process.expect("failed to unwrap");
    proc.stdin(file_name);
    Ok(Some(proc))
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
    if commands.is_empty() {
        println!("Need something after the pipe to run a pipe command");
    }
    //TODO
    // Write code here to handle a pipe between processes -- a unidirectional data stream
    // that can be used for interprocess communication.
    let stdout = process
        .unwrap()
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn")
        .stdout;
    let mut arguments = vec![];
    for i in 1..commands.len() {
        arguments.push(commands[i].to_string());
    }

    let mut new_process = Command::new(&commands[0]);
    new_process.args(arguments);
    new_process.stdin(stdout.unwrap());
    Ok(Some(new_process))

    // Write code here to run and connect the left hand side process to a new ready-to-execute Command
}
