use sonorust_ir::IRValue;
use tracing::warn;

use crate::blocks::ReadableBlock;

#[derive(Debug)]
pub struct PlayLevelOption(pub Vec<IRValue>);

impl PlayLevelOption {
    pub const BLOCK_ID: u64 = 2002;
}

impl ReadableBlock for PlayLevelOption {
    fn read(&self, index: usize) -> Option<IRValue> {
        match self.0.get(index) {
            Some(value) => Some(*value),
            None => {
                warn!("Attempted to read from out of bounds index {index} on PlayLevelOption");
                None
            }
        }
    }
}
