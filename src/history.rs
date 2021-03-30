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

    pub fn add(&mut self, slice: &[String]){
        let mut vector: Vec<String> = Vec::new();
        for string in slice{
            let temp = String::from(string);
            vector.push(temp);
        }
        &self.commands.push(vector);
    }

    pub fn print_commands(&self){
        for outer in self.commands.iter(){
            for inner in outer.iter(){
                print!("\"{}\" ", inner);
            }
            print!("\n");
        }
    }
}

// TODO: Help I need some methods to work
