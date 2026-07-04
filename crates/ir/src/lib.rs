use std::ops::{Deref, DerefMut};

pub mod nodes;

pub type IRValue = f32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
pub struct IRIndex(usize);

impl From<usize> for IRIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Deref for IRIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IRIndex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn modulo(a: IRValue, n: IRValue) -> IRValue {
    ((a % n) + n) % n
}
