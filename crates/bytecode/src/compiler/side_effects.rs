use sonorust_ir::{
    IRValue,
    nodes::{Draw, Spawn},
};

use crate::{compiler::compilable::Compilable, instruction::Instruction};

impl Compilable for Draw {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.sprite_id);
        compiler.compile_node(self.x1);
        compiler.compile_node(self.y1);
        compiler.compile_node(self.x2);
        compiler.compile_node(self.y2);
        compiler.compile_node(self.x3);
        compiler.compile_node(self.y3);
        compiler.compile_node(self.x4);
        compiler.compile_node(self.y4);
        compiler.compile_node(self.z1);
        compiler.compile_node(self.alpha);
        if let Some(z2) = self.z2 {
            compiler.compile_node(z2);
        } else {
            compiler.add_inst(Instruction::Push(0.0));
        }
        if let Some(z3) = self.z3 {
            compiler.compile_node(z3);
        } else {
            compiler.add_inst(Instruction::Push(0.0));
        }
        if let Some(z4) = self.z4 {
            compiler.compile_node(z4);
        } else {
            compiler.add_inst(Instruction::Push(0.0));
        }
        compiler.add_inst(Instruction::Draw);
    }
}

impl Compilable for Spawn {
    fn compile(&self, compiler: &mut super::Compiler) {
        for data_index in &self.data {
            compiler.compile_node(*data_index);
        }
        compiler.add_inst(Instruction::Push(self.data.len() as IRValue));
        compiler.compile_node(self.archetype_id);
        compiler.add_inst(Instruction::Spawn);
    }
}
