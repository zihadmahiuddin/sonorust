use tracing::{info, warn};

use crate::vm::VM;

impl VM {
    #[inline(always)]
    pub(crate) fn execute_debug_log(&mut self) {
        let val = self.pop_value("value");
        info!("DebugLog: {val}");
        self.stack.push(0.0);
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_debug_pause(&mut self) {
        warn!("DebugPause");
        self.stack.push(0.0);
        self.pc += 1;
        self.pause();
    }
}
