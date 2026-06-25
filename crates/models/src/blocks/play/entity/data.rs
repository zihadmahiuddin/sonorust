use std::collections::BTreeMap;

use sonorust_ir::IRValue;
use tracing::warn;

use crate::{
    blocks::{ReadableBlock, WritableBlock},
    ids::EntityId,
};

#[derive(Debug)]
pub struct PlayEntityDataArray {
    pub items: BTreeMap<EntityId, PlayEntityData>,
}

impl PlayEntityDataArray {
    pub const BLOCK_ID: u64 = 4101;

    pub fn new(entities: impl Iterator<Item = (EntityId, PlayEntityData)>) -> Self {
        Self {
            items: entities.collect(),
        }
    }
}

impl ReadableBlock for PlayEntityDataArray {
    fn read(&self, index: usize) -> Option<IRValue> {
        let item_index = index / PlayEntityData::SIZE;
        let index_in_item = index % PlayEntityData::SIZE;
        match self.items.get(&EntityId(item_index)) {
            Some(entity_data) => entity_data.read(index_in_item),
            None => {
                warn!("Attempted to read EntityData of non-existing entity {item_index}");
                None
            }
        }
    }
}

impl WritableBlock for PlayEntityDataArray {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        let item_index = index / PlayEntityData::SIZE;
        let index_in_item = index % PlayEntityData::SIZE;
        match self.items.get_mut(&EntityId(item_index)) {
            Some(entity_data) => entity_data.write(index_in_item, value),
            None => {
                warn!("Attempted to write EntityData of non-existing entity {item_index}");
                false
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayEntityData(pub [IRValue; Self::SIZE]);

impl PlayEntityData {
    pub const BLOCK_ID: u64 = 4001;
    pub const SIZE: usize = 32;

    pub fn new(data: [IRValue; Self::SIZE]) -> PlayEntityData {
        Self(data)
    }
}

impl ReadableBlock for PlayEntityData {
    fn read(&self, index: usize) -> Option<IRValue> {
        match self.0.get(index) {
            Some(value) => Some(*value),
            None => {
                warn!("Attempted to read from out of bounds index {index} on PlayEntityData");
                None
            }
        }
    }
}

impl WritableBlock for PlayEntityData {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match self.0.get_mut(index) {
            Some(mut_value) => {
                *mut_value = value;
                true
            }
            None => {
                warn!("Attempted to write to out of bounds index {index} on PlayEntityData");
                false
            }
        }
    }
}
