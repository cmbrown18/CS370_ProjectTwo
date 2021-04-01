use crate::history::History;
use std::fs::OpenOptions;
use std::fs::{self};
use std::io::Error;
use std::path::Path;
use std::process::exit;

/// Handles builtins
///
/// # Arguments
///
/// * `commands` - A string slice representing a command and its arguments
/// * `history` - History struct that keeps track of all commands entered in shell
///
/// # Return value
///
/// True if the command was a builtin, else false.
pub fn builtin(commands: &[String], mut history: &mut History) -> Result<bool, Error> {
    history.add(&commands);
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

    //if the command is literally 'ls'
    if args.len() == 1 {
        //next few lines are from internet, if statement below is probably useless
        for entry in fs::read_dir(".")? {
            println!("\"{}\"", entry.unwrap().path().display());
        }
    } else {
        //if it is 2
        let mut count = 0;
        for arg in args.iter() {
            if count > 0 {
                println!("\n{}:", arg);
                for entry in fs::read_dir(&arg)? {
                    println!("\"{}\"", entry.unwrap().path().display());
                }
            }
            count += 1;
        }
    }
    Ok(())
}

/// Implements a built-in version of the 'rm' command.
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
fn file_remove_builtin(args: &[String]) -> Result<(), Error> {
    if !args.is_empty() {
        if args[1] == "-r" {
            if args.len() != 3 {
                println!("rm: missing operand")
            } else {
                for i in 2..args.len() {
                    if Path::new(&args[i]).is_dir() {
                        fs::remove_dir_all(&args[i]).expect("Cannot remove");
                    } else if Path::new(&args[i]).exists() {
                        fs::remove_file(&args[i]).expect("Failed to remove");
                    } else {
                        println!("rm: cannot remove '{}': No such file or directory", args[2]);
                    }
                }
            }
        } else if args[1] != "-r" {
            for i in 1..args.len() {
                let path = Path::new(&args[i]);
                if path.is_dir() {
                    println!(
                        "'{}' is a directory, cannot delete without -r tag",
                        &args[i]
                    );
                } else if path.exists() {
                    std::fs::remove_file(&args[i]).expect("Failed to remove");
                } else {
                    println!("rm: cannot remove '{}': No such file or directory", args[1]);
                }
            }
        }
    } else {
        print!("rm <list of files>")
    }
    Ok(())
}

/// Implements a built-in version of the 'touch' command.
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
fn touch_builtin(args: &[String]) -> Result<(), Error> {
    if !args.is_empty() {
        for i in 1..args.len() {
            if Path::new(&args[i]).exists() {
                let filedata = fs::metadata(&args[i])?;
                let file = OpenOptions::new()
                    .create(false)
                    .write(true)
                    .read(true)
                    .open(&args[i])
                    .unwrap();
                file.set_len(filedata.len())?;
            } else {
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .read(true)
                    .open(&args[i])
                    .unwrap();
            }
        }
    }
    Ok(())
}

/// Implements a built-in version of the 'cd' command.
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
fn change_dir_builtin(args: &[String]) -> Result<(), Error> {
    if args.len() == 1 {
        let key = "HOME";
        let val = std::env::var_os(key);
        std::env::set_current_dir(val.unwrap()).expect("Failed to change directories");
    } else {
        if Path::new(&args[1]).is_dir() {
            std::env::set_current_dir(&args[1]).expect("Failed to change directories");
        } else {
            println!("cd: {}: no such file or directory", &args[1]);
        }
    }
    Ok(())
}

/// Implements a built-in version of the 'pwd' command.
fn pwd_builtin() {
    let current_directory = std::env::current_dir().unwrap();
    println!("{:?}", fs::canonicalize(&current_directory).unwrap());
}

/// Implements a built-in command history
///
/// # Arguments
///
/// * `args` - A vector of strings corresponding to the command and its arguments.
/// * `history` - A history struct to use for our shell
fn history_builtin(args: &[String], history: &mut History) -> Result<(), Error> {
    if args.len() == 1 {
        history.print_commands(&String::from("none"));
    } else {
        history.print_commands(&args[1]);
    }
    Ok(())
}
