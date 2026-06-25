use glam::Mat4;
use tracing::warn;

use crate::blocks::{ReadableBlock, WritableBlock};

#[derive(Debug)]
pub struct PlayRuntimeParticleTransform(pub Mat4);

impl PlayRuntimeParticleTransform {
    pub const BLOCK_ID: u64 = 1004;
}

impl ReadableBlock for PlayRuntimeParticleTransform {
    fn read(&self, index: usize) -> Option<sonorust_ir::IRValue> {
        match self.0.to_cols_array().get(index) {
            Some(&value) => Some(value),
            None => {
                warn!(
                    "Attempted to read from out of bounds index {index} to PlayRuntimeParticleTransform"
                );
                None
            }
        }
    }
}

impl WritableBlock for PlayRuntimeParticleTransform {
    fn write(&mut self, index: usize, value: sonorust_ir::IRValue) -> bool {
        match self.0.to_cols_array().get_mut(index) {
            Some(mut_value) => {
                *mut_value = value;
                true
            }
            None => {
                warn!(
                    "Attempted to write to out of bounds index {index} to PlayRuntimeParticleTransform"
                );
                false
            }
        }
    }
}
