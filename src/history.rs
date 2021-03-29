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
}

// TODO: Help I need some methods to work
