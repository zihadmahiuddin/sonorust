use std::collections::BTreeMap;

use sonorust_ir::IRValue;
use tracing::warn;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::{
    blocks::{ReadableBlock, WritableBlock},
    ids::EntityId,
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayEntityMemory(
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::serde::deserialize_array"),
        serde(serialize_with = "crate::serde::serialize_array")
    )]
    pub [IRValue; Self::SIZE],
);

impl PlayEntityMemory {
    pub const SIZE: usize = 64;
}

impl Default for PlayEntityMemory {
    fn default() -> Self {
        Self([0.0; 64])
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PlayEntityMemoryArray {
    pub items: BTreeMap<EntityId, PlayEntityMemory>,
}

impl PlayEntityMemoryArray {
    pub const BLOCK_ID: u64 = 4000;
}

impl PlayEntityMemoryArray {
    pub fn new<'a>(entities: impl Iterator<Item = &'a EntityId>) -> Self {
        Self {
            items: entities
                .map(|id| (*id, PlayEntityMemory::default()))
                .collect(),
        }
    }
}

impl ReadableBlock for PlayEntityMemoryArray {
    fn read(&self, index: usize) -> Option<IRValue> {
        let item_index = index / PlayEntityMemory::SIZE;
        let index_in_item = index % PlayEntityMemory::SIZE;
        match self.items.get(&EntityId(item_index)) {
            Some(item) => item.read(index_in_item),
            None => {
                warn!("Attempted to read PlayEntityMemory of non-existent index {item_index}");
                None
            }
        }
    }
}

impl WritableBlock for PlayEntityMemoryArray {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        let item_index = index / PlayEntityMemory::SIZE;
        let index_in_item = index % PlayEntityMemory::SIZE;
        match self.items.get_mut(&EntityId(item_index)) {
            Some(item) => item.write(index_in_item, value),
            None => {
                warn!("Attempted to write PlayEntityMemory on non-existent index {item_index}");
                false
            }
        }
    }
}

impl ReadableBlock for PlayEntityMemory {
    fn read(&self, index: usize) -> Option<IRValue> {
        match self.0.get(index) {
            Some(value) => Some(*value),
            None => {
                warn!("Attempted to read from out of bounds index {index} on PlayEntityMemory");
                None
            }
        }
    }
}

impl WritableBlock for PlayEntityMemory {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match self.0.get_mut(index) {
            Some(mut_value) => {
                *mut_value = value;
                true
            }
            None => {
                warn!("Attempted to write to out of bounds index {index} on PlayEntityMemory");
                false
            }
        }
    }
}
