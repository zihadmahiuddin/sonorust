use std::collections::BTreeMap;

use sonorust_ir::IRValue;
use tracing::warn;

use crate::{
    blocks::{ReadableBlock, WritableBlock},
    ids::EntityId,
};

#[derive(Debug)]
pub struct PlayEntityLife {
    pub items: BTreeMap<EntityId, PlayEntityLifeItem>,
}

impl PlayEntityLife {
    pub const BLOCK_ID: u64 = 4007;

    pub fn new<'a>(entities: impl Iterator<Item = &'a EntityId>) -> Self {
        Self {
            items: entities
                .map(|id| (*id, PlayEntityLifeItem::default()))
                .collect(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PlayEntityLifeItem {
    pub perfect_life_increment: IRValue,
    pub great_life_increment: IRValue,
    pub good_life_increment: IRValue,
    pub miss_life_increment: IRValue,
}

impl PlayEntityLifeItem {
    pub const SIZE: usize = 4;

    pub const INDEX_PERFECT_LIFE_INCREMENT: usize = 0;
    pub const INDEX_GREAT_LIFE_INCREMENT: usize = 1;
    pub const INDEX_GOOD_LIFE_INCREMENT: usize = 2;
    pub const INDEX_MISS_LIFE_INCREMENT: usize = 3;
}

impl ReadableBlock for PlayEntityLifeItem {
    fn read(&self, index: usize) -> Option<IRValue> {
        match index {
            Self::INDEX_PERFECT_LIFE_INCREMENT => Some(self.perfect_life_increment),
            Self::INDEX_GREAT_LIFE_INCREMENT => Some(self.great_life_increment),
            Self::INDEX_GOOD_LIFE_INCREMENT => Some(self.good_life_increment),
            Self::INDEX_MISS_LIFE_INCREMENT => Some(self.miss_life_increment),
            other => {
                warn!("Attempted to read from out of bounds index {other} on PlayEntityLifeItem");
                None
            }
        }
    }
}

impl WritableBlock for PlayEntityLifeItem {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match index {
            Self::INDEX_PERFECT_LIFE_INCREMENT => self.perfect_life_increment = value,
            Self::INDEX_GREAT_LIFE_INCREMENT => self.great_life_increment = value,
            Self::INDEX_GOOD_LIFE_INCREMENT => self.good_life_increment = value,
            Self::INDEX_MISS_LIFE_INCREMENT => self.miss_life_increment = value,
            other => {
                warn!("Attempted to write to out of bounds index {other} on PlayEntityLifeItem");
                return false;
            }
        }

        true
    }
}

impl ReadableBlock for PlayEntityLife {
    fn read(&self, index: usize) -> Option<IRValue> {
        let item_index = index / PlayEntityLifeItem::SIZE;
        let index_in_item = index % PlayEntityLifeItem::SIZE;
        match self.items.get(&EntityId(item_index)) {
            Some(item) => item.read(index_in_item),
            None => {
                warn!("Attempted to read PlayEntityLife of non-existent index {item_index}");
                None
            }
        }
    }
}

impl WritableBlock for PlayEntityLife {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        let item_index = index / PlayEntityLifeItem::SIZE;
        let index_in_item = index % PlayEntityLifeItem::SIZE;
        match self.items.get_mut(&EntityId(item_index)) {
            Some(item) => item.write(index_in_item, value),
            None => {
                warn!("Attempted to write PlayEntityLife on non-existent index {item_index}");
                false
            }
        }
    }
}
