use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

pub mod nodes;

pub type IRValue = f32;

#[cfg(feature = "wasm")]
#[cfg_attr(feature = "wasm", wasm_bindgen(typescript_custom_section))]
const TS_IR_VALUE_TYPE: &'static str = r#"
export type IRValue = number;
"#;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify), wasm_bindgen)]
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
