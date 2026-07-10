use sonorust_ir::{IRValue, nodes::*};
use tracing::{info, warn};

use crate::{compiler::compilable::Compilable, instruction::Instruction::*};

impl Compilable for Execute {
    fn compile(&self, compiler: &mut super::Compiler) {
        let len = self.args.len();

        for (i, arg_index) in self.args.iter().enumerate() {
            compiler.compile_node(*arg_index);

            // If this is NOT the last statement in the block,
            // we must throw away its result so it doesn't pollute the stack.
            if i < len - 1
                && compiler
                    .instructions
                    .last()
                    .is_some_and(|opcode| opcode.produces_result())
            {
                compiler.add_inst(Pop);
            }
        }
    }
}

impl Compilable for Execute0 {
    fn compile(&self, compiler: &mut super::Compiler) {
        for arg_index in self.args.iter() {
            compiler.compile_node(*arg_index);

            // Pop the result of every executed body
            compiler.add_inst(Pop);
        }

        // Then return 0 at the end
        compiler.add_inst(Push(0.0));
    }
}

impl Compilable for If {
    fn compile(&self, compiler: &mut super::Compiler) {
        let cf_id = compiler.control_flow_counter;
        compiler.control_flow_counter += 1;

        compiler.start_new_block(Some(format!("if_condition_{cf_id}")));
        compiler.compile_node(self.test);

        let jump_if_false_address = compiler.instructions.len();
        compiler.add_inst(JumpZero(0));

        compiler.start_new_block(Some(format!("if_true_{cf_id}")));
        compiler.compile_node(self.consequent);

        let jump_to_end_address = compiler.instructions.len();
        compiler.add_inst(Jump(0));

        let else_branch_start = compiler.instructions.len();
        compiler.instructions[jump_if_false_address] = JumpZero(else_branch_start);

        compiler.start_new_block(Some(format!("if_false_{cf_id}")));
        compiler.compile_node(self.alternate);

        compiler.start_new_block(Some(format!("if_end_{cf_id}")));

        let completely_done = compiler.instructions.len();
        compiler.instructions[jump_to_end_address] = Jump(completely_done);
    }
}

impl Compilable for While {
    fn compile(&self, compiler: &mut super::Compiler) {
        let cf_id = compiler.control_flow_counter;
        compiler.control_flow_counter += 1;

        compiler.start_new_block(Some(format!("while_test_{cf_id}")));
        let loop_start_address = compiler.instructions.len();

        compiler.compile_node(self.test);

        let jump_out_address = compiler.instructions.len();
        compiler.add_inst(JumpZero(0));

        compiler.start_new_block(Some(format!("while_body_{cf_id}")));
        compiler.compile_node(self.body);
        compiler.add_inst(Pop);
        compiler.add_inst(Jump(loop_start_address));

        compiler.start_new_block(Some(format!("while_exit_{cf_id}")));
        let loop_end_address = compiler.instructions.len();
        compiler.instructions[jump_out_address] = JumpZero(loop_end_address);
    }
}

impl Compilable for Block {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.break_scopes.push(Vec::new());

        compiler.compile_node(self.body);

        let end_address = compiler.instructions.len();
        if let Some(patches) = compiler.break_scopes.pop() {
            for patch_addr in patches {
                compiler.instructions[patch_addr] = Jump(end_address);
            }
        }
    }
}

impl Compilable for Break {
    fn compile(&self, compiler: &mut super::Compiler) {
        info!("Compiling Break: {self:?}");
        info!("Break scopes: {:?}", compiler.break_scopes);

        compiler.compile_node(self.value);

        let count_node = &compiler.nodes[*self.count];
        let IRNode::Value(count) = count_node else {
            warn!("Dynamic break count encountered! Node: {:?}", count_node);
            return;
        };

        let target_depth = compiler
            .break_scopes
            .len()
            .checked_sub(*count as usize) // TODO: IT SHOULD SUPPORT DYNAMIC
            .expect("Break count depth out of bounds");

        let patch_address = compiler.instructions.len();
        compiler.add_inst(Jump(0));
        compiler.break_scopes[target_depth].push(patch_address);
    }
}

impl Compilable for Switch {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.discriminant);
        compiler.break_scopes.push(Vec::new());

        let mut case_chunks = self.tests_and_consequents.chunks_exact(2);
        let mut next_case_jump_patches = Vec::new();

        while let Some(chunk) = case_chunks.next() {
            let test_node = chunk[0];
            let consequent_node = chunk[1];

            if !next_case_jump_patches.is_empty() {
                let current_idx = compiler.instructions.len();
                for patch in next_case_jump_patches.drain(..) {
                    compiler.instructions[patch] = JumpZero(current_idx);
                }
            }

            compiler.add_inst(Dup {
                count: 1,
                offset: 0,
            });
            compiler.compile_node(test_node);
            compiler.add_inst(Equal);

            let mismatch_jump = compiler.instructions.len();
            compiler.add_inst(JumpZero(0));
            next_case_jump_patches.push(mismatch_jump);

            compiler.add_inst(Pop);
            compiler.compile_node(consequent_node);

            let case_end = compiler.instructions.len();
            compiler.add_inst(Jump(0));
            compiler.break_scopes.last_mut().unwrap().push(case_end);
        }

        let current_idx = compiler.instructions.len();
        for patch in next_case_jump_patches {
            compiler.instructions[patch] = JumpZero(current_idx);
        }
        compiler.add_inst(Pop);

        let end_address = compiler.instructions.len();
        if let Some(patches) = compiler.break_scopes.pop() {
            for patch_addr in patches {
                compiler.instructions[patch_addr] = Jump(end_address);
            }
        }
    }
}

impl Compilable for SwitchWithDefault {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.discriminant);
        compiler.break_scopes.push(Vec::new());

        let mut case_chunks = self.tests_and_consequents.chunks_exact(2);
        let mut next_case_jump_patches = Vec::new();

        while let Some(chunk) = case_chunks.next() {
            let test_node = chunk[0];
            let consequent_node = chunk[1];

            if !next_case_jump_patches.is_empty() {
                let current_idx = compiler.instructions.len();
                for patch in next_case_jump_patches.drain(..) {
                    compiler.instructions[patch] = JumpZero(current_idx);
                }
            }

            compiler.add_inst(Dup {
                count: 1,
                offset: 0,
            });
            compiler.compile_node(test_node);
            compiler.add_inst(Equal);

            let mismatch_jump = compiler.instructions.len();
            compiler.add_inst(JumpZero(0));
            next_case_jump_patches.push(mismatch_jump);

            compiler.add_inst(Pop);
            compiler.compile_node(consequent_node);

            let case_end = compiler.instructions.len();
            compiler.add_inst(Jump(0));
            compiler.break_scopes.last_mut().unwrap().push(case_end);
        }

        let default_idx = compiler.instructions.len();
        for patch in next_case_jump_patches {
            compiler.instructions[patch] = JumpZero(default_idx);
        }
        compiler.add_inst(Pop);
        compiler.compile_node(self.default_consequent);

        let end_address = compiler.instructions.len();
        if let Some(patches) = compiler.break_scopes.pop() {
            for patch_addr in patches {
                compiler.instructions[patch_addr] = Jump(end_address);
            }
        }
    }
}

impl Compilable for SwitchInteger {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.discriminant);

        let table_address = compiler.instructions.len();
        compiler.add_inst(JumpTable {
            targets: vec![],
            default_target: 0,
        });

        compiler.break_scopes.push(Vec::new());
        let mut targets = Vec::new();

        for consequent in &self.consequents {
            targets.push(compiler.instructions.len());
            compiler.compile_node(*consequent);

            let exit_jump = compiler.instructions.len();
            compiler.add_inst(Jump(0));
            compiler.break_scopes.last_mut().unwrap().push(exit_jump);
        }

        let end_address = compiler.instructions.len();

        compiler.instructions[table_address] = JumpTable {
            targets,
            default_target: end_address,
        };

        if let Some(patches) = compiler.break_scopes.pop() {
            for patch_addr in patches {
                compiler.instructions[patch_addr] = Jump(end_address);
            }
        }
    }
}

impl Compilable for SwitchIntegerWithDefault {
    fn compile(&self, compiler: &mut super::Compiler) {
        compiler.compile_node(self.discriminant);

        let table_address = compiler.instructions.len();
        compiler.add_inst(JumpTable {
            targets: vec![],
            default_target: 0,
        });

        compiler.break_scopes.push(Vec::new());
        let mut targets = Vec::new();

        for consequent in &self.consequents {
            targets.push(compiler.instructions.len());
            compiler.compile_node(*consequent);
            let exit_jump = compiler.instructions.len();
            compiler.add_inst(Jump(0));
            compiler.break_scopes.last_mut().unwrap().push(exit_jump);
        }

        let default_target = compiler.instructions.len();
        compiler.compile_node(self.default_consequent);

        let end_address = compiler.instructions.len();
        compiler.instructions[table_address] = JumpTable {
            targets,
            default_target,
        };

        if let Some(patches) = compiler.break_scopes.pop() {
            for patch_addr in patches {
                compiler.instructions[patch_addr] = Jump(end_address);
            }
        }
    }
}

impl Compilable for JumpLoop {
    fn compile(&self, compiler: &mut super::Compiler) {
        let last_index = 1 + self.middle.len();
        compiler.break_scopes.push(Vec::new());

        compiler.add_inst(Push(0.0));

        let loop_start_address = compiler.instructions.len();

        compiler.add_inst(Dup {
            count: 1,
            offset: 0,
        });
        compiler.add_inst(Push(last_index as IRValue));
        compiler.add_inst(Equal);

        let jump_to_switch = compiler.instructions.len();
        compiler.add_inst(JumpZero(0));

        compiler.add_inst(Pop);
        compiler.compile_node(self.last);

        let end_jump = compiler.instructions.len();
        compiler.add_inst(Jump(0));

        let switch_start = compiler.instructions.len();
        compiler.instructions[jump_to_switch] = JumpZero(switch_start);

        let table_address = compiler.instructions.len();
        compiler.add_inst(JumpTable {
            targets: vec![],
            default_target: 0,
        });

        let mut targets = Vec::new();

        targets.push(compiler.instructions.len());
        compiler.compile_node(self.first);
        compiler.add_inst(Jump(loop_start_address));

        for mid in &self.middle {
            targets.push(compiler.instructions.len());
            compiler.compile_node(*mid);
            compiler.add_inst(Jump(loop_start_address));
        }

        let out_of_bounds = compiler.instructions.len();
        compiler.add_inst(Push(0.0));

        let exit_jump = compiler.instructions.len();
        compiler.add_inst(Jump(0));

        let end_address = compiler.instructions.len();
        compiler.instructions[end_jump] = Jump(end_address);
        compiler.instructions[exit_jump] = Jump(end_address);

        compiler.instructions[table_address] = JumpTable {
            targets,
            default_target: out_of_bounds,
        };

        if let Some(patches) = compiler.break_scopes.pop() {
            for patch in patches {
                compiler.instructions[patch] = Jump(end_address);
            }
        }
    }
}
