use std::{cell::RefCell, collections::HashMap};

use crate::context::{MemoryAccess, RuntimeContext};

use tracing::warn;

#[derive(Default)]
#[allow(dead_code)] // used in tests
pub struct BasicRuntimeContext {
    pub memory: BasicMemory,
}

impl<'a> BasicRuntimeContext {
    #[allow(dead_code)] // used in tests
    pub fn as_ctx(&'a mut self) -> RuntimeContext<'a> {
        RuntimeContext {
            memory: &self.memory,
        }
    }
}

pub struct BasicMemory {
    pub read_only: HashMap<u64, Vec<f32>>,
    pub writable: HashMap<u64, RefCell<Vec<f32>>>,
}

impl Default for BasicMemory {
    fn default() -> Self {
        let mut read_only = HashMap::new();
        read_only.insert(1, vec![0.0; 4096]);
        let mut writable = HashMap::new();
        writable.insert(0, RefCell::new(vec![0.0; 4096]));
        Self {
            read_only,
            writable,
        }
    }
}

impl MemoryAccess for BasicMemory {
    fn read(&self, block_id: u64, index: usize) -> Option<f32> {
        if let Some(block) = self.writable.get(&block_id) {
            block
                .try_borrow()
                .expect("only 1 usage at a time")
                .get(index)
                .copied()
        } else if let Some(block) = self.read_only.get(&block_id) {
            block.get(index).copied()
        } else {
            None
        }
    }

    fn write(&self, block_id: u64, index: usize, value: f32) -> Option<f32> {
        if let Some(block) = self.writable.get(&block_id) {
            if let Some(old_value) = block
                .try_borrow_mut()
                .expect("only 1 usage at a time")
                .get_mut(index)
            {
                *old_value = value;
            } else {
                warn!("Failed to write to block {block_id} index {index}")
            }
            Some(value)
        } else {
            None
        }
    }
}
