use std::{cell::RefCell, collections::HashMap};

use crate::{
    access::{MemoryAccess, SideEffectAccess, TimingAccess},
    context::{CurrentEntity, RuntimeContext},
    side_effects::{DrawSideEffect, SpawnSideEffect},
};

use sonorust_ir::IRValue;
use sonorust_models::ids::{ArchetypeId, EntityId};
use tracing::warn;

#[derive(Default)]
#[allow(dead_code)] // used in tests
pub struct TestingRuntimeContext {
    pub memory: TestingMemory,
    pub timing: TestingTiming,
    pub side_effects: TestingSideEffects,
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
            timing: &self.timing,
            side_effects: &self.side_effects,
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

#[derive(Debug, Default)]
pub struct TestingTiming;

impl TimingAccess for TestingTiming {
    fn beat_to_time(&self, _beat: IRValue) -> IRValue {
        todo!()
    }

    fn beat_to_bpm(&self, _beat: IRValue) -> IRValue {
        todo!()
    }

    fn time_to_scaled_time(&self, _time: IRValue) -> IRValue {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct TestingSideEffects {
    spawns: RefCell<Vec<SpawnSideEffect>>,
    draws: RefCell<Vec<DrawSideEffect>>,
}

impl SideEffectAccess for TestingSideEffects {
    fn spawn(&self, spawn_side_effect: SpawnSideEffect) {
        self.spawns.borrow_mut().push(spawn_side_effect);
    }

    fn draw(&self, draw_side_effect: DrawSideEffect) {
        self.draws.borrow_mut().push(draw_side_effect);
    }
}
