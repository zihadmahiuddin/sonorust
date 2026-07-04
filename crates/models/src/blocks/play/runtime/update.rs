use sonorust_ir::IRValue;
use tracing::warn;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::blocks::ReadableBlock;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayRuntimeUpdate {
    pub time: IRValue,
    pub delta_time: IRValue,
    pub scaled_time: IRValue,
    pub touch_count: usize,
}

impl PlayRuntimeUpdate {
    pub const BLOCK_ID: u64 = 1001;
    pub const SIZE: usize = 4;

    pub const INDEX_TIME: usize = 0;
    pub const INDEX_DELTA_TIME: usize = 1;
    pub const INDEX_SCALED_TIME: usize = 2;
    pub const INDEX_TOUCH_COUNT: usize = 3;
}

impl ReadableBlock for PlayRuntimeUpdate {
    fn read(&self, index: usize) -> Option<IRValue> {
        match index {
            Self::INDEX_TIME => Some(self.time),
            Self::INDEX_DELTA_TIME => Some(self.delta_time),
            Self::INDEX_SCALED_TIME => Some(self.scaled_time),
            Self::INDEX_TOUCH_COUNT => Some(self.touch_count as IRValue),
            other => {
                warn!("Attempted to read from out of bounds index {other} on PlayRuntimeUpdate");
                None
            }
        }
    }
}
