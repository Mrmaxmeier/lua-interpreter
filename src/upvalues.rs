use parser::*;
use stack::StackLevel;
use interpreter::Context;
use types::Type;

use std::sync::Arc;
use parking_lot::{Mutex, MutexGuard};

#[derive(Debug, Clone, PartialEq)]
pub struct UpvalueInfo {
    pub name: Option<String>,
    pub instack: bool,
    pub index: u8,
}


impl Parsable for UpvalueInfo {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        UpvalueInfo {
            name: None,
            instack: u8::parse(r) > 0,
            index: u8::parse(r),
        }
    }
}

pub type UpvalueInfos = Vec<UpvalueInfo>;

impl Parsable for UpvalueInfos {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let amount = u32::parse(r);
        (0..amount).map(|_| UpvalueInfo::parse(r)).collect()
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Upvalue {
    Open { position: StackLevel, next: SharedUpvalue },
    Closed(Type),
}

#[derive(Debug, Clone)]
pub struct SharedUpvalue(Arc<Mutex<Upvalue>>);

impl SharedUpvalue {
    pub fn new(uv: Upvalue) -> Self {
        SharedUpvalue(Arc::new(Mutex::new(uv)))
    }
    pub fn value(&self, context: &Context) -> Type {
        let _guard = self.lock();
        _guard.value(context)
    }
    pub fn next(&self) -> Option<SharedUpvalue> {
        let _guard = self.lock();
        _guard.next()
    }
    pub fn lock(&self) -> MutexGuard<Upvalue> {
        self.0.lock()
    }
}

impl PartialEq for SharedUpvalue {
    fn eq(&self, other: &SharedUpvalue) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Upvalue {
    pub fn value(&self, context: &Context) -> Type {
        // println!("value({:?})", self);
        let stack = &context.stack;
        match *self {
            Upvalue::Open { ref position, .. } => {
                stack.get(*position)
                    .unwrap_or(Type::Nil)
            }
            Upvalue::Closed(ref data) => data.clone()
        }
    }

    pub fn next(&self) -> Option<SharedUpvalue> {
        match *self {
            Upvalue::Open { ref next, .. } => {
                Some((*next).clone())
            },
            Upvalue::Closed(_) => None
        }
    }
}