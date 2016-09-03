use types::{Type, LuaTable, SharedType};
use std::sync::Arc;
use parking_lot::Mutex;

pub enum Environment {
    Empty,
    LuaStd,
    Custom(LuaTable),
}

impl Environment {
    pub fn make(&self) -> SharedType {
        let mut table = LuaTable::new();
        table.insert("nil_value".into(), Type::Nil);
        let as_type = Type::Table(table);
        Arc::new(Mutex::new(as_type))
    }
}
