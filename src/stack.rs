use std::ops;
use std::ops::{Index, IndexMut};
use std::fmt;

use types::{Type, Representable};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct StackLevel {
    base: usize,
    index: usize
}

impl Into<usize> for StackLevel {
    fn into(self) -> usize {
        self.base + self.index
    }
}

pub trait StackIndex {
    fn idx(&self, base: usize) -> usize;
}

impl StackIndex for StackLevel {
    fn idx(&self, _: usize) -> usize {
        (*self).into()
    }
}

impl StackIndex for usize {
    fn idx(&self, base: usize) -> usize {
        base + self
    }
}

#[derive(Debug, Clone)]
pub enum StackEntry {
    Type(Type),
    ClosureBarrier,
}

impl StackEntry {
    pub fn as_type(&self) -> Type {
        match *self {
            StackEntry::Type(ref t) => t.clone(),
            _ => panic!("attempted to call as_type on {:?}", self)
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
    _stack: Vec<StackEntry>,
    _closure_base_cache: usize,
}

impl Stack {
    pub fn new() -> Self {
        Stack::default()
    }

    pub fn top(&self) -> usize {
        self._stack.len() - 1 - self._closure_base_cache
    }

    pub fn pop_barrier(&mut self) {
        while let Some(elem) = self._stack.pop() {
            if let StackEntry::ClosureBarrier = elem {
                self._calc_base();
                return
            }
        }
    }

    fn _calc_base(&mut self) {
        for (i, elem) in self._stack.iter().enumerate().rev() {
            if let StackEntry::ClosureBarrier = *elem {
                self._closure_base_cache = i + 1;
                return
            }
            if i == 0 {
                self._closure_base_cache = 0
            }
        }
    }
    
    pub fn insert_barrier(&mut self) {
        self._stack.push(StackEntry::ClosureBarrier);
        self._calc_base();
    }

    pub fn repr(&self) -> String {
        let elements: Vec<String> = self._stack.iter()
            .map(|e|
                match *e {
                    StackEntry::ClosureBarrier => "<Barrier />".to_owned(),
                    StackEntry::Type(ref t) => t.repr()
                }
            )
            .collect();
        format!("[{}]", elements.join(", "))
    }

    pub fn get<T: StackIndex>(&self, index: T) -> Option<Type> {
        let index = index.idx(self._closure_base_cache);
        self._stack.get(index)
            .and_then(|val| match *val {
                StackEntry::ClosureBarrier => None,
                StackEntry::Type(ref val) => Some(val.clone())
            })
    }

    pub fn get_level(&self, index: usize) -> StackLevel {
        StackLevel {
            base: self._closure_base_cache,
            index: index
        }
    }
}

impl Index<usize> for Stack {
    type Output = StackEntry;
    #[inline]
    fn index(&self, index: usize) -> &StackEntry {
        &self._stack[index + self._closure_base_cache]
    }
}

impl Index<ops::Range<usize>> for Stack {
    type Output = [StackEntry];

    #[inline]
    fn index(&self, index: ops::Range<usize>) -> &[StackEntry] {
        let range = index.start + self._closure_base_cache..index.end + self._closure_base_cache;
        Index::index(&*self._stack, range)
    }
}

impl IndexMut<usize> for Stack {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut StackEntry {
        let abs = index + self._closure_base_cache;
        if self._stack.len() <= abs {
            self._stack.push(StackEntry::Type(Type::Nil))
        }
        assert!(abs < self._stack.len());
        &mut self._stack[abs]
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&self._stack, f)
    }
}