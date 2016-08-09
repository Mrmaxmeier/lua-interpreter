use types::Type;

#[derive(Debug, Clone)]
pub struct Interpreter {
    stack: Vec<Type>,
    register_a: (),
    register_b: (),
    register_c: (),
    pc: usize,
}

/*
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: Vec::new()
        }
    }

    fn step() {

    }
}
*/