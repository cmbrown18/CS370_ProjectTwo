/// I Need some comments

/// I can be changed to fit your way of modeling a shell history. You may need to change method signatures that accept
/// a History struct as a parameter.

pub struct History {
    commands: Vec<Vec<String>>,
}

impl History {
    pub const fn new() -> Self {
        History {
            commands: Vec::new(),
        }
    }

    ///Method to add vectors of commands entered in our shell
    ///
    /// # Arguments
    ///
    /// * `slice` - String slice containing a single command that was entered into our shell
    pub fn add(&mut self, slice: &[String]){
        let mut vector: Vec<String> = Vec::new();
        for string in slice{
            let temp = String::from(string);
            vector.push(temp);
        }
        &self.commands.push(vector);
    }

    ///Method to print out vector of commands
    pub fn print_commands(&mut self, num_slice: &String) {
        let temp = String::from(num_slice);
        if temp != "none" {
            let mut count = temp.trim().parse::<usize>().expect("history.print_commands: Failed to parse to i8");
            let mut i = 0;
            for vector in self.commands.iter(){
                if i >= count {
                    let mut string = vector.join(" ");
                    string = "\"".to_owned() + &string + "\"";
                    println!("{}", string);
                }
                i += 1;
            }
        } else {
            for outer in self.commands.iter() {
                let mut string = outer.join(" ");
                string = "\"".to_owned() + &string + "\"";
                println!("{}", string);
            }
        }
    }
}

// TODO: Help I need some methods to work
