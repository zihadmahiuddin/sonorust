use std::collections::HashMap;

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
