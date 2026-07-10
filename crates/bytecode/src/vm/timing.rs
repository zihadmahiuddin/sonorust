use sonorust_runtime::context::RuntimeContext;

use crate::vm::VM;

impl VM {
    #[inline(always)]
    pub(crate) fn execute_beat_to_time(&mut self, runtime_ctx: &RuntimeContext) {
        let beat = self.pop_value("beat");
        self.stack.push(runtime_ctx.timing.beat_to_time(beat));
        self.pc += 1;
    }
}
