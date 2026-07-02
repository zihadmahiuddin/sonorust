use std::collections::BTreeMap;

use sonorust_ir::IRValue;
use tracing::warn;

use crate::{
    blocks::{ReadableBlock, WritableBlock},
    ids::EntityId,
};

#[derive(Debug)]
pub struct PlayEntityScore {
    pub items: BTreeMap<EntityId, IRValue>,
}

impl PlayEntityScore {
    pub const BLOCK_ID: u64 = 4006;
    pub const DEFAULT: IRValue = 0.0;
}

impl PlayEntityScore {
    pub fn new<'a>(entities: impl Iterator<Item = &'a EntityId>) -> Self {
        Self {
            items: entities.map(|id| (*id, PlayEntityScore::DEFAULT)).collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&EntityId, &IRValue)> {
        self.items.iter()
    }

    pub fn insert(&mut self, entity_id: EntityId, value: IRValue) {
        self.items.insert(entity_id, value);
    }

    pub fn remove(&mut self, entity_id: &EntityId) {
        self.items.remove(entity_id);
    }
}

impl ReadableBlock for PlayEntityScore {
    fn read(&self, index: usize) -> Option<IRValue> {
        match self.items.get(&EntityId(index)) {
            Some(item) => Some(*item),
            None => {
                warn!("Attempted to read PlayEntityScore of non-existent index {index}");
                None
            }
        }
    }
}

impl WritableBlock for PlayEntityScore {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match self.items.get_mut(&EntityId(index)) {
            Some(item) => {
                *item = value;
                true
            }
            None => {
                warn!("Attempted to write PlayEntityScore on non-existent index {index}");
                false
            }
        }
    }
}
