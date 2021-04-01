#Operating Systems: Project Two
##Author: Brett Dale
##Author: Chris Brown

###How to run:
cd into the src folder and use command `cargo run main.rs`

###File descriptions:


builtin.rs // Builtins (rm, ls, cd)

grammar.pest // Grammar for parsing

history.rs // Command history

main.rs // Main shell

parser.rs // Parses input

README.md // Self

redirect.rs // Handles redirection

utils.rs // Misc functions

###Known Errors
- typing in commands that are not recognized built-ins will cause the program to panic
- Control C does not automatically create a new line

###NOTE
We did implement the 'ls' feature where it will list multiple directories.