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
    pub initial_life: IRValue,
    pub max_life: IRValue,
}

impl Default for PlayLevelLife {
    fn default() -> Self {
        Self {
            consecutive_perfect_increment: 0.0,
            consecutive_perfect_step: 0.0,
            consecutive_great_increment: 0.0,
            consecutive_great_step: 0.0,
            consecutive_good_increment: 0.0,
            consecutive_good_step: 0.0,
            initial_life: 1000.0,
            max_life: 1000.0,
        }
    }
}

impl PlayLevelLife {
    pub const BLOCK_ID: u64 = 2005;

    pub const INDEX_CONSECUTIVE_PERFECT_INCREMENT: usize = 0;
    pub const INDEX_CONSECUTIVE_PERFECT_STEP: usize = 1;
    pub const INDEX_CONSECUTIVE_GREAT_INCREMENT: usize = 2;
    pub const INDEX_CONSECUTIVE_GREAT_STEP: usize = 3;
    pub const INDEX_CONSECUTIVE_GOOD_INCREMENT: usize = 4;
    pub const INDEX_CONSECUTIVE_GOOD_STEP: usize = 5;
    pub const INDEX_INITIAL_LIFE: usize = 6;
    pub const INDEX_MAX_LIFE: usize = 7;
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
            Self::INDEX_INITIAL_LIFE => Some(self.initial_life),
            Self::INDEX_MAX_LIFE => Some(self.max_life),
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
            Self::INDEX_INITIAL_LIFE => {
                self.initial_life = value;
                true
            }
            Self::INDEX_MAX_LIFE => {
                self.max_life = value;
                true
            }
            other => {
                warn!("Attempted to write to out of bounds index {other} on PlayLevelLife");
                false
            }
        }
    }
}
