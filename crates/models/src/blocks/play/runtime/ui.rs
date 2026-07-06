use glam::Vec2;
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
pub struct PlayRuntimeUi {
    pub menu: PlayRuntimeUiItem,
    pub judgment: PlayRuntimeUiItem,
    pub combo_value: PlayRuntimeUiItem,
    pub combo_text: PlayRuntimeUiItem,
    pub primary_metric_bar: PlayRuntimeUiItem,
    pub primary_metric_value: PlayRuntimeUiItem,
    pub secondary_metric_bar: PlayRuntimeUiItem,
    pub secondary_metric_value: PlayRuntimeUiItem,
}

impl PlayRuntimeUi {
    pub const BLOCK_ID: u64 = 1006;

    pub const INDEX_MENU: usize = 0;
    pub const INDEX_JUDGMENT: usize = 1;
    pub const INDEX_COMBO_VALUE: usize = 2;
    pub const INDEX_COMBO_TEXT: usize = 3;
    pub const INDEX_PRIMARY_METRIC_BAR: usize = 4;
    pub const INDEX_PRIMARY_METRIC_VALUE: usize = 5;
    pub const INDEX_SECONDARY_METRIC_BAR: usize = 6;
    pub const INDEX_SECONDARY_METRIC_VALUE: usize = 7;
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayRuntimeUiItem {
    pub anchor: Vec2,
    pub pivot: Vec2,
    pub width: IRValue,
    pub height: IRValue,
    pub rotation: IRValue,
    pub alpha: IRValue,
    pub horizontal_alignment: HorizontalAlignment,
    pub background: IRValue,
}

impl PlayRuntimeUiItem {
    pub const SIZE: usize = 10;

    pub const INDEX_ANCHOR_X: usize = 0;
    pub const INDEX_ANCHOR_Y: usize = 1;
    pub const INDEX_PIVOT_X: usize = 2;
    pub const INDEX_PIVOT_Y: usize = 3;
    pub const INDEX_WIDTH: usize = 4;
    pub const INDEX_HEIGHT: usize = 5;
    pub const INDEX_ROTATION: usize = 6;
    pub const INDEX_ALPHA: usize = 7;
    pub const INDEX_HORIZONTAL_ALIGNMENT: usize = 8;
    pub const INDEX_BACKGROUND: usize = 9;
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum HorizontalAlignment {
    Left,
    #[default]
    Center,
    Right,
}

impl From<HorizontalAlignment> for IRValue {
    fn from(value: HorizontalAlignment) -> Self {
        match value {
            HorizontalAlignment::Left => -1.0,
            HorizontalAlignment::Center => 0.0,
            HorizontalAlignment::Right => 1.0,
        }
    }
}

impl From<IRValue> for HorizontalAlignment {
    fn from(value: IRValue) -> Self {
        match value {
            -1.0 => HorizontalAlignment::Left,
            0.0 => HorizontalAlignment::Center,
            1.0 => HorizontalAlignment::Right,
            _ => unreachable!(),
        }
    }
}

impl ReadableBlock for PlayRuntimeUiItem {
    fn read(&self, index: usize) -> Option<IRValue> {
        match index {
            Self::INDEX_ANCHOR_X => Some(self.anchor.x),
            Self::INDEX_ANCHOR_Y => Some(self.anchor.y),
            Self::INDEX_PIVOT_X => Some(self.pivot.x),
            Self::INDEX_PIVOT_Y => Some(self.pivot.y),
            Self::INDEX_WIDTH => Some(self.width),
            Self::INDEX_HEIGHT => Some(self.height),
            Self::INDEX_ROTATION => Some(self.rotation),
            Self::INDEX_ALPHA => Some(self.alpha),
            Self::INDEX_HORIZONTAL_ALIGNMENT => Some(self.horizontal_alignment.into()),
            Self::INDEX_BACKGROUND => Some(self.background),
            other => {
                warn!("Attempted to read from out of bounds index {other} on PlayRuntimeUiItem");
                None
            }
        }
    }
}

impl WritableBlock for PlayRuntimeUiItem {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match index {
            Self::INDEX_ANCHOR_X => {
                self.anchor.x = value;
                true
            }
            Self::INDEX_ANCHOR_Y => {
                self.anchor.y = value;
                true
            }
            Self::INDEX_PIVOT_X => {
                self.pivot.x = value;
                true
            }
            Self::INDEX_PIVOT_Y => {
                self.pivot.y = value;
                true
            }
            Self::INDEX_WIDTH => {
                self.width = value;
                true
            }
            Self::INDEX_HEIGHT => {
                self.height = value;
                true
            }
            Self::INDEX_ROTATION => {
                self.rotation = value;
                true
            }
            Self::INDEX_ALPHA => {
                self.alpha = value;
                true
            }
            Self::INDEX_HORIZONTAL_ALIGNMENT => {
                self.horizontal_alignment = value.into();
                true
            }
            Self::INDEX_BACKGROUND => {
                self.background = value;
                true
            }
            other => {
                warn!("Attempted to write to out of bounds index {other} on PlayRuntimeUiItem");
                false
            }
        }
    }
}

impl ReadableBlock for PlayRuntimeUi {
    fn read(&self, index: usize) -> Option<IRValue> {
        let item_index = index / PlayRuntimeUiItem::SIZE;
        let index_in_item = index % PlayRuntimeUiItem::SIZE;
        match item_index {
            Self::INDEX_MENU => self.menu.read(index_in_item),
            Self::INDEX_JUDGMENT => self.judgment.read(index_in_item),
            Self::INDEX_COMBO_VALUE => self.combo_value.read(index_in_item),
            Self::INDEX_COMBO_TEXT => self.combo_text.read(index_in_item),
            Self::INDEX_PRIMARY_METRIC_BAR => self.primary_metric_bar.read(index_in_item),
            Self::INDEX_PRIMARY_METRIC_VALUE => self.primary_metric_value.read(index_in_item),
            Self::INDEX_SECONDARY_METRIC_BAR => self.secondary_metric_bar.read(index_in_item),
            Self::INDEX_SECONDARY_METRIC_VALUE => self.secondary_metric_value.read(index_in_item),
            other => {
                warn!("Attempted to read PlayRuntimeUiItem of non-existent index {other}");
                None
            }
        }
    }
}

impl WritableBlock for PlayRuntimeUi {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        let item_index = index / PlayRuntimeUiItem::SIZE;
        let index_in_item = index % PlayRuntimeUiItem::SIZE;
        match item_index {
            Self::INDEX_MENU => {
                self.menu.write(index_in_item, value);
                true
            }
            Self::INDEX_JUDGMENT => {
                self.judgment.write(index_in_item, value);
                true
            }
            Self::INDEX_COMBO_VALUE => {
                self.combo_value.write(index_in_item, value);
                true
            }
            Self::INDEX_COMBO_TEXT => {
                self.combo_text.write(index_in_item, value);
                true
            }
            Self::INDEX_PRIMARY_METRIC_BAR => {
                self.primary_metric_bar.write(index_in_item, value);
                true
            }
            Self::INDEX_PRIMARY_METRIC_VALUE => {
                self.primary_metric_value.write(index_in_item, value);
                true
            }
            Self::INDEX_SECONDARY_METRIC_BAR => {
                self.secondary_metric_bar.write(index_in_item, value);
                true
            }
            Self::INDEX_SECONDARY_METRIC_VALUE => {
                self.secondary_metric_value.write(index_in_item, value);
                true
            }
            other => {
                warn!("Attempted to write PlayRuntimeUiItem to non-existent index {other}");
                false
            }
        }
    }
}
