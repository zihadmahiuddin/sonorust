use sonorust_ir::{IRIndex, nodes::*};

use crate::{
    compiler::{Compiler, compilable::Compilable},
    instruction::Instruction::{self, *},
};

impl Compilable for Get {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.block_id);
        compiler.compile_node(self.index);
        compiler.add_inst(ReadMem);
    }
}

impl Compilable for GetShifted {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.block_id);
        compiler.compile_node(self.x);
        compiler.compile_node(self.y);
        compiler.compile_node(self.s);

        compiler.add_inst(Multiply);
        compiler.add_inst(Add);

        compiler.add_inst(ReadMem);
    }
}

impl Compilable for Set {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.block_id);
        compiler.compile_node(self.index);
        compiler.compile_node(self.value);
        compiler.add_inst(WriteMem);
    }
}

impl Compilable for SetAdd {
    fn compile(&self, compiler: &mut super::Compiler) {
        compile_compound_assign(compiler, self.block_id, self.index, self.value, Add);
    }
}

impl Compilable for SetMultiply {
    fn compile(&self, compiler: &mut super::Compiler) {
        compile_compound_assign(compiler, self.block_id, self.index, self.value, Multiply);
    }
}

fn compile_compound_assign(
    compiler: &mut Compiler,
    block_id: IRIndex,
    index: IRIndex,
    value: IRIndex,
    op: Instruction,
) {
    compiler.compile_node(block_id);
    compiler.compile_node(index);
    compiler.add_inst(Dup {
        offset: 0,
        count: 2,
    });
    compiler.add_inst(ReadMem);
    compiler.compile_node(value);
    compiler.add_inst(op);
    compiler.add_inst(WriteMem);
}
