use std::collections::HashMap;

use rand::Rng;
use sonorust_ir::IRValue;
use sonorust_models::ids::{ArchetypeId, EntityId};
use tracing::warn;

use crate::access::{MemoryAccess, SideEffectAccess, TimingAccess};

#[derive(Debug, Clone, Copy)]
pub struct CurrentEntity {
    pub id: EntityId,
    pub archetype_id: ArchetypeId,
}

pub struct RuntimeContext<'a> {
    pub current_entity: CurrentEntity,
    pub memory: &'a dyn MemoryAccess,
    pub timing: &'a dyn TimingAccess,
    pub side_effects: &'a dyn SideEffectAccess,
}

impl<'a> RuntimeContext<'a> {
    pub fn copy_memory(
        &mut self,
        src_block_id: u64,
        src_index: usize,
        dst_block_id: u64,
        dst_index: usize,
        count: usize,
    ) -> IRValue {
        let mut temp = Vec::with_capacity(count);
        for i in 0..count {
            match self.memory.read(self, src_block_id, src_index + i) {
                Some(v) => temp.push(v),
                None => {
                    warn!(
                        "Failed to read from block {src_block_id} index {}",
                        src_index + i
                    );
                    break;
                }
            }
        }

        for (i, value) in temp.into_iter().enumerate() {
            if self
                .memory
                .write(self, dst_block_id, dst_index + i, value)
                .is_none()
            {
                warn!(
                    "Failed to write to block {dst_block_id} index {}",
                    dst_index + i
                );
                break;
            }
        }

        0.0
    }
}

pub type ExternalFunction = *const u8;
pub type ExternalFunctionsMap<'a> = HashMap<&'a str, *const u8>;

pub fn get_external_functions<'a>() -> ExternalFunctionsMap<'a> {
    let mut externals_addrs = ExternalFunctionsMap::new();
    externals_addrs.insert("read_mem", read_mem as ExternalFunction);
    externals_addrs.insert("write_mem", write_mem as ExternalFunction);
    externals_addrs.insert("copy_mem", copy_mem as ExternalFunction);
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
extern "C" fn read_mem(ctx: *mut RuntimeContext, block_id: i64, index: i64) -> IRValue {
    let ctx = unsafe { &mut *ctx };
    if let Some(value) = ctx.memory.read(ctx, block_id as u64, index as usize) {
        value
    } else {
        warn!("Failed to read from block {block_id} index {index}");
        0.0
    }
}

#[unsafe(no_mangle)]
extern "C" fn write_mem(
    ctx: *mut RuntimeContext,
    block_id: i64,
    index: i64,
    value: IRValue,
) -> IRValue {
    let ctx = unsafe { &mut *ctx };
    if let Some(value) = ctx
        .memory
        .write(ctx, block_id as u64, index as usize, value)
    {
        value
    } else {
        warn!("Failed to write to block {block_id} index {index}");
        0.0
    }
}

#[unsafe(no_mangle)]
extern "C" fn copy_mem(
    ctx: *mut RuntimeContext,
    src_block_id: i64,
    src_index: i64,
    dst_block_id: i64,
    dst_index: i64,
    count: i64,
) -> IRValue {
    if count < 0 {
        warn!("Copy with negative count: {count}");
        return 0.0;
    }

    let src_block_id = src_block_id as u64;
    let dst_block_id = dst_block_id as u64;
    let src_index = src_index as usize;
    let dst_index = dst_index as usize;
    let count = count as usize;

    let ctx = unsafe { &mut *ctx };

    ctx.copy_memory(src_block_id, src_index, dst_block_id, dst_index, count)
}

#[unsafe(no_mangle)]
extern "C" fn pow(_ctx: *mut RuntimeContext, a: IRValue, b: IRValue) -> IRValue {
    a.powf(b)
}

#[unsafe(no_mangle)]
extern "C" fn sin(_ctx: *mut RuntimeContext, value: IRValue) -> IRValue {
    value.sin()
}

#[unsafe(no_mangle)]
extern "C" fn cos(_ctx: *mut RuntimeContext, value: IRValue) -> IRValue {
    value.cos()
}

#[unsafe(no_mangle)]
extern "C" fn tan(_ctx: *mut RuntimeContext, value: IRValue) -> IRValue {
    value.tan()
}

#[unsafe(no_mangle)]
extern "C" fn sinh(_ctx: *mut RuntimeContext, value: IRValue) -> IRValue {
    value.sinh()
}

#[unsafe(no_mangle)]
extern "C" fn cosh(_ctx: *mut RuntimeContext, value: IRValue) -> IRValue {
    value.cosh()
}

#[unsafe(no_mangle)]
extern "C" fn tanh(_ctx: *mut RuntimeContext, value: IRValue) -> IRValue {
    value.tanh()
}

#[unsafe(no_mangle)]
extern "C" fn arcsin(_ctx: *mut RuntimeContext, value: IRValue) -> IRValue {
    value.asin()
}

#[unsafe(no_mangle)]
extern "C" fn arccos(_ctx: *mut RuntimeContext, value: IRValue) -> IRValue {
    value.acos()
}

#[unsafe(no_mangle)]
extern "C" fn arctan(_ctx: *mut RuntimeContext, value: IRValue) -> IRValue {
    value.atan()
}

#[unsafe(no_mangle)]
extern "C" fn arctan2(_ctx: *mut RuntimeContext, a: IRValue, b: IRValue) -> IRValue {
    b.atan2(a)
}

#[unsafe(no_mangle)]
extern "C" fn degree(_ctx: *mut RuntimeContext, value: IRValue) -> IRValue {
    value.to_degrees()
}

#[unsafe(no_mangle)]
extern "C" fn radian(_ctx: *mut RuntimeContext, value: IRValue) -> IRValue {
    value.to_radians()
}

#[unsafe(no_mangle)]
extern "C" fn log(_ctx: *mut RuntimeContext, value: IRValue) -> IRValue {
    value.ln()
}

#[unsafe(no_mangle)]
extern "C" fn random(_ctx: *mut RuntimeContext, min: IRValue, max: IRValue) -> IRValue {
    rand::rng().random_range(min..=max)
}

#[unsafe(no_mangle)]
extern "C" fn random_integer(_ctx: *mut RuntimeContext, min: IRValue, max: IRValue) -> IRValue {
    let min = min.round() as i32;
    let max = max.round() as i32;
    rand::rng().random_range(min..max) as IRValue
}
