use glam::Mat4;
use tracing::warn;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::blocks::{ReadableBlock, WritableBlock};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayRuntimeSkinTransform(pub Mat4);

impl PlayRuntimeSkinTransform {
    pub const BLOCK_ID: u64 = 1003;
}

impl ReadableBlock for PlayRuntimeSkinTransform {
    fn read(&self, index: usize) -> Option<sonorust_ir::IRValue> {
        match self.0.to_cols_array().get(index) {
            Some(&value) => Some(value),
            None => {
                warn!(
                    "Attempted to read from out of bounds index {index} to PlayRuntimeSkinTransform"
                );
                None
            }
        }
    }
}

impl WritableBlock for PlayRuntimeSkinTransform {
    fn write(&mut self, index: usize, value: sonorust_ir::IRValue) -> bool {
        match self.0.to_cols_array().get_mut(index) {
            Some(mut_value) => {
                *mut_value = value;
                true
            }
            None => {
                warn!(
                    "Attempted to write to out of bounds index {index} to PlayRuntimeSkinTransform"
                );
                false
            }
        }
    }
}
