use std::collections::BTreeMap;
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::fmt;
use std::cmp;
use types::*;
use parking_lot::{Mutex, MutexGuard};

pub type LuaTableRaw = BTreeMap<Type, Type>;

#[derive(Clone, Default)]
pub struct LuaTable (Shared<LuaTableRaw>);

impl LuaTable {
    pub fn new() -> Self {
        Self::default()
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

impl Ord for LuaTable {
    fn cmp(&self, other: &LuaTable) -> cmp::Ordering {
        if self == other {
            cmp::Ordering::Equal
        } else {
            let s = self.lock();
            let o = other.lock();
            Ord::cmp(&(*s), &(*o))
        }
    }
}

impl PartialOrd for LuaTable {
    fn partial_cmp(&self, other: &LuaTable) -> Option<cmp::Ordering> {
        if self == other {
            Some(cmp::Ordering::Equal)
        } else {
            let s = self.lock();
            let o = other.lock();
            PartialOrd::partial_cmp(&(*s), &(*o))
        }
    }
}

impl Hash for LuaTable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lock().hash(state)
    }
}

impl fmt::Debug for LuaTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}