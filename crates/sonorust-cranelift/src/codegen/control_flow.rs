use cranelift::{codegen::ir::BlockArg, frontend::Switch, prelude::*};

use crate::codegen::{BlockKind, CodegenContext};
use sonorust_ir::nodes::{self, *};

impl<'s, 'b> CodegenContext<'s, 'b> {
    pub(crate) fn build_execute_ir(&mut self, node: &Execute) -> Value {
        let mut last_val = crate::ir_value_cranelift_const(self.builder.ins(), 0.0);
        for &child_index in &node.nodes {
            last_val = self.build_node_ir(child_index);
        }
        last_val
    }

    pub(crate) fn build_execute0_ir(&mut self, node: &Execute0) -> Value {
        self.build_execute_ir(&Execute {
            nodes: node.nodes.clone(),
        });
        crate::ir_value_cranelift_const(self.builder.ins(), 0.0)
    }

    fn with_terminated_reset<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> (T, bool) {
        let prev = self.current_block_terminated;
        self.current_block_terminated = false;
        let result = f(self);
        let broken = self.current_block_terminated;
        self.current_block_terminated = prev;
        (result, broken)
    }

    pub(crate) fn build_if_ir(&mut self, node: &If) -> Value {
        let (test_val, broken) = self.with_terminated_reset(|s| s.build_node_ir(node.test));
        if broken {
            return test_val;
        }

        let zero = crate::ir_value_cranelift_const(self.builder.ins(), 0.0);
        let condition = self.builder.ins().fcmp(FloatCC::NotEqual, test_val, zero);

        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let join_block = self.builder.create_block();

        self.builder.append_block_param(join_block, crate::IR_VALUE_CRANELIFT_TYPE);

        self.builder
            .ins()
            .brif(condition, then_block, &[], else_block, &[]);

        self.builder.switch_to_block(then_block);
        let (then_val, broken) = self.with_terminated_reset(|s| s.build_node_ir(node.consequent));
        if !broken {
            self.builder
                .ins()
                .jump(join_block, &[BlockArg::Value(then_val)]);
        }
        self.builder.seal_block(then_block);

        self.builder.switch_to_block(else_block);
        let (else_val, broken) = self.with_terminated_reset(|s| s.build_node_ir(node.alternate));
        if !broken {
            self.builder
                .ins()
                .jump(join_block, &[BlockArg::Value(else_val)]);
        }
        self.builder.seal_block(else_block);

        self.builder.switch_to_block(join_block);
        self.builder.seal_block(join_block);
        self.builder.block_params(join_block)[0]
    }

    pub(crate) fn build_block_ir(&mut self, node: &nodes::Block) -> Value {
        // While sets the pending_block_stack to communicate its blocks to the Block
        let block_kind = self
            .pending_block_stack
            .take()
            .unwrap_or_else(|| BlockKind::Regular(self.builder.create_block()));
        let exit_block_param = self
            .builder
            .append_block_param(block_kind.exit_block(), crate::IR_VALUE_CRANELIFT_TYPE);

        self.block_stack.push(block_kind.clone());
        let (body_value, broken) = self.with_terminated_reset(|s| s.build_node_ir(node.body));
        self.current_block_terminated = broken;
        self.block_stack.pop();

        if !broken {
            self.current_block_terminated = true;
            if let BlockKind::While { head, .. } = block_kind {
                self.builder.ins().jump(head, &[]);
                self.builder.switch_to_block(block_kind.exit_block());
                self.builder.seal_block(block_kind.exit_block());
                return exit_block_param;
            } else {
                self.builder
                    .ins()
                    .jump(block_kind.exit_block(), &[BlockArg::Value(body_value)]);
                self.builder.switch_to_block(block_kind.exit_block());
                self.builder.seal_block(block_kind.exit_block());
                return exit_block_param;
            }
        }
        self.builder.switch_to_block(block_kind.exit_block());
        self.builder.seal_block(block_kind.exit_block());
        exit_block_param
    }

    pub(crate) fn build_break_ir(&mut self, node: &Break) -> Value {
        let value_to_return = self.build_node_ir(node.value);
        let break_count = match self.nodes[node.count] {
            ResolvedNode::Value(count) => count as usize,
            _ => unimplemented!("Break with dynamic count is not supported."),
        };

        let target_block = self
            .block_stack
            .iter()
            .rev()
            .nth(break_count - 1)
            .cloned()
            .expect("Break count exceeds block stack depth");
        self.builder.ins().jump(
            target_block.exit_block(),
            &[BlockArg::Value(value_to_return)],
        );
        self.current_block_terminated = true;
        value_to_return
    }

    pub(crate) fn build_while_ir(&mut self, node: &While) -> Value {
        let loop_head = self.builder.create_block();
        let loop_body = self.builder.create_block();
        let loop_exit = self.builder.create_block();

        self.builder.ins().jump(loop_head, &[]);
        self.builder.switch_to_block(loop_head);

        let condition_value = self.build_node_ir(node.test);
        let zero = crate::ir_value_cranelift_const(self.builder.ins(), 0.0);
        let one = crate::ir_value_cranelift_const(self.builder.ins(), 1.0);
        let compare_result = self
            .builder
            .ins()
            .fcmp(FloatCC::Equal, condition_value, one);
        self.builder.ins().brif(
            compare_result,
            loop_body,
            &[],
            loop_exit,
            &[BlockArg::Value(zero)],
        );

        // build_block_ir will then properly handle it
        // by pushing it to the stack
        // properly using it as the exit point
        // and also sealing the exit block
        self.pending_block_stack.replace(BlockKind::While {
            head: loop_head,
            body: loop_body,
            exit: loop_exit,
        });
        self.builder.switch_to_block(loop_body);
        let (_, broken) = self.with_terminated_reset(|s| s.build_node_ir(node.body));
        if !broken {
            self.builder.ins().jump(loop_head, &[]);
        }

        self.builder.seal_block(loop_head);
        self.builder.seal_block(loop_body);

        self.builder.block_params(loop_exit)[0]
    }

    pub(crate) fn build_switch_integer_ir(&mut self, node: &SwitchInteger) -> Value {
        let test_value_f32 = self.build_node_ir(node.discriminant);
        let test_value_i64 = self.builder.ins().fcvt_to_sint(types::I64, test_value_f32);

        let mut switch = Switch::new();
        let mut case_blocks = Vec::new();
        for i in 0..node.consequents.len() {
            let block = self.builder.create_block();
            switch.set_entry(i as _, block);
            case_blocks.push(block);
        }

        let default_block = self.builder.create_block();
        let join_block = self.builder.create_block();
        let join_param = self.builder.append_block_param(join_block, crate::IR_VALUE_CRANELIFT_TYPE);

        switch.emit(self.builder, test_value_i64, default_block);

        for (i, &case_block) in case_blocks.iter().enumerate() {
            self.builder.switch_to_block(case_block);
            let (val, broken) =
                self.with_terminated_reset(|s| s.build_node_ir(node.consequents[i]));
            if !broken {
                self.builder.ins().jump(join_block, &[BlockArg::Value(val)]);
            }
            self.builder.seal_block(case_block);
        }
        self.builder.switch_to_block(default_block);
        let zero = crate::ir_value_cranelift_const(self.builder.ins(), 0.0);
        self.builder
            .ins()
            .jump(join_block, &[BlockArg::Value(zero)]);
        self.builder.seal_block(default_block);

        self.builder.switch_to_block(join_block);
        self.builder.seal_block(join_block);

        join_param
    }

    pub(crate) fn build_switch_integer_with_default_ir(
        &mut self,
        node: &SwitchIntegerWithDefault,
    ) -> Value {
        let test_value_f32 = self.build_node_ir(node.discriminant);
        let test_value_i64 = self.builder.ins().fcvt_to_sint(types::I64, test_value_f32);

        let mut switch = Switch::new();
        let mut case_blocks = Vec::new();
        for i in 0..node.consequents.len() {
            let block = self.builder.create_block();
            switch.set_entry(i as _, block);
            case_blocks.push(block);
        }

        let default_block = self.builder.create_block();
        let join_block = self.builder.create_block();
        let join_param = self.builder.append_block_param(join_block, crate::IR_VALUE_CRANELIFT_TYPE);

        switch.emit(self.builder, test_value_i64, default_block);

        for (i, &case_block) in case_blocks.iter().enumerate() {
            self.builder.switch_to_block(case_block);
            let (val, broken) =
                self.with_terminated_reset(|s| s.build_node_ir(node.consequents[i]));
            if !broken {
                self.builder.ins().jump(join_block, &[BlockArg::Value(val)]);
            }
            self.builder.seal_block(case_block);
        }
        self.builder.switch_to_block(default_block);
        let (val, broken) =
            self.with_terminated_reset(|s| s.build_node_ir(node.default_consequent));
        if !broken {
            self.builder.ins().jump(join_block, &[BlockArg::Value(val)]);
        }
        self.builder.seal_block(default_block);

        self.builder.switch_to_block(join_block);
        self.builder.seal_block(join_block);

        join_param
    }
}
