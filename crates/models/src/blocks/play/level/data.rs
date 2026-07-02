use sonorust_ir::IRValue;
use tracing::warn;

use crate::blocks::{ReadableBlock, WritableBlock};

#[derive(Debug)]
pub struct PlayLevelData(pub [IRValue; 4096]);

impl PlayLevelData {
    pub const BLOCK_ID: u64 = 2001;
}

impl Default for PlayLevelData {
    fn default() -> Self {
        Self([0.0; 4096])
    }
}

impl ReadableBlock for PlayLevelData {
    fn read(&self, index: usize) -> Option<IRValue> {
        match self.0.get(index) {
            Some(value) => Some(*value),
            None => {
                warn!("Attempted to read from out of bounds index {index} on PlayLevelData");
                None
            }
        }
    }
}

impl WritableBlock for PlayLevelData {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match self.0.get_mut(index) {
            Some(mut_value) => {
                *mut_value = value;
                true
            }
            None => {
                warn!("Attempted to write to out of bounds index {index} on PlayLevelData");
                false
            }
        }
    }
}
