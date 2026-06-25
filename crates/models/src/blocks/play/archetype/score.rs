use std::collections::BTreeMap;

use sonorust_ir::IRValue;
use tracing::warn;

use crate::{
    blocks::{ReadableBlock, WritableBlock},
    ids::ArchetypeId,
};

#[derive(Debug, Default)]
pub struct PlayArchetypeScore {
    pub items: BTreeMap<ArchetypeId, IRValue>,
}

impl PlayArchetypeScore {
    pub const BLOCK_ID: u64 = 5001;
    pub const DEFAULT_SCORE: IRValue = 1.0;
}

impl PlayArchetypeScore {
    pub fn new(archetype_count: usize) -> Self {
        Self {
            items: (0..archetype_count)
                .map(|i| (ArchetypeId(i), PlayArchetypeScore::DEFAULT_SCORE))
                .collect(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ArchetypeId, &IRValue)> {
        self.items.iter()
    }

    pub fn insert(&mut self, archetype_id: ArchetypeId, value: IRValue) {
        self.items.insert(archetype_id, value);
    }

    pub fn remove(&mut self, archetype_id: &ArchetypeId) {
        self.items.remove(archetype_id);
    }
}

impl ReadableBlock for PlayArchetypeScore {
    fn read(&self, index: usize) -> Option<IRValue> {
        match self.items.get(&ArchetypeId(index)) {
            Some(item) => Some(*item),
            None => {
                warn!("Attempted to read PlayArchetypeScore of non-existent index {index}");
                None
            }
        }
    }
}

impl WritableBlock for PlayArchetypeScore {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match self.items.get_mut(&ArchetypeId(index)) {
            Some(item) => {
                *item = value;
                true
            }
            None => {
                warn!("Attempted to write PlayArchetypeScore on non-existent index {index}");
                false
            }
        }
    }
}
