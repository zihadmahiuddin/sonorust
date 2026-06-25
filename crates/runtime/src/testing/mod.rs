use std::{cell::RefCell, collections::HashMap};

use crate::context::{CurrentEntity, MemoryAccess, RuntimeContext};

use sonorust_ir::IRValue;
use sonorust_models::ids::{ArchetypeId, EntityId};
use tracing::warn;

#[derive(Default)]
#[allow(dead_code)] // used in tests
pub struct TestingRuntimeContext {
    pub memory: TestingMemory,
}

impl<'a> TestingRuntimeContext {
    #[allow(dead_code)] // used in tests
    pub fn as_ctx(&'a self) -> RuntimeContext<'a> {
        RuntimeContext {
            current_entity: CurrentEntity {
                id: EntityId(0),
                archetype_id: ArchetypeId(0),
            },
            memory: &self.memory,
        }
    }
}

pub struct TestingMemory {
    pub read_only: HashMap<u64, Vec<IRValue>>,
    pub writable: HashMap<u64, RefCell<Vec<IRValue>>>,
}

impl Default for TestingMemory {
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

impl MemoryAccess for TestingMemory {
    fn read(&self, _ctx: &RuntimeContext, block_id: u64, index: usize) -> Option<IRValue> {
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

    fn write(
        &self,
        _ctx: &RuntimeContext,
        block_id: u64,
        index: usize,
        value: IRValue,
    ) -> Option<IRValue> {
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
