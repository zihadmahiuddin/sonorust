use sonorust_ir::IRValue;
use tracing::warn;

use crate::blocks::{ReadableBlock, WritableBlock};

#[derive(Debug)]
pub struct TemporaryMemory(pub [IRValue; 4096]);

impl Default for TemporaryMemory {
    fn default() -> Self {
        Self([0.0; 4096])
    }
}

impl TemporaryMemory {
    pub const BLOCK_ID: u64 = 10000;
}

impl ReadableBlock for TemporaryMemory {
    fn read(&self, index: usize) -> Option<IRValue> {
        match self.0.get(index) {
            Some(&value) => Some(value),
            None => {
                warn!("Attempted to read out of bounds index {index} on TemporaryMemory");
                None
            }
        }
    }
}

impl WritableBlock for TemporaryMemory {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match self.0.get_mut(index) {
            Some(old_value) => {
                *old_value = value;
                true
            }
            None => {
                warn!("Attempted to read out of bounds index {index} on TemporaryMemory");
                false
            }
        }
    }
}
