use std::collections::HashMap;

use rand::Rng;
use tracing::warn;

pub trait MemoryAccess {
    fn read(&self, block_id: u64, index: usize) -> Option<f32>;
    fn write(&self, block_id: u64, index: usize, value: f32) -> Option<f32>;
}

pub struct RuntimeContext<'a> {
    pub memory: &'a dyn MemoryAccess,
}

pub type ExternalFunction = *const u8;
pub type ExternalFunctionsMap<'a> = HashMap<&'a str, *const u8>;

pub fn get_external_functions<'a>() -> ExternalFunctionsMap<'a> {
    let mut externals_addrs = ExternalFunctionsMap::new();
    externals_addrs.insert("read_mem", read_mem as ExternalFunction);
    externals_addrs.insert("write_mem", write_mem as ExternalFunction);
    externals_addrs.insert("pow", pow as ExternalFunction);
    externals_addrs.insert("sin", sin as ExternalFunction);
    externals_addrs.insert("cos", cos as ExternalFunction);
    externals_addrs.insert("tan", tan as ExternalFunction);
    externals_addrs.insert("sinh", sinh as ExternalFunction);
    externals_addrs.insert("cosh", cosh as ExternalFunction);
    externals_addrs.insert("tanh", tanh as ExternalFunction);
    externals_addrs.insert("arcsin", arcsin as ExternalFunction);
    externals_addrs.insert("arccos", arccos as ExternalFunction);
    externals_addrs.insert("arctan", arctan as ExternalFunction);
    externals_addrs.insert("arctan2", arctan2 as ExternalFunction);
    externals_addrs.insert("degree", degree as ExternalFunction);
    externals_addrs.insert("radian", radian as ExternalFunction);
    externals_addrs.insert("log", log as ExternalFunction);
    externals_addrs.insert("random", random as ExternalFunction);
    externals_addrs.insert("random_integer", random_integer as ExternalFunction);
    externals_addrs
}

#[unsafe(no_mangle)]
extern "C" fn read_mem(ctx: *mut RuntimeContext, block_id: i64, index: i64) -> f32 {
    let ctx = unsafe { &mut *ctx };
    if let Some(value) = ctx.memory.read(block_id as u64, index as usize) {
        value
    } else {
        warn!("Failed to read from block {block_id} index {index}");
        0.0
    }
}

#[unsafe(no_mangle)]
extern "C" fn write_mem(ctx: *mut RuntimeContext, block_id: i64, index: i64, value: f32) -> f32 {
    let ctx = unsafe { &mut *ctx };
    if let Some(value) = ctx.memory.write(block_id as u64, index as usize, value) {
        value
    } else {
        warn!("Failed to write to block {block_id} index {index}");
        0.0
    }
}

#[unsafe(no_mangle)]
extern "C" fn pow(_ctx: *mut RuntimeContext, a: f32, b: f32) -> f32 {
    a.powf(b)
}

#[unsafe(no_mangle)]
extern "C" fn sin(_ctx: *mut RuntimeContext, value: f32) -> f32 {
    value.sin()
}

#[unsafe(no_mangle)]
extern "C" fn cos(_ctx: *mut RuntimeContext, value: f32) -> f32 {
    value.cos()
}

#[unsafe(no_mangle)]
extern "C" fn tan(_ctx: *mut RuntimeContext, value: f32) -> f32 {
    value.tan()
}

#[unsafe(no_mangle)]
extern "C" fn sinh(_ctx: *mut RuntimeContext, value: f32) -> f32 {
    value.sinh()
}

#[unsafe(no_mangle)]
extern "C" fn cosh(_ctx: *mut RuntimeContext, value: f32) -> f32 {
    value.cosh()
}

#[unsafe(no_mangle)]
extern "C" fn tanh(_ctx: *mut RuntimeContext, value: f32) -> f32 {
    value.tanh()
}

#[unsafe(no_mangle)]
extern "C" fn arcsin(_ctx: *mut RuntimeContext, value: f32) -> f32 {
    value.asin()
}

#[unsafe(no_mangle)]
extern "C" fn arccos(_ctx: *mut RuntimeContext, value: f32) -> f32 {
    value.acos()
}

#[unsafe(no_mangle)]
extern "C" fn arctan(_ctx: *mut RuntimeContext, value: f32) -> f32 {
    value.atan()
}

#[unsafe(no_mangle)]
extern "C" fn arctan2(_ctx: *mut RuntimeContext, a: f32, b: f32) -> f32 {
    b.atan2(a)
}

#[unsafe(no_mangle)]
extern "C" fn degree(_ctx: *mut RuntimeContext, value: f32) -> f32 {
    value.to_degrees()
}

#[unsafe(no_mangle)]
extern "C" fn radian(_ctx: *mut RuntimeContext, value: f32) -> f32 {
    value.to_radians()
}

#[unsafe(no_mangle)]
extern "C" fn log(_ctx: *mut RuntimeContext, value: f32) -> f32 {
    value.ln()
}

#[unsafe(no_mangle)]
extern "C" fn random(_ctx: *mut RuntimeContext, min: f32, max: f32) -> f32 {
    rand::rng().random_range(min..=max)
}

#[unsafe(no_mangle)]
extern "C" fn random_integer(_ctx: *mut RuntimeContext, min: f32, max: f32) -> f32 {
    let min = min.round() as i32;
    let max = max.round() as i32;
    rand::rng().random_range(min..max) as f32
}
