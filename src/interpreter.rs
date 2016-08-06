use types::Type;

#[derive(Debug, Clone)]
pub struct Interpreter {
    stack: Vec<Type>
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: Vec::new()
        }
    }

    fn step() {

    }
}