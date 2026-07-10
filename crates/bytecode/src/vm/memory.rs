use sonorust_runtime::context::RuntimeContext;

use crate::vm::VM;

impl VM {
    #[inline(always)]
    pub(crate) fn execute_read_mem(&mut self, runtime_ctx: &RuntimeContext) {
        // TODO: maybe instead of rounding it's better to do strict checks... that's for future me to worry about
        let index = self.pop_value("index").round() as usize;
        let block_id = self.pop_value("block_id").round() as u64;
        let value = runtime_ctx
            .memory
            .read(&runtime_ctx, block_id, index)
            .unwrap_or_default();
        self.stack.push(value);
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_write_mem(&mut self, runtime_ctx: &RuntimeContext) {
        let value = self.pop_value("value").round();
        let index = self.pop_value("index").round() as usize;
        let block_id = self.pop_value("block_id").round() as u64;
        let value = runtime_ctx
            .memory
            .write(&runtime_ctx, block_id, index, value)
            .unwrap_or_default();
        self.stack.push(value);
        self.pc += 1;
    }
}
