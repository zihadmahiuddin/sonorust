use glam::{Vec2, Vec4};
use sonorust_ir::IRValue;
use tracing::warn;

use crate::blocks::ReadableBlock;

#[derive(Debug)]
pub struct PlayRuntimeTouchArray(pub Vec<PlayRuntimeTouch>);

impl PlayRuntimeTouchArray {
    pub const BLOCK_ID: u64 = 1002;
}

#[derive(Debug)]
pub struct PlayRuntimeTouch {
    pub id: IRValue,
    pub started: bool,
    pub ended: bool,
    /// Touch t: time of touch event reported by operating system, with input offset accounted for
    pub time: IRValue,
    /// Touch st: start time of touch reported by operating system, with input offset accounted for
    pub start_time: IRValue,
    /// Touch x,y: x,y of touch's current position
    pub xy: Vec2,
    /// Touch sx,sy: x,y of touch's starting position
    pub start_xy: Vec2,
    /// Touch dx,dy: delta x,y of touch's position since last update cycle
    pub delta_xy: Vec2,
    /// Touch vx,vy,vr,vw: x,y,r,w of touch's velocity
    pub velocity_xyrw: Vec4,
}

impl PlayRuntimeTouch {
    pub const SIZE: usize = 15;

    pub const INDEX_ID: usize = 0;
    pub const INDEX_STARTED: usize = 1;
    pub const INDEX_ENDED: usize = 2;
    pub const INDEX_TIME: usize = 3;
    pub const INDEX_START_TIME: usize = 4;
    pub const INDEX_X: usize = 5;
    pub const INDEX_Y: usize = 6;
    pub const INDEX_START_X: usize = 7;
    pub const INDEX_START_Y: usize = 8;
    pub const INDEX_DELTA_X: usize = 9;
    pub const INDEX_DELTA_Y: usize = 10;
    pub const INDEX_VELOCITY_X: usize = 11;
    pub const INDEX_VELOCITY_Y: usize = 12;
    pub const INDEX_VELOCITY_R: usize = 13;
    pub const INDEX_VELOCITY_W: usize = 14;

    fn read(&self, index: usize) -> Option<IRValue> {
        match index {
            Self::INDEX_ID => Some(self.id),
            Self::INDEX_STARTED => Some(IRValue::from(self.started)),
            Self::INDEX_ENDED => Some(IRValue::from(self.ended)),
            Self::INDEX_TIME => Some(self.time),
            Self::INDEX_START_TIME => Some(self.start_time),
            Self::INDEX_X => Some(self.xy.x),
            Self::INDEX_Y => Some(self.xy.y),
            Self::INDEX_START_X => Some(self.start_xy.x),
            Self::INDEX_START_Y => Some(self.start_xy.y),
            Self::INDEX_DELTA_X => Some(self.delta_xy.x),
            Self::INDEX_DELTA_Y => Some(self.delta_xy.y),
            Self::INDEX_VELOCITY_X => Some(self.velocity_xyrw.x),
            Self::INDEX_VELOCITY_Y => Some(self.velocity_xyrw.y),
            Self::INDEX_VELOCITY_R => Some(self.velocity_xyrw.z),
            Self::INDEX_VELOCITY_W => Some(self.velocity_xyrw.w),
            other => {
                warn!("Attempted to read from out of bounds index {other} on PlayRuntimeTouch");
                None
            }
        }
    }
}

impl ReadableBlock for PlayRuntimeTouchArray {
    fn read(&self, index: usize) -> Option<IRValue> {
        let touch_index = index / PlayRuntimeTouch::SIZE;
        let index_in_touch = index % PlayRuntimeTouch::SIZE;
        match self.0.get(touch_index) {
            Some(entity) => entity.read(index_in_touch),
            None => {
                warn!("Attempted to read PlayRuntimeTouch of non-existent touch {touch_index}");
                None
            }
        }
    }
}
