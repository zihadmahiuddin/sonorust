use std::collections::BTreeMap;

use sonorust_ir::IRValue;
use tracing::warn;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::{
    blocks::{ReadableBlock, WritableBlock},
    ids::ArchetypeId,
};

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayArchetypeScore {
    pub items: BTreeMap<ArchetypeId, IRValue>,
}

impl PlayArchetypeScore {
    pub const BLOCK_ID: u64 = 5001;
    pub const DEFAULT: IRValue = 1.0;
}

impl PlayArchetypeScore {
    pub fn new<'a>(entities: impl Iterator<Item = &'a ArchetypeId>) -> Self {
        Self {
            items: entities
                .map(|id| (*id, PlayArchetypeScore::DEFAULT))
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
