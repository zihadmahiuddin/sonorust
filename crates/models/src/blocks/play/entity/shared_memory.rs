use sonorust_ir::IRValue;
use tracing::warn;

use crate::blocks::{ReadableBlock, WritableBlock};

#[derive(Debug)]
pub struct PlayEntitySharedMemoryArray {
    pub items: Vec<PlayEntitySharedMemory>,
}

impl PlayEntitySharedMemoryArray {
    pub const BLOCK_ID: u64 = 4102;

    pub fn new(entity_count: usize) -> Self {
        Self {
            items: vec![PlayEntitySharedMemory::default(); entity_count],
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayEntitySharedMemory(pub [IRValue; Self::SIZE]);

impl PlayEntitySharedMemory {
    pub const BLOCK_ID: u64 = 4002;
    pub const SIZE: usize = 32;
}

impl Default for PlayEntitySharedMemory {
    fn default() -> Self {
        Self([0.0; 32])
    }
}

impl ReadableBlock for PlayEntitySharedMemoryArray {
    fn read(&self, index: usize) -> Option<IRValue> {
        let item_index = index / PlayEntitySharedMemory::SIZE;
        let index_in_item = index % PlayEntitySharedMemory::SIZE;
        match self.items.get(item_index) {
            Some(item) => item.read(index_in_item),
            None => {
                warn!(
                    "Attempted to read PlayEntitySharedMemory of non-existent index {item_index}"
                );
                None
            }
        }
    }
}

impl WritableBlock for PlayEntitySharedMemoryArray {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        let item_index = index / PlayEntitySharedMemory::SIZE;
        let index_in_item = index % PlayEntitySharedMemory::SIZE;
        match self.items.get_mut(item_index) {
            Some(item) => item.write(index_in_item, value),
            None => {
                warn!(
                    "Attempted to write PlayEntitySharedMemory on non-existent index {item_index}"
                );
                false
            }
        }
    }
}

impl ReadableBlock for PlayEntitySharedMemory {
    fn read(&self, index: usize) -> Option<IRValue> {
        match self.0.get(index) {
            Some(value) => Some(*value),
            None => {
                warn!(
                    "Attempted to read from out of bounds index {index} on PlayEntitySharedMemory"
                );
                None
            }
        }
    }
}

impl WritableBlock for PlayEntitySharedMemory {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match self.0.get_mut(index) {
            Some(mut_value) => {
                *mut_value = value;
                true
            }
            None => {
                warn!(
                    "Attempted to write to out of bounds index {index} on PlayEntitySharedMemory"
                );
                false
            }
        }
    }
}
