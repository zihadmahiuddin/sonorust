use std::collections::BTreeMap;

use sonorust_ir::IRValue;
use tracing::warn;

use crate::{
    blocks::{ReadableBlock, WritableBlock},
    ids::ArchetypeId,
};

#[derive(Debug)]
pub struct PlayArchetypeLife {
    pub items: BTreeMap<ArchetypeId, PlayArchetypeLifeItem>,
}

impl PlayArchetypeLife {
    pub const BLOCK_ID: u64 = 5000;

    pub fn new<'a>(entities: impl Iterator<Item = &'a ArchetypeId>) -> Self {
        Self {
            items: entities
                .map(|id| (*id, PlayArchetypeLifeItem::default()))
                .collect(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct PlayArchetypeLifeItem {
    pub perfect_life_increment: IRValue,
    pub great_life_increment: IRValue,
    pub good_life_increment: IRValue,
    pub miss_life_increment: IRValue,
}

impl PlayArchetypeLifeItem {
    pub const SIZE: usize = 4;

    pub const INDEX_PERFECT_LIFE_INCREMENT: usize = 0;
    pub const INDEX_GREAT_LIFE_INCREMENT: usize = 1;
    pub const INDEX_GOOD_LIFE_INCREMENT: usize = 2;
    pub const INDEX_MISS_LIFE_INCREMENT: usize = 3;
}

impl ReadableBlock for PlayArchetypeLifeItem {
    fn read(&self, index: usize) -> Option<IRValue> {
        match index {
            Self::INDEX_PERFECT_LIFE_INCREMENT => Some(self.perfect_life_increment),
            Self::INDEX_GREAT_LIFE_INCREMENT => Some(self.great_life_increment),
            Self::INDEX_GOOD_LIFE_INCREMENT => Some(self.good_life_increment),
            Self::INDEX_MISS_LIFE_INCREMENT => Some(self.miss_life_increment),
            other => {
                warn!(
                    "Attempted to read from out of bounds index {other} on PlayArchetypeLifeItem"
                );
                None
            }
        }
    }
}

impl WritableBlock for PlayArchetypeLifeItem {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match index {
            Self::INDEX_PERFECT_LIFE_INCREMENT => self.perfect_life_increment = value,
            Self::INDEX_GREAT_LIFE_INCREMENT => self.great_life_increment = value,
            Self::INDEX_GOOD_LIFE_INCREMENT => self.good_life_increment = value,
            Self::INDEX_MISS_LIFE_INCREMENT => self.miss_life_increment = value,
            other => {
                warn!("Attempted to write to out of bounds index {other} on PlayArchetypeLifeItem");
                return false;
            }
        }

        true
    }
}

impl ReadableBlock for PlayArchetypeLife {
    fn read(&self, index: usize) -> Option<IRValue> {
        let item_index = index / PlayArchetypeLifeItem::SIZE;
        let index_in_item = index % PlayArchetypeLifeItem::SIZE;
        match self.items.get(&ArchetypeId(item_index)) {
            Some(item) => item.read(index_in_item),
            None => {
                warn!("Attempted to read PlayArchetypeLife of non-existent index {item_index}");
                None
            }
        }
    }
}

impl WritableBlock for PlayArchetypeLife {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        let item_index = index / PlayArchetypeLifeItem::SIZE;
        let index_in_item = index % PlayArchetypeLifeItem::SIZE;
        match self.items.get_mut(&ArchetypeId(item_index)) {
            Some(item) => item.write(index_in_item, value),
            None => {
                warn!("Attempted to write PlayArchetypeLife on non-existent index {item_index}");
                false
            }
        }
    }
}
