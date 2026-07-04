use sonorust_ir::IRValue;
use tracing::warn;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::blocks::{ReadableBlock, WritableBlock};

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayLevelScore {
    pub perfect_multiplier: IRValue,
    pub great_multiplier: IRValue,
    pub good_multiplier: IRValue,
    pub consecutive_perfect_multiplier: IRValue,
    pub consecutive_perfect_step: IRValue,
    pub consecutive_perfect_cap: IRValue,
    pub consecutive_great_multiplier: IRValue,
    pub consecutive_great_step: IRValue,
    pub consecutive_great_cap: IRValue,
    pub consecutive_good_multiplier: IRValue,
    pub consecutive_good_step: IRValue,
    pub consecutive_good_cap: IRValue,
}

impl PlayLevelScore {
    pub const BLOCK_ID: u64 = 2004;
    pub const SIZE: usize = 12;

    pub const INDEX_PERFECT_MULTIPLIER: usize = 0;
    pub const INDEX_GREAT_MULTIPLIER: usize = 1;
    pub const INDEX_GOOD_MULTIPLIER: usize = 2;
    pub const INDEX_CONSECUTIVE_PERFECT_MULTIPLIER: usize = 3;
    pub const INDEX_CONSECUTIVE_PERFECT_STEP: usize = 4;
    pub const INDEX_CONSECUTIVE_PERFECT_CAP: usize = 5;
    pub const INDEX_CONSECUTIVE_GREAT_MULTIPLIER: usize = 6;
    pub const INDEX_CONSECUTIVE_GREAT_STEP: usize = 7;
    pub const INDEX_CONSECUTIVE_GREAT_CAP: usize = 8;
    pub const INDEX_CONSECUTIVE_GOOD_MULTIPLIER: usize = 9;
    pub const INDEX_CONSECUTIVE_GOOD_STEP: usize = 10;
    pub const INDEX_CONSECUTIVE_GOOD_CAP: usize = 11;
}

impl ReadableBlock for PlayLevelScore {
    fn read(&self, index: usize) -> Option<IRValue> {
        match index {
            Self::INDEX_PERFECT_MULTIPLIER => Some(self.perfect_multiplier),
            Self::INDEX_GREAT_MULTIPLIER => Some(self.great_multiplier),
            Self::INDEX_GOOD_MULTIPLIER => Some(self.good_multiplier),
            Self::INDEX_CONSECUTIVE_PERFECT_MULTIPLIER => Some(self.consecutive_perfect_multiplier),
            Self::INDEX_CONSECUTIVE_PERFECT_STEP => Some(self.consecutive_perfect_step),
            Self::INDEX_CONSECUTIVE_PERFECT_CAP => Some(self.consecutive_perfect_cap),
            Self::INDEX_CONSECUTIVE_GREAT_MULTIPLIER => Some(self.consecutive_great_multiplier),
            Self::INDEX_CONSECUTIVE_GREAT_STEP => Some(self.consecutive_great_step),
            Self::INDEX_CONSECUTIVE_GREAT_CAP => Some(self.consecutive_great_cap),
            Self::INDEX_CONSECUTIVE_GOOD_MULTIPLIER => Some(self.consecutive_good_multiplier),
            Self::INDEX_CONSECUTIVE_GOOD_STEP => Some(self.consecutive_good_step),
            Self::INDEX_CONSECUTIVE_GOOD_CAP => Some(self.consecutive_good_cap),
            other => {
                warn!("Attempted to read from out of bounds index {other} on PlayLevelScore");
                None
            }
        }
    }
}

impl WritableBlock for PlayLevelScore {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match index {
            Self::INDEX_PERFECT_MULTIPLIER => {
                self.perfect_multiplier = value;
                true
            }
            Self::INDEX_GREAT_MULTIPLIER => {
                self.great_multiplier = value;
                true
            }
            Self::INDEX_GOOD_MULTIPLIER => {
                self.good_multiplier = value;
                true
            }
            Self::INDEX_CONSECUTIVE_PERFECT_MULTIPLIER => {
                self.consecutive_perfect_multiplier = value;
                true
            }
            Self::INDEX_CONSECUTIVE_PERFECT_STEP => {
                self.consecutive_perfect_step = value;
                true
            }
            Self::INDEX_CONSECUTIVE_PERFECT_CAP => {
                self.consecutive_perfect_cap = value;
                true
            }
            Self::INDEX_CONSECUTIVE_GREAT_MULTIPLIER => {
                self.consecutive_great_multiplier = value;
                true
            }
            Self::INDEX_CONSECUTIVE_GREAT_STEP => {
                self.consecutive_great_step = value;
                true
            }
            Self::INDEX_CONSECUTIVE_GREAT_CAP => {
                self.consecutive_great_cap = value;
                true
            }
            Self::INDEX_CONSECUTIVE_GOOD_MULTIPLIER => {
                self.consecutive_good_multiplier = value;
                true
            }
            Self::INDEX_CONSECUTIVE_GOOD_STEP => {
                self.consecutive_good_step = value;
                true
            }
            Self::INDEX_CONSECUTIVE_GOOD_CAP => {
                self.consecutive_good_cap = value;
                true
            }
            other => {
                warn!("Attempted to write to out of bounds index {other} to PlayLevelScore");
                false
            }
        }
    }
}
