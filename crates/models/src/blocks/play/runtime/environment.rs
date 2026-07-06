use glam::Vec2;
use sonorust_ir::IRValue;
use tracing::warn;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::blocks::{ReadableBlock, WritableBlock};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayRuntimeEnvironment {
    pub debug_mode: bool,
    pub screen_aspect_ratio: IRValue,
    pub audio_offset: IRValue,
    pub input_offset: IRValue,
    pub multiplayer: bool,
    pub safe_area_min: Vec2,
    pub safe_area_max: Vec2,
}

impl PlayRuntimeEnvironment {
    pub const BLOCK_ID: u64 = 1000;

    pub const INDEX_DEBUG_MODE: usize = 0;
    pub const INDEX_SCREEN_ASPECT_RATIO: usize = 1;
    pub const INDEX_AUDIO_OFFSET: usize = 2;
    pub const INDEX_INPUT_OFFSET: usize = 3;
    pub const INDEX_MULTIPLAYER: usize = 4;
    pub const INDEX_SAFE_AREA_MIN_X: usize = 5;
    pub const INDEX_SAFE_AREA_MAX_X: usize = 6;
    pub const INDEX_SAFE_AREA_MIN_Y: usize = 7;
    pub const INDEX_SAFE_AREA_MAX_Y: usize = 8;
}

impl ReadableBlock for PlayRuntimeEnvironment {
    fn read(&self, index: usize) -> Option<IRValue> {
        match index {
            Self::INDEX_DEBUG_MODE => Some(IRValue::from(self.debug_mode)),
            Self::INDEX_SCREEN_ASPECT_RATIO => Some(self.screen_aspect_ratio),
            Self::INDEX_AUDIO_OFFSET => Some(self.audio_offset),
            Self::INDEX_INPUT_OFFSET => Some(self.input_offset),
            Self::INDEX_MULTIPLAYER => Some(IRValue::from(self.multiplayer)),
            Self::INDEX_SAFE_AREA_MIN_X => Some(self.safe_area_min.x),
            Self::INDEX_SAFE_AREA_MAX_X => Some(self.safe_area_max.x),
            Self::INDEX_SAFE_AREA_MIN_Y => Some(self.safe_area_min.y),
            Self::INDEX_SAFE_AREA_MAX_Y => Some(self.safe_area_max.y),
            other => {
                warn!(
                    "Attempted to read from out of bounds index {other} on PlayRuntimeEnvironment"
                );
                None
            }
        }
    }
}

impl WritableBlock for PlayRuntimeEnvironment {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match index {
            Self::INDEX_DEBUG_MODE => self.debug_mode = value != 0.0,
            Self::INDEX_SCREEN_ASPECT_RATIO => self.screen_aspect_ratio = value,
            Self::INDEX_AUDIO_OFFSET => self.audio_offset = value,
            Self::INDEX_INPUT_OFFSET => self.input_offset = value,
            Self::INDEX_MULTIPLAYER => self.multiplayer = value != 0.0,
            Self::INDEX_SAFE_AREA_MIN_X => self.safe_area_min.x = value,
            Self::INDEX_SAFE_AREA_MAX_X => self.safe_area_max.x = value,
            Self::INDEX_SAFE_AREA_MIN_Y => self.safe_area_min.y = value,
            Self::INDEX_SAFE_AREA_MAX_Y => self.safe_area_max.y = value,
            other => {
                warn!(
                    "Attempted to write to out of bounds index {other} on PlayRuntimeEnvironment"
                );
                return false;
            }
        }

        true
    }
}
