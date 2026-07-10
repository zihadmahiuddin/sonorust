use tracing::error;

use crate::{instruction::Instruction, vm::VM};

impl VM {
    #[inline(always)]
    pub(crate) fn execute_jump(&mut self, target: usize) {
        self.pc = target;
    }

    #[inline(always)]
    pub(crate) fn execute_jump_zero(&mut self, target: usize) {
        let cond = self.pop_value("condition");
        if cond == 0.0 {
            self.pc = target;
        } else {
            self.pc += 1;
        }
    }

    #[inline(always)]
    pub(crate) fn execute_jump_table(&mut self) {
        let index_val = self.pop_value("index");

        let inst = &self.instructions[self.pc];
        let Instruction::JumpTable {
            targets,
            default_target,
        } = inst
        else {
            error!(
                "Unexpected instruction {:?} found, expected JumpTable",
                inst
            );
            return;
        };

        let index = index_val as usize;

        if index < targets.len() {
            self.pc = targets[index];
        } else {
            self.pc = *default_target;
        }
    }
}
