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
pub struct PlayRuntimeUiConfiguration {
    pub menu: PlayRuntimeUiConfigurationItem,
    pub judgment: PlayRuntimeUiConfigurationItem,
    pub combo: PlayRuntimeUiConfigurationItem,
    pub primary_metric: PlayRuntimeUiConfigurationItem,
    pub secondary_metric: PlayRuntimeUiConfigurationItem,
}

impl PlayRuntimeUiConfiguration {
    pub const BLOCK_ID: u64 = 1007;
    pub const SIZE: usize = PlayRuntimeUiConfigurationItem::SIZE * 5;

    pub const INDEX_MENU: usize = 0;
    pub const INDEX_JUDGMENT: usize = 1;
    pub const INDEX_COMBO: usize = 2;
    pub const INDEX_PRIMARY_METRIC: usize = 3;
    pub const INDEX_SECONDARY_METRIC: usize = 4;
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayRuntimeUiConfigurationItem {
    pub scale: IRValue,
    pub alpha: IRValue,
}

impl PlayRuntimeUiConfigurationItem {
    pub const SIZE: usize = 2;

    pub const INDEX_SCALE: usize = 0;
    pub const INDEX_ALPHA: usize = 1;
}

impl Default for PlayRuntimeUiConfigurationItem {
    fn default() -> Self {
        Self {
            alpha: 1.0,
            scale: 1.0,
        }
    }
}

impl ReadableBlock for PlayRuntimeUiConfigurationItem {
    fn read(&self, index: usize) -> Option<IRValue> {
        match index {
            Self::INDEX_SCALE => Some(self.scale),
            Self::INDEX_ALPHA => Some(self.alpha),
            other => {
                warn!(
                    "Attempted to read from out of bounds index {other} on PlayRuntimeUiConfigurationItem"
                );
                None
            }
        }
    }
}

impl WritableBlock for PlayRuntimeUiConfigurationItem {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match index {
            Self::INDEX_SCALE => {
                self.scale = value;
                true
            }
            Self::INDEX_ALPHA => {
                self.alpha = value;
                true
            }
            other => {
                warn!(
                    "Attempted to write to out of bounds index {other} on PlayRuntimeUiConfigurationItem"
                );
                false
            }
        }
    }
}

impl ReadableBlock for PlayRuntimeUiConfiguration {
    fn read(&self, index: usize) -> Option<IRValue> {
        let item_index = index / PlayRuntimeUiConfigurationItem::SIZE;
        let index_in_item = index % PlayRuntimeUiConfigurationItem::SIZE;
        match item_index {
            Self::INDEX_MENU => self.menu.read(index_in_item),
            Self::INDEX_JUDGMENT => self.menu.read(index_in_item),
            Self::INDEX_COMBO => self.menu.read(index_in_item),
            Self::INDEX_PRIMARY_METRIC => self.menu.read(index_in_item),
            Self::INDEX_SECONDARY_METRIC => self.menu.read(index_in_item),
            other => {
                warn!(
                    "Attempted to read PlayRuntimeUiConfigurationItem of non-existent index {other}"
                );
                None
            }
        }
    }
}

impl WritableBlock for PlayRuntimeUiConfiguration {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        let item_index = index / PlayRuntimeUiConfigurationItem::SIZE;
        let index_in_item = index % PlayRuntimeUiConfigurationItem::SIZE;
        match item_index {
            Self::INDEX_MENU => {
                self.menu.write(index_in_item, value);
                true
            }
            Self::INDEX_JUDGMENT => {
                self.menu.write(index_in_item, value);
                true
            }
            Self::INDEX_COMBO => {
                self.menu.write(index_in_item, value);
                true
            }
            Self::INDEX_PRIMARY_METRIC => {
                self.menu.write(index_in_item, value);
                true
            }
            Self::INDEX_SECONDARY_METRIC => {
                self.menu.write(index_in_item, value);
                true
            }
            other => {
                warn!(
                    "Attempted to write PlayRuntimeUiConfigurationItem to non-existent index {other}"
                );
                false
            }
        }
    }
}
