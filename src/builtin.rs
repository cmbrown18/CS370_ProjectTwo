use crate::history::History;
// TODO: include crates you need from std::env
// use std::env::{Crates Needed};
// TODO: include crates you need from std::fs
// use std::fs::{Crates Needed};
use std::io::Error;
use std::path::PathBuf;
use std::process::exit;

/// Handles builtins
///
/// # Arguments
///
/// * `commands` - A string slice representing a command and its arguments
/// * TODO: YOU TELL ME
///
/// # Return value
///
/// True if the command was a builtin, else false.
pub fn builtin(commands: &[String], mut history: &mut History) -> Result<bool, Error> {

    match &commands.first().unwrap_or(&String::new())[..] {
        "ls" => {
            if let Err(e) = list_files_builtin(commands) {
                eprintln!("Error: Could not list contents\n{}", e);
                return Err(e);
            }
            Ok(true)
        }
        "rm" => {
            if let Err(e) = file_remove_builtin(commands) {
                eprintln!("Error: Could not remove file/directory\n{}", e);
                return Err(e);
            }
            Ok(true)
        }
        "touch" => {
            if let Err(e) = touch_builtin(commands) {
                eprintln!("Error: Could not create file\n{}", e);
                return Err(e);
            }
            Ok(true)
        }
        "cd" => {
            if let Err(e) = change_dir_builtin(commands) {
                eprintln!("Error: Could not change directories\n{}", e);
                return Err(e);
            }
            Ok(true)
        }
        "pwd" => {
            pwd_builtin();
            Ok(true)
        }
        "history" => {
            if let Err(e) = history_builtin(commands, &mut history) {
                eprintln!("Error: Could not display history\n{}", e);
                return Err(e);
            }
            Ok(true)
        }
        "exit" => {
            exit(0);
        }
        _ => Ok(false),
    }
}

/// Implements a built-in version of the 'ls' command.
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
fn list_files_builtin(args: &[String]) -> Result<(), Error> {
 
    // TODO: Write code here that will list the content of the specified directory (or if no directory was specified,
    // the current directory).
    Ok(())
}

/// Implements a built-in version of the 'rm' command.
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
fn file_remove_builtin(args: &[String]) -> Result<(), Error> {
     // TODO: Write code here that will remove the specified list of files.  If no file list is specified, print a
     // usage message.
    Ok(())
}

/// Implements a built-in version of the 'touch' command.
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
fn touch_builtin(args: &[String]) -> Result<(), Error> {
     // TODO: Write code here that will create a file or update a timestamp of a file.
    Ok(())
}

/// Implements a built-in version of the 'cd' command.
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
fn change_dir_builtin(args: &[String]) -> Result<(), Error> {
     // TODO: Write code here that will change to a specified directory.
    Ok(())
}

/// Implements a built-in version of the 'pwd' command.
fn pwd_builtin() {
     // TODO: Write code here that will print the current working directory.
}


/// Implements a built-in command history
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
/// * TODO: YOU TELL ME
fn history_builtin(args: &[String], history: &mut History) -> Result<(), Error> {
    // TODO: Write code here that will print the last n commands executed via this shell.
    Ok(())
}
