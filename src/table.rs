use std::collections::HashMap;
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::fmt;
use types::*;
use parking_lot::{Mutex, MutexGuard};

pub type LuaTableRaw = HashMap<Type, Type>;

#[derive(Clone)]
pub struct LuaTable (Shared<LuaTableRaw>);

impl LuaTable {
    pub fn new() -> Self {
        let table: LuaTableRaw = HashMap::new();
        LuaTable (Arc::new(Mutex::new(table)))
    }

    pub fn lock<'a>(&'a self) -> MutexGuard<'a, LuaTableRaw> {
        self.0.lock()
    }
}

impl From<LuaTableRaw> for LuaTable {
    fn from(table: LuaTableRaw) -> Self {
        LuaTable (Arc::new(Mutex::new(table)))
    }
}

impl Eq for LuaTable {}
impl PartialEq for LuaTable {
    fn eq(&self, other: &LuaTable) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Hash for LuaTable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unimplemented!();
    }
}

impl fmt::Debug for LuaTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}