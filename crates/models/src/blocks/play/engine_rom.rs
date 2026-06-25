use sonorust_ir::IRValue;
use tracing::warn;

use crate::blocks::ReadableBlock;

#[derive(Debug)]
pub struct PlayEngineRom(pub Box<[IRValue]>);

impl PlayEngineRom {
    pub const BLOCK_ID: u64 = 3000;
}

impl ReadableBlock for PlayEngineRom {
    fn read(&self, index: usize) -> Option<IRValue> {
        match self.0.get(index) {
            Some(value) => Some(*value),
            None => {
                warn!("Attempted to read from out of bounds index {index} on PlayEngineRome");
                None
            }
        }
    }
}
