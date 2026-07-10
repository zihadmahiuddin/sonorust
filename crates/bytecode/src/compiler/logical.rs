use sonorust_ir::nodes::*;

use crate::{compiler::compilable::Compilable, instruction::Instruction};

impl Compilable for Less {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.lhs);
        compiler.compile_node(self.rhs);
        compiler.add_inst(Instruction::Less);
    }
}

impl Compilable for Greater {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.lhs);
        compiler.compile_node(self.rhs);
        compiler.add_inst(Instruction::Greater);
    }
}

impl Compilable for LessOr {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.lhs);
        compiler.compile_node(self.rhs);
        compiler.add_inst(Instruction::LessOr);
    }
}

impl Compilable for GreaterOr {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.lhs);
        compiler.compile_node(self.rhs);
        compiler.add_inst(Instruction::GreaterOr);
    }
}

impl Compilable for Equal {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.lhs);
        compiler.compile_node(self.rhs);
        compiler.add_inst(Instruction::Equal);
    }
}

impl Compilable for NotEqual {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.lhs);
        compiler.compile_node(self.rhs);
        compiler.add_inst(Instruction::Equal);
        compiler.add_inst(Instruction::Not);
    }
}

impl Compilable for Not {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Not);
    }
}

impl Compilable for And {
    fn compile(&self, compiler: &mut super::Compiler) {
        if self.args.is_empty() {
            compiler.add_inst(Instruction::Push(0.0));
            return;
        }

        compiler.compile_node(self.args[0]);

        for arg_index in self.args.iter().skip(1) {
            compiler.compile_node(*arg_index);
            compiler.add_inst(Instruction::And);
        }
    }
}

impl Compilable for Or {
    fn compile(&self, compiler: &mut super::Compiler) {
        if self.args.is_empty() {
            compiler.add_inst(Instruction::Push(0.0));
            return;
        }

        compiler.compile_node(self.args[0]);

        for arg_index in self.args.iter().skip(1) {
            compiler.compile_node(*arg_index);
            compiler.add_inst(Instruction::Or);
        }
    }
}
