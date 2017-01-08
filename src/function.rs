use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use parking_lot::Mutex;

use stack::StackEntry;
use types::{Type, Representable};
use function_block::FunctionBlock;
use interpreter::{Upvalue, Context};

pub type NativeFunction = Box<Fn(&mut FunctionInterface)>;

pub struct FunctionInterface<'a> {
    params: &'a [StackEntry],
    pub ret: Vec<Type>
}

impl<'a> FunctionInterface<'a> {
    pub fn new(params: &'a [StackEntry]) -> Self {
        FunctionInterface {
            params: params,
            ret: Vec::new(),
        }
    }
    pub fn arguments(&self) -> Vec<StackEntry> {
        self.params.into()
    }
    pub fn get(&self, index: usize) -> &StackEntry {
        &self.params[index]
    }
    pub fn returns<T: Into<Vec<Type>>>(&mut self, ret: T) {
        self.ret = ret.into()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LuaFunction {
    // pub function_index: usize, // function index in the stack
    // top: usize, // top for this function
    // struct CallInfo *previous, *next;  /* dynamic call link */
    // base: usize,
    // savedpc: usize,
    // extra: usize,
    // number_results: usize,
    // callstatus: u8,
    pub upvalues: Vec<Upvalue>,
    pub proto: FunctionBlock,
}

impl LuaFunction {
    pub fn new(context: &mut Context, proto: FunctionBlock) -> Self {
        let upvals = proto.upvalues.iter().map(|uv| {
            if uv.instack { // upValue refers to local variable
                let level = context.stack.get_level(uv.index as usize);
                context.find_upvalue(level)
            } else { // get upValue from enclosing function
                context.ci().upvalues[uv.index as usize].clone()
            }
        }).collect();
        LuaFunction {
            proto: proto,
            upvalues: upvals
        }
    }
}

#[derive(Clone)]
pub enum Function {
    Lua(LuaFunction),
    Native(Arc<Mutex<NativeFunction>>),
}

impl From<NativeFunction> for Function {
    fn from(f: NativeFunction) -> Function {
        let syncable: Arc<Mutex<NativeFunction>> = Arc::new(Mutex::new(f));
        Function::Native(syncable)
    }
}

impl Eq for Function {}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        if let (&Function::Lua(ref f_self), &Function::Lua(ref f_other)) = (self, other) {
            return f_self == f_other
        }
        false
    }
}

impl Ord for Function {
    fn cmp(&self, _: &Function) -> ::std::cmp::Ordering {
        unimplemented!()
    }
}

impl PartialOrd for Function {
    fn partial_cmp(&self, _: &Function) -> Option<::std::cmp::Ordering> {
        unimplemented!()
    }
}

impl Hash for Function {
    fn hash<H: Hasher>(&self, _: &mut H) {
        unimplemented!()
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Function::Lua(ref lua_func) => write!(f, "{:?}", lua_func),
            Function::Native(_) => write!(f, "Function::Native(...)"),
        }
    }
}

impl Representable for Function {
    fn repr(&self) -> String {
        match *self {
            Function::Lua(ref lf) => format!("function: {:p}", lf),
            Function::Native(ref nf) => format!("function: {:p}", nf),
        }
    }
}