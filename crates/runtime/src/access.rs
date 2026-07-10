use sonorust_ir::IRValue;

use crate::{
    context::RuntimeContext,
    side_effects::{DrawSideEffect, SpawnSideEffect},
};

pub trait MemoryAccess {
    fn read(&self, ctx: &RuntimeContext, block_id: u64, index: usize) -> Option<IRValue>;
    fn write(
        &self,
        ctx: &RuntimeContext,
        block_id: u64,
        index: usize,
        value: IRValue,
    ) -> Option<IRValue>;
}

pub trait TimingAccess {
    fn beat_to_time(&self, beat: IRValue) -> IRValue;
    fn beat_to_bpm(&self, beat: IRValue) -> IRValue;
    fn time_to_scaled_time(&self, time: IRValue) -> IRValue;
}

pub trait SideEffectAccess {
    fn spawn(&self, spawn_side_effect: SpawnSideEffect);
    fn draw(&self, draw_side_effect: DrawSideEffect);
}
