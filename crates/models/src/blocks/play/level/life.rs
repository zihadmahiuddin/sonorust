use sonorust_ir::IRValue;
use tracing::warn;

use crate::blocks::{ReadableBlock, WritableBlock};

#[derive(Debug)]
pub struct PlayLevelLife {
    pub consecutive_perfect_increment: IRValue,
    pub consecutive_perfect_step: IRValue,
    pub consecutive_great_increment: IRValue,
    pub consecutive_great_step: IRValue,
    pub consecutive_good_increment: IRValue,
    pub consecutive_good_step: IRValue,
}

impl PlayLevelLife {
    pub const BLOCK_ID: u64 = 2005;

    pub const INDEX_CONSECUTIVE_PERFECT_INCREMENT: usize = 0;
    pub const INDEX_CONSECUTIVE_PERFECT_STEP: usize = 1;
    pub const INDEX_CONSECUTIVE_GREAT_INCREMENT: usize = 2;
    pub const INDEX_CONSECUTIVE_GREAT_STEP: usize = 3;
    pub const INDEX_CONSECUTIVE_GOOD_INCREMENT: usize = 4;
    pub const INDEX_CONSECUTIVE_GOOD_STEP: usize = 5;
}

impl ReadableBlock for PlayLevelLife {
    fn read(&self, index: usize) -> Option<IRValue> {
        match index {
            Self::INDEX_CONSECUTIVE_PERFECT_INCREMENT => Some(self.consecutive_perfect_increment),
            Self::INDEX_CONSECUTIVE_PERFECT_STEP => Some(self.consecutive_perfect_step),
            Self::INDEX_CONSECUTIVE_GREAT_INCREMENT => Some(self.consecutive_great_increment),
            Self::INDEX_CONSECUTIVE_GREAT_STEP => Some(self.consecutive_great_step),
            Self::INDEX_CONSECUTIVE_GOOD_INCREMENT => Some(self.consecutive_good_increment),
            Self::INDEX_CONSECUTIVE_GOOD_STEP => Some(self.consecutive_good_step),
            other => {
                warn!("Attempted to read from out of bounds index {other} on PlayLevelLife");
                None
            }
        }
    }
}

impl WritableBlock for PlayLevelLife {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match index {
            Self::INDEX_CONSECUTIVE_PERFECT_INCREMENT => {
                self.consecutive_perfect_increment = value;
                true
            }
            Self::INDEX_CONSECUTIVE_PERFECT_STEP => {
                self.consecutive_perfect_step = value;
                true
            }
            Self::INDEX_CONSECUTIVE_GREAT_INCREMENT => {
                self.consecutive_great_increment = value;
                true
            }
            Self::INDEX_CONSECUTIVE_GREAT_STEP => {
                self.consecutive_great_step = value;
                true
            }
            Self::INDEX_CONSECUTIVE_GOOD_INCREMENT => {
                self.consecutive_good_increment = value;
                true
            }
            Self::INDEX_CONSECUTIVE_GOOD_STEP => {
                self.consecutive_good_step = value;
                true
            }
            other => {
                warn!("Attempted to write to out of bounds index {other} on PlayLevelLife");
                false
            }
        }
    }
}
