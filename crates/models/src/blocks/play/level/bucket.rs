use sonorust_ir::IRValue;
use tracing::warn;

use crate::blocks::{ReadableBlock, WritableBlock};

#[derive(Debug)]
pub struct PlayLevelBucket(pub Vec<PlayLevelBucketItem>);

#[derive(Debug)]
pub struct PlayLevelBucketItem {
    pub min_perfect_window: IRValue,
    pub max_perfect_window: IRValue,
    pub min_great_window: IRValue,
    pub max_great_window: IRValue,
    pub min_good_window: IRValue,
    pub max_good_window: IRValue,
}

impl PlayLevelBucketItem {
    pub const SIZE: usize = 6;

    pub const INDEX_MIN_PERFECT_WINDOW: usize = 0;
    pub const INDEX_MAX_PERFECT_WINDOW: usize = 1;
    pub const INDEX_MIN_GREAT_WINDOW: usize = 2;
    pub const INDEX_MAX_GREAT_WINDOW: usize = 3;
    pub const INDEX_MIN_GOOD_WINDOW: usize = 4;
    pub const INDEX_MAX_GOOD_WINDOW: usize = 5;
}

impl PlayLevelBucket {
    pub const BLOCK_ID: u64 = 2003;
}

impl ReadableBlock for PlayLevelBucketItem {
    fn read(&self, index: usize) -> Option<IRValue> {
        match index {
            Self::INDEX_MIN_PERFECT_WINDOW => Some(self.min_perfect_window),
            Self::INDEX_MAX_PERFECT_WINDOW => Some(self.max_perfect_window),
            Self::INDEX_MIN_GREAT_WINDOW => Some(self.min_great_window),
            Self::INDEX_MAX_GREAT_WINDOW => Some(self.max_great_window),
            Self::INDEX_MIN_GOOD_WINDOW => Some(self.min_good_window),
            Self::INDEX_MAX_GOOD_WINDOW => Some(self.max_good_window),
            other => {
                warn!("Attempted to read from out of bounds index {other} on PlayLevelBucketItem");
                None
            }
        }
    }
}

impl WritableBlock for PlayLevelBucketItem {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match index {
            Self::INDEX_MIN_PERFECT_WINDOW => {
                self.min_perfect_window = value;
                true
            }
            Self::INDEX_MAX_PERFECT_WINDOW => {
                self.max_perfect_window = value;
                true
            }
            Self::INDEX_MIN_GREAT_WINDOW => {
                self.min_great_window = value;
                true
            }
            Self::INDEX_MAX_GREAT_WINDOW => {
                self.max_great_window = value;
                true
            }
            Self::INDEX_MIN_GOOD_WINDOW => {
                self.min_good_window = value;
                true
            }
            Self::INDEX_MAX_GOOD_WINDOW => {
                self.max_good_window = value;
                true
            }
            other => {
                warn!("Attempted to write to out of bounds index {other} to PlayLevelBucketItem");
                false
            }
        }
    }
}

impl ReadableBlock for PlayLevelBucket {
    fn read(&self, index: usize) -> Option<IRValue> {
        let item_index = index / PlayLevelBucketItem::SIZE;
        let index_in_item = index % PlayLevelBucketItem::SIZE;
        match self.0.get(item_index) {
            Some(item) => item.read(index_in_item),
            None => {
                warn!("Attempted to read PlayLevelBucketItem of non-existent index {item_index}");
                None
            }
        }
    }
}

impl WritableBlock for PlayLevelBucket {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        let item_index = index / PlayLevelBucketItem::SIZE;
        let index_in_item = index % PlayLevelBucketItem::SIZE;
        match self.0.get_mut(item_index) {
            Some(item) => item.write(index_in_item, value),
            None => {
                warn!("Attempted to write PlayLevelBucketItem on non-existent index {item_index}");
                false
            }
        }
    }
}
