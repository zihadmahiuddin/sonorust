use sonorust_ir::nodes::*;

use crate::{
    compiler::{Compiler, compilable::Compilable},
    instruction::Instruction,
};

impl Compilable for Add {
    fn compile(&self, compiler: &mut Compiler) {
        if self.args.is_empty() {
            compiler.add_inst(Instruction::Push(0.0));
            return;
        }

        compiler.compile_node(self.args[0]);

        for arg_index in self.args.iter().skip(1) {
            compiler.compile_node(*arg_index);
            compiler.add_inst(Instruction::Add);
        }
    }
}

impl Compilable for Divide {
    fn compile(&self, compiler: &mut Compiler) {
        if self.args.is_empty() {
            compiler.add_inst(Instruction::Push(0.0));
            return;
        }

        compiler.compile_node(self.args[0]);

        for arg_index in self.args.iter().skip(1) {
            compiler.compile_node(*arg_index);
            compiler.add_inst(Instruction::Divide);
        }
    }
}

impl Compilable for Multiply {
    fn compile(&self, compiler: &mut Compiler) {
        if self.args.is_empty() {
            compiler.add_inst(Instruction::Push(0.0));
            return;
        }

        compiler.compile_node(self.args[0]);

        for arg_index in self.args.iter().skip(1) {
            compiler.compile_node(*arg_index);
            compiler.add_inst(Instruction::Multiply);
        }
    }
}

impl Compilable for Subtract {
    fn compile(&self, compiler: &mut Compiler) {
        if self.args.is_empty() {
            compiler.add_inst(Instruction::Push(0.0));
            return;
        }

        compiler.compile_node(self.args[0]);

        for arg_index in self.args.iter().skip(1) {
            compiler.compile_node(*arg_index);
            compiler.add_inst(Instruction::Subtract);
        }
    }
}

impl Compilable for Abs {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Abs);
    }
}

impl Compilable for Frac {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Frac);
    }
}

impl Compilable for Trunc {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Trunc);
    }
}

impl Compilable for Negate {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Negate);
    }
}

impl Compilable for Mod {
    fn compile(&self, compiler: &mut Compiler) {
        if self.args.is_empty() {
            compiler.add_inst(Instruction::Push(0.0));
            return;
        }

        compiler.compile_node(self.args[0]);

        for arg_index in self.args.iter().skip(1) {
            compiler.compile_node(*arg_index);
            compiler.add_inst(Instruction::Mod);
        }
    }
}

impl Compilable for Rem {
    fn compile(&self, compiler: &mut Compiler) {
        if self.args.is_empty() {
            compiler.add_inst(Instruction::Push(0.0));
            return;
        }

        compiler.compile_node(self.args[0]);

        for arg_index in self.args.iter().skip(1) {
            compiler.compile_node(*arg_index);
            compiler.add_inst(Instruction::Rem);
        }
    }
}

impl Compilable for Power {
    fn compile(&self, compiler: &mut Compiler) {
        if self.args.is_empty() {
            compiler.add_inst(Instruction::Push(0.0));
            return;
        }

        compiler.compile_node(self.args[0]);

        for arg_index in self.args.iter().skip(1) {
            compiler.compile_node(*arg_index);
            compiler.add_inst(Instruction::Power);
        }
    }
}

impl Compilable for Clamp {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.min);
        compiler.compile_node(self.max);
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Clamp);
    }
}

impl Compilable for Lerp {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.min);
        compiler.compile_node(self.max);
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Lerp);
    }
}

impl Compilable for LerpClamped {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.min);
        compiler.compile_node(self.max);
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::LerpClamped);
    }
}

impl Compilable for Unlerp {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.min);
        compiler.compile_node(self.max);
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Unlerp);
    }
}

impl Compilable for UnlerpClamped {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.min);
        compiler.compile_node(self.max);
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::UnlerpClamped);
    }
}

impl Compilable for Min {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.x);
        compiler.compile_node(self.y);
        compiler.add_inst(Instruction::Min);
    }
}

impl Compilable for Max {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.x);
        compiler.compile_node(self.y);
        compiler.add_inst(Instruction::Max);
    }
}

impl Compilable for Remap {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.from_min);
        compiler.compile_node(self.from_max);
        compiler.compile_node(self.to_min);
        compiler.compile_node(self.to_max);
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Remap);
    }
}

impl Compilable for RemapClamped {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.from_min);
        compiler.compile_node(self.from_max);
        compiler.compile_node(self.to_min);
        compiler.compile_node(self.to_max);
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::RemapClamped);
    }
}

impl Compilable for Round {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Round);
    }
}

impl Compilable for Floor {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Floor);
    }
}

impl Compilable for Ceil {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Ceil);
    }
}

impl Compilable for Sin {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Sin);
    }
}

impl Compilable for Sinh {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Sinh);
    }
}

impl Compilable for Cos {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Cos);
    }
}

impl Compilable for Cosh {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Cosh);
    }
}

impl Compilable for Tan {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Tan);
    }
}

impl Compilable for Tanh {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Tanh);
    }
}

impl Compilable for Arcsin {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Arcsin);
    }
}

impl Compilable for Arccos {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Arccos);
    }
}

impl Compilable for Arctan {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Arctan);
    }
}

impl Compilable for Arctan2 {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_node(self.y);
        compiler.compile_node(self.x);
        compiler.add_inst(Instruction::Arctan2);
    }
}

impl Compilable for Degree {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Degree);
    }
}

impl Compilable for Radian {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Radian);
    }
}

impl Compilable for Log {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Log);
    }
}

impl Compilable for Sign {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_node(self.value);
        compiler.add_inst(Instruction::Sign);
    }
}

impl Compilable for Random {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_node(self.min);
        compiler.compile_node(self.max);
        compiler.add_inst(Instruction::Random);
    }
}

impl Compilable for RandomInteger {
    fn compile(&self, compiler: &mut Compiler) {
        compiler.compile_node(self.min);
        compiler.compile_node(self.max);
        compiler.add_inst(Instruction::RandomInteger);
    }
}
