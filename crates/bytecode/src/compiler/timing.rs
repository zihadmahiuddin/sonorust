use sonorust_ir::nodes::BeatToTime;

use crate::{compiler::compilable::Compilable, instruction::Instruction};

impl Compilable for BeatToTime {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.beat);
        compiler.add_inst(Instruction::BeatToTime);
    }
}
