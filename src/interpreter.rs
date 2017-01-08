use instruction::Instruction;
use bytecode::Bytecode;
use function_block::FunctionBlock;
use env::Environment;
use types::Type;
use stack::{Stack, StackLevel};
use std::ops::AddAssign;
use upvalues::{Upvalue, SharedUpvalue};

#[derive(Debug, Clone, PartialEq)]
pub struct PC {
    _pc: usize,
    _instructions: Vec<Instruction>,
}

impl PC {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        PC {
            _pc: 0,
            _instructions: instructions,
        }
    }

    pub fn at_end(&self) -> bool {
        self._pc >= self._instructions.len()
    }
}

impl PC {
    pub fn get(&self, relative_index: isize) -> &Instruction {
        let index = self._pc as isize + relative_index;
        &self._instructions[index as usize]
    }
    pub fn current(&self) -> &Instruction { self.get(0) }
    pub fn skip(&mut self, n: usize) { *self += n as isize }
}

impl AddAssign<isize> for PC {
    fn add_assign(&mut self, _rhs: isize) {
        self._pc = (self._pc as isize + _rhs) as usize;
    }
}

pub struct RunResult {
    instruction_count: usize
}

#[derive(Debug, Clone)]
pub struct CallInfo {
    pub pc: PC,
    pub func: FunctionBlock,
    pub upvalues: Vec<SharedUpvalue>,
    pub _subcall_returns: Option<Vec<Type>>,
}

impl CallInfo {
    pub fn new(func: FunctionBlock, upvalues: &[SharedUpvalue]) -> Self {
        CallInfo {
            pc: PC::new(func.instructions.clone()),
            upvalues: upvalues.into(),
            func: func,
            _subcall_returns: None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    pub call_info: Vec<CallInfo>,
    pub stack: Stack,
    open_upval: SharedUpvalue
}

impl Context {
    pub fn new(stack: &Stack) -> Self {
        Context {
            call_info: vec![],
            stack: stack.clone(),
            open_upval: SharedUpvalue::new(Upvalue::Closed(Type::Nil))
        }
    }
    
    pub fn ci(&self) -> &CallInfo {
        assert!(!self.call_info.is_empty());
        &self.call_info[self.call_info.len() - 1]
    }

    pub fn ci_mut(&mut self) -> &mut CallInfo {
        assert!(!self.call_info.is_empty());
        let index = self.call_info.len() - 1;
        &mut self.call_info[index]
    }

    pub fn close_upvalues(&mut self, upto: StackLevel) {
        println!("\nclose_upvalues(upto: {:?})", upto);
        println!("stack: {}", self.stack.repr());
        while let Some(next) = self.open_upval.next() {
            println!("self.open_upval: {:#?}", self.open_upval);
            println!("open_upval.value {:?}", self.open_upval.value(self));
            match *self.open_upval.lock() {
                Upvalue::Open { ref position, .. } => {
                    let _p: usize = (*position).into();
                    if _p < upto.into() {
                        println!("reached end of open upvals");
                        return
                    }
                },
                Upvalue::Closed(_) => { break; }
            }
            {
                let val = self.open_upval.value(self);
                let mut uv = self.open_upval.lock();
                *uv = Upvalue::Closed(val);
            }
            self.open_upval = next.clone();
        }
    }

    pub fn find_upvalue(&mut self, level: StackLevel) -> SharedUpvalue {
        let mut uv = self.open_upval.clone();
        while let Some(next) = uv.next() {
            uv = next;
            if let Upvalue::Open{ref position, ..} = *uv.lock() {
                if *position == level {
                    return uv.clone()
                }
            }
        }
        self.open_upval = SharedUpvalue::new(Upvalue::Open {
            position: level,
            next: self.open_upval.clone()
        });
        self.open_upval.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub context: Context,
    pub bytecode: Bytecode,
    pub env: Type,
}

impl Interpreter {
    pub fn new(bytecode: Bytecode, env: Environment) -> Self {
        let env = env.make();
        let mut _stack = Stack::new();
        _stack[0] = env.clone().into();
        let mut context = Context::new(&_stack);

        let env_upval = Upvalue::Closed(env.clone());
        context.open_upval = SharedUpvalue::new(env_upval);
        let entry_frame = CallInfo::new(bytecode.func.clone(), &[
            context.open_upval.clone()
        ]);

        context.call_info.push(entry_frame);
        Interpreter {
            context: context,
            bytecode: bytecode,
            env: env,
        }
    }

    pub fn pc(&self) -> &PC {
        &self.context.ci().pc
    }

    pub fn step(&mut self) {
        let instruction = *self.pc().current();
        self.context.ci_mut().pc += 1;
        instruction.exec(&mut self.context);
    }

    fn print_current_line(&self) {
        let func = &self.context.ci().func;
        if let Some(ref debug) = func.debug {
            match func.source_name {
                Some(ref name) => match debug.line_info.get(self.pc()._pc) {
                    Some(line) => {
                        let current = func.lines.0 + *line as usize;
                        print!(" {}:{};", name, current)
                    },
                    None => print!(" {}:?;", name),
                },
                None => print!(" @?:?;"),
            }
        }
    }

    pub fn debug(&mut self) {
        print!("pc: {};", self.pc()._pc);
        self.print_current_line();
        println!(" {:?}", self.pc().current());
        self.step();
        println!("stack: {}", self.context.stack.repr());
        // println!("upval: {:#?}", self.context.ci().upvalues);
        // println!("");
    }

    pub fn run(&mut self) -> RunResult {
        let mut result = RunResult {
            instruction_count: 0
        };
        while !self.context.call_info.is_empty() {
            self.step();
            result.instruction_count += 1
        }
        result
    }

    pub fn run_debug(&mut self) -> RunResult {
        let mut result = RunResult {
            instruction_count: 0
        };
        while !self.context.call_info.is_empty() {
            self.debug();
            result.instruction_count += 1
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use bytecode::Bytecode;
    use parser::Parsable;
    use env::Environment;
    use std::io::Cursor;
    use std::sync::mpsc;

    fn interpreter_from_bytes(data: &[u8]) -> (Interpreter, mpsc::Receiver<String>) {
        let bytecode = Bytecode::parse(&mut Cursor::new(data.to_vec()));
        let (tx, rx) = mpsc::channel();
        let interpreter = Interpreter::new(bytecode, Environment::Testing(tx));
        (interpreter, rx)
    }

    #[test]
    fn runs_hello_world() {
        let (mut interpreter, rx) = interpreter_from_bytes(include_bytes!("../fixtures/hello_world"));
        let result = interpreter.run_debug();
        assert_eq!(result.instruction_count, 4);
        assert_eq!(rx.recv().unwrap(), "Hello, World!");
    }

    #[test]
    fn runs_a_bunch_of_constants() {
        let (mut interpreter, _) = interpreter_from_bytes(include_bytes!("../fixtures/a_bunch_of_constants"));
        interpreter.run_debug();
    }

    #[should_panic]
    #[test]
    fn assert_false_panics() {
        let (mut interpreter, _) = interpreter_from_bytes(include_bytes!("../fixtures/assert_false"));
        interpreter.run_debug();
    }

    #[test]
    fn runs_assertions() {
        let (mut interpreter, _) = interpreter_from_bytes(include_bytes!("../fixtures/assertions"));
        interpreter.run_debug();
    }

    #[test]
    fn runs_table_ops_test() {
        let (mut interpreter, _) = interpreter_from_bytes(include_bytes!("../fixtures/table_ops"));
        interpreter.run_debug();
    }

    #[test]
    fn calls_lua_functions() {
        let (mut interpreter, rx) = interpreter_from_bytes(include_bytes!("../fixtures/function"));
        interpreter.run_debug();
        assert_eq!(rx.recv().unwrap(), "outside a");
        assert_eq!(rx.recv().unwrap(), "inside a");
        assert_eq!(rx.recv().unwrap(), "after a");
    }

    #[test]
    fn calculates_gcds_correctly() {
        let (mut interpreter, rx) = interpreter_from_bytes(include_bytes!("../fixtures/gcd"));
        interpreter.run_debug();
        assert_eq!(rx.recv().unwrap(), "recursive_gcd(99, 56) = 1");
        assert_eq!(rx.recv().unwrap(), "gcd(123, 456) = 3");
    }

    #[test]
    fn branches_correctly() {
        let (mut interpreter, rx) = interpreter_from_bytes(include_bytes!("../fixtures/if_conditions"));
        interpreter.run_debug();
        assert_eq!(rx.recv().unwrap(), "true is truthy");
        assert_eq!(rx.recv().unwrap(), "false is falsey");
    }

    #[test]
    fn calculates_n_queens_solution() {
        let (mut interpreter, rx) = interpreter_from_bytes(include_bytes!("../fixtures/n_queens"));
        interpreter.run_debug();
        assert_eq!(rx.recv().unwrap(), "TODO");
    }

    #[test]
    fn fib_recursive() {
        let (mut interpreter, rx) = interpreter_from_bytes(include_bytes!("../fixtures/fib"));
        interpreter.run_debug();
        assert_eq!(rx.recv().unwrap(), "233"); // fib(12)
        assert_eq!(rx.recv().unwrap(), "377"); // fib(13)
    }

    #[test]
    fn test_closure() {
        let (mut interpreter, rx) = interpreter_from_bytes(include_bytes!("../fixtures/closures"));
        interpreter.run_debug();
        assert_eq!(rx.recv().unwrap(), "making add(2)");
        assert_eq!(rx.recv().unwrap(), "add2(5) =");
        assert_eq!(rx.recv().unwrap(), "7");
    }

    #[test]
    fn fizz_buzz() {
        let (mut interpreter, rx) = interpreter_from_bytes(include_bytes!("../fixtures/fizz_buzz"));
        interpreter.run_debug();
        let output = vec![
            "1",
            "2",
            "Fizz",
            "4",
            "Buzz",
            "Fizz",
            "7",
            "8",
            "Fizz",
            "Buzz",
            "11",
            "Fizz",
            "13",
            "14",
            "FizzBuzz",
        ];
        for s in output {
            assert_eq!(rx.recv().unwrap(), s);
        }
    }

    #[bench]
    fn step_infinite_loop(b: &mut Bencher) {
        let (mut interpreter, _) = interpreter_from_bytes(include_bytes!("../fixtures/loop"));
        b.iter(|| interpreter.step())
    }
}
