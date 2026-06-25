use std::collections::BTreeMap;

use sonorust_ir::IRValue;
use tracing::warn;

use crate::{
    blocks::{ReadableBlock, WritableBlock},
    ids::EntityId,
};

#[derive(Debug)]
pub struct PlayEntityInput {
    pub items: BTreeMap<EntityId, PlayEntityInputItem>,
}

impl PlayEntityInput {
    pub const BLOCK_ID: u64 = 4005;
}

impl PlayEntityInput {
    pub fn new<'a>(entities: impl Iterator<Item = &'a EntityId>) -> Self {
        Self {
            items: entities
                .map(|id| (*id, PlayEntityInputItem::default()))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayEntityInputItem {
    pub judgment: IRValue,
    pub accuracy: IRValue,
    pub bucket_index: IRValue,
    pub bucket_value: IRValue,
}

impl PlayEntityInputItem {
    pub const SIZE: usize = 64;

    pub const INDEX_JUDGMENT: usize = 0;
    pub const INDEX_ACCURACY: usize = 1;
    pub const INDEX_BUCKET_INDEX: usize = 2;
    pub const INDEX_BUCKET_VALUE: usize = 3;
}

impl Default for PlayEntityInputItem {
    fn default() -> Self {
        Self {
            bucket_index: -1.0,
            accuracy: 0.0,
            bucket_value: 0.0,
            judgment: 0.0,
        }
    }
}

impl ReadableBlock for PlayEntityInputItem {
    fn read(&self, index: usize) -> Option<IRValue> {
        match index {
            Self::INDEX_JUDGMENT => Some(self.judgment),
            Self::INDEX_ACCURACY => Some(self.accuracy),
            Self::INDEX_BUCKET_INDEX => Some(self.bucket_index),
            Self::INDEX_BUCKET_VALUE => Some(self.bucket_value),
            other => {
                warn!("Attempted to read from out of bounds index {other} on PlayEntityInputItem");
                None
            }
        }
    }
}

impl WritableBlock for PlayEntityInputItem {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match index {
            Self::INDEX_JUDGMENT => self.judgment = value,
            Self::INDEX_ACCURACY => self.accuracy = value,
            Self::INDEX_BUCKET_INDEX => self.bucket_index = value,
            Self::INDEX_BUCKET_VALUE => self.bucket_value = value,
            other => {
                warn!("Attempted to write to out of bounds index {other} on PlayEntityInputItem");
                return false;
            }
        }

        true
    }
}

impl ReadableBlock for PlayEntityInput {
    fn read(&self, index: usize) -> Option<IRValue> {
        let item_index = index / PlayEntityInputItem::SIZE;
        let index_in_item = index % PlayEntityInputItem::SIZE;
        match self.items.get(&EntityId(item_index)) {
            Some(item) => item.read(index_in_item),
            None => {
                warn!("Attempted to read PlayEntityInput of non-existent index {item_index}");
                None
            }
        }
    }
}

impl WritableBlock for PlayEntityInput {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        let item_index = index / PlayEntityInputItem::SIZE;
        let index_in_item = index % PlayEntityInputItem::SIZE;
        match self.items.get_mut(&EntityId(item_index)) {
            Some(item) => item.write(index_in_item, value),
            None => {
                warn!("Attempted to write PlayEntityInput on non-existent index {item_index}");
                false
            }
        }
    }
}
