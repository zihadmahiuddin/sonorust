use std::collections::BTreeSet;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::{
    blocks::{ReadableBlock, WritableBlock},
    ids::EntityId,
};

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayEntityDespawn {
    pub items: BTreeSet<EntityId>,
}

impl PlayEntityDespawn {
    pub const BLOCK_ID: u64 = 4004;
}

impl PlayEntityDespawn {
    pub fn iter(&self) -> impl Iterator<Item = &EntityId> {
        self.items.iter()
    }

    pub fn add(&mut self, entity_id: EntityId) {
        self.items.insert(entity_id);
    }

    pub fn remove(&mut self, entity_id: &EntityId) {
        self.items.remove(entity_id);
    }
}

impl ReadableBlock for PlayEntityDespawn {
    fn read(&self, index: usize) -> Option<sonorust_ir::IRValue> {
        Some(self.items.contains(&EntityId(index)).into())
    }
}

impl WritableBlock for PlayEntityDespawn {
    fn write(&mut self, index: usize, value: sonorust_ir::IRValue) -> bool {
        let entity_id = EntityId(index);
        if value != 0.0 {
            self.add(entity_id);
        } else {
            self.remove(&entity_id);
        }
        true
    }
}
