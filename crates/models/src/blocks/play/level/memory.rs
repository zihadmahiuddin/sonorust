use sonorust_ir::IRValue;
use tracing::warn;

use crate::blocks::{ReadableBlock, WritableBlock};

#[derive(Debug)]
pub struct PlayLevelMemory(pub [IRValue; 4096]);

impl PlayLevelMemory {
    pub const BLOCK_ID: u64 = 2000;
}

impl ReadableBlock for PlayLevelMemory {
    fn read(&self, index: usize) -> Option<IRValue> {
        match self.0.get(index) {
            Some(value) => Some(*value),
            None => {
                warn!("Attempted to read from out of bounds index {index} on PlayLevelMemory");
                None
            }
        }
    }
}

impl WritableBlock for PlayLevelMemory {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match self.0.get_mut(index) {
            Some(mut_value) => {
                *mut_value = value;
                true
            }
            None => {
                warn!("Attempted to write to out of bounds index {index} on PlayLevelMemory");
                false
            }
        }
    }
}
