use sonorust_models::ids::ArchetypeId;
use sonorust_runtime::{
    context::RuntimeContext,
    side_effects::{DrawSideEffect, SpawnSideEffect},
};
use tracing::{info, warn};

use crate::vm::VM;

impl VM {
    #[inline(always)]
    pub(crate) fn execute_draw(&mut self, runtime_ctx: &RuntimeContext) {
        let z4 = if self.stack.len() >= 14 {
            Some(self.pop_value("z4"))
        } else {
            None
        };
        let z3 = if self.stack.len() >= 13 {
            Some(self.pop_value("z3"))
        } else {
            None
        };
        let z2 = if self.stack.len() >= 12 {
            Some(self.pop_value("z2"))
        } else {
            None
        };
        let alpha = self.pop_value("alpha");
        let z1 = self.pop_value("z1");
        let y4 = self.pop_value("y4");
        let x4 = self.pop_value("x4");
        let y3 = self.pop_value("y3");
        let x3 = self.pop_value("x3");
        let y2 = self.pop_value("y2");
        let x2 = self.pop_value("x2");
        let y1 = self.pop_value("y1");
        let x1 = self.pop_value("x1");
        let sprite_id = self.pop_value("sprite_id") as usize;
        let draw_side_effect = DrawSideEffect {
            sprite_id,
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            x4,
            y4,
            alpha,
            z1,
            z2,
            z3,
            z4,
        };
        warn!("TODO: Draw: {draw_side_effect:?}");
        runtime_ctx.side_effects.draw(draw_side_effect);
        self.stack.push(0.0);
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_spawn(&mut self, runtime_ctx: &RuntimeContext) {
        let archetype_id = ArchetypeId(self.pop_value("archetype_id") as usize);
        let data: Vec<_> = (0..self.pop_value("data_len") as usize)
            .map(|i| self.pop_value(&format!("data_{i}")))
            .collect();
        info!("Spawn {}, data: {data:?}", *archetype_id);
        runtime_ctx
            .side_effects
            .spawn(SpawnSideEffect { archetype_id, data });
        self.stack.push(0.0);
        self.pc += 1;
    }
}
