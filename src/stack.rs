use std::ops;
use std::ops::{Index, IndexMut};
use std::fmt;

use types::{Type, SharedType};

#[derive(Debug, Clone)]
pub enum StackEntry {
    Type(Type),
    SharedType(SharedType),
    // TODO: stack barriers / guards?
}

impl StackEntry {
    pub fn as_type(&self) -> Type {
        match *self {
            StackEntry::Type(ref t) => t.clone(),
            StackEntry::SharedType(ref t) => (*t).lock().clone(),
        }
    }
}

impl From<Type> for StackEntry {
    fn from(t: Type) -> Self {
        StackEntry::Type(t)
    }
}

#[derive(Clone, Default)]
pub struct Stack {
    _stack: Vec<StackEntry>
}

impl Stack {
    pub fn new() -> Self {
        Stack::default()
    }
    pub fn top(&self) -> usize {
        self._stack.len() - 1
    }
}

impl Index<usize> for Stack {
    type Output = StackEntry;
    #[inline]
    fn index(&self, index: usize) -> &StackEntry {
        &self._stack[index]
    }
}

impl Index<ops::Range<usize>> for Stack {
    type Output = [StackEntry];

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &[StackEntry] {
        Index::index(&*self._stack, index)
    }
}

impl IndexMut<usize> for Stack {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut StackEntry {
        if self._stack.len() < index + 1 {
            self._stack.push(StackEntry::Type(Type::Nil))
        }
        &mut self._stack[index]
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&self._stack, f)
    }
}