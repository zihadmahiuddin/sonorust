use glam::Vec2;
use sonorust_ir::IRValue;
use tracing::warn;

use crate::blocks::{ReadableBlock, WritableBlock};

#[derive(Debug)]
pub struct PlayRuntimeBackground {
    pub bottom_left: Vec2,
    pub top_left: Vec2,
    pub top_right: Vec2,
    pub bottom_right: Vec2,
}

impl PlayRuntimeBackground {
    pub const BLOCK_ID: u64 = 1005;

    pub const fn as_array(&self) -> [IRValue; 8] {
        [
            self.bottom_left.x,
            self.bottom_left.y,
            self.top_left.x,
            self.top_left.y,
            self.top_right.x,
            self.top_right.y,
            self.bottom_right.x,
            self.bottom_right.y,
        ]
    }

    pub fn as_mut_array(&mut self) -> [&mut IRValue; 8] {
        [
            &mut self.bottom_left.x,
            &mut self.bottom_left.y,
            &mut self.top_left.x,
            &mut self.top_left.y,
            &mut self.top_right.x,
            &mut self.top_right.y,
            &mut self.bottom_right.x,
            &mut self.bottom_right.y,
        ]
    }
}

impl ReadableBlock for PlayRuntimeBackground {
    fn read(&self, index: usize) -> Option<IRValue> {
        match self.as_array().get(index) {
            Some(&value) => Some(value),
            None => {
                warn!(
                    "Attempted to read from out of bounds index {index} to PlayRuntimeBackground"
                );
                None
            }
        }
    }
}

impl WritableBlock for PlayRuntimeBackground {
    fn write(&mut self, index: usize, value: IRValue) -> bool {
        match self.as_mut_array().get_mut(index) {
            Some(mut_value) => {
                **mut_value = value;
                true
            }
            None => {
                warn!("Attempted to write to out of bounds index {index} to PlayRuntimeBackground");
                false
            }
        }
    }
}
