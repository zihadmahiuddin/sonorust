use sonorust_ir::nodes::*;

use crate::{compiler::compilable::Compilable, instruction::Instruction};

impl Compilable for DebugLog {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::DebugLog);
    }
}

impl Compilable for DebugPause {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.add_inst(Instruction::Pause);
    }
}
