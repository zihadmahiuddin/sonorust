use cranelift::{codegen::ir::BlockArg, frontend::Switch, prelude::*};

use crate::codegen::{BlockKind, CodegenContext};
use crate::nodes::{self, *};

impl<'s, 'b> CodegenContext<'s, 'b> {
    pub(crate) fn build_execute_ir(&mut self, node: &Execute) -> Value {
        let mut last_val = self.builder.ins().f32const(0.0);
        for &child_index in &node.nodes {
            last_val = self.build_node_ir(child_index);
        }
        last_val
    }

    pub(crate) fn build_execute0_ir(&mut self, node: &Execute0) -> Value {
        self.build_execute_ir(&Execute {
            nodes: node.nodes.clone(),
        });
        self.builder.ins().f32const(0.0)
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

        let zero = self.builder.ins().f32const(0.0);
        let condition = self.builder.ins().fcmp(FloatCC::NotEqual, test_val, zero);

        let then_block = self.builder.create_block();
        let else_block = self.builder.create_block();
        let join_block = self.builder.create_block();

        self.builder.append_block_param(join_block, types::F32);

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
            .append_block_param(block_kind.exit_block(), types::F32);

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
        let zero = self.builder.ins().f32const(0.0);
        let one = self.builder.ins().f32const(1.0);
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
        let join_param = self.builder.append_block_param(join_block, types::F32);

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
        let zero = self.builder.ins().f32const(0.0);
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
        let join_param = self.builder.append_block_param(join_block, types::F32);

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

#[cfg(test)]
mod tests {
    use crate::{
        codegen::jit::build_and_return_function,
        nodes::*,
        runtime::{basic::BasicRuntimeContext, context::MemoryAccess},
    };

    #[test]
    fn test_execute() {
        let nodes = vec![
            ResolvedNode::Value(1.0),
            ResolvedNode::Value(2.0),
            ResolvedNode::OpCode(OpCode::Add(Add { inputs: vec![0, 1] })), // = 3.0
            ResolvedNode::Value(100.0),
            ResolvedNode::OpCode(OpCode::Execute(Execute { nodes: vec![2, 3] })), // should return 100.0
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 100.0);
    }

    #[test]
    fn test_execute_0() {
        let nodes = vec![
            ResolvedNode::Value(1.0),
            ResolvedNode::Value(2.0),
            ResolvedNode::OpCode(OpCode::Add(Add { inputs: vec![0, 1] })), // = 3.0
            ResolvedNode::Value(100.0),
            ResolvedNode::OpCode(OpCode::Execute0(Execute0 { nodes: vec![2, 3] })), // should return 100.0
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_execute_chained() {
        let nodes = vec![
            ResolvedNode::Value(1.0),                                                // 0
            ResolvedNode::Value(2.0),                                                // 1
            ResolvedNode::OpCode(OpCode::Add(Add { inputs: vec![0, 1] })),           // 2 = 3.0
            ResolvedNode::Value(5.0),                                                // 3
            ResolvedNode::OpCode(OpCode::Multiply(Multiply { inputs: vec![2, 3] })), // 4 = 15.0
            ResolvedNode::OpCode(OpCode::Execute(Execute { nodes: vec![2, 4] })),    // returns 15.0
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_if_true() {
        let nodes = vec![
            ResolvedNode::Value(1.0),  // test (true)
            ResolvedNode::Value(42.0), // consequent
            ResolvedNode::Value(99.0), // alternate
            ResolvedNode::OpCode(OpCode::If(If {
                test: 0,
                consequent: 1,
                alternate: 2,
            })),
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_if_false() {
        let nodes = vec![
            ResolvedNode::Value(0.0),  // test (false)
            ResolvedNode::Value(42.0), // consequent
            ResolvedNode::Value(99.0), // alternate
            ResolvedNode::OpCode(OpCode::If(If {
                test: 0,
                consequent: 1,
                alternate: 2,
            })),
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 99.0);
    }

    #[test]
    fn test_if_with_expression_consequent() {
        let nodes = vec![
            ResolvedNode::Value(1.0), // test = true
            ResolvedNode::Value(2.0),
            ResolvedNode::Value(3.0),
            ResolvedNode::OpCode(OpCode::Add(Add { inputs: vec![1, 2] })), // 3 = 5.0
            ResolvedNode::Value(100.0),                                    // alternate
            ResolvedNode::OpCode(OpCode::If(If {
                test: 0,
                consequent: 3,
                alternate: 4,
            })),
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_if_with_expression_alternate() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // test = false
            ResolvedNode::Value(2.0),
            ResolvedNode::Value(3.0),
            ResolvedNode::OpCode(OpCode::Multiply(Multiply { inputs: vec![1, 2] })), // 3 = 6.0
            ResolvedNode::Value(42.0),                                               // consequent
            ResolvedNode::OpCode(OpCode::If(If {
                test: 0,
                consequent: 4,
                alternate: 3,
            })),
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_nested_if() {
        let nodes = vec![
            ResolvedNode::Value(1.0),  // test 0 (true)
            ResolvedNode::Value(0.0),  // test 1 (false)
            ResolvedNode::Value(11.0), // alternate inner
            ResolvedNode::Value(22.0), // consequent inner
            ResolvedNode::OpCode(OpCode::If(If {
                test: 1,
                consequent: 3,
                alternate: 2,
            })), // inner If = 11.0
            ResolvedNode::OpCode(OpCode::If(If {
                test: 0,
                consequent: 4,
                alternate: 2,
            })), // outer If = 11.0
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 11.0);
    }

    #[test]
    fn test_block_returns_value() {
        let nodes = vec![
            ResolvedNode::Value(42.0),                              // 0: body value
            ResolvedNode::OpCode(OpCode::Block(Block { body: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_block_returns_from_break() {
        let nodes = vec![
            ResolvedNode::Value(99.0), // 0: break value
            ResolvedNode::Value(1.0),  // 1: break count
            ResolvedNode::OpCode(OpCode::Break(Break { value: 0, count: 1 })), // 2
            ResolvedNode::OpCode(OpCode::Block(Block { body: 2 })), // 3
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 99.0);
    }

    #[test]
    fn test_block_with_if_breaks() {
        let nodes = vec![
            ResolvedNode::Value(55.5), // 0: break value
            ResolvedNode::Value(1.0),  // 1: break count
            ResolvedNode::OpCode(OpCode::Break(Break { value: 0, count: 1 })), // 2
            ResolvedNode::Value(1.0),  // 3: true condition
            ResolvedNode::Value(0.0),  // 4: else branch
            ResolvedNode::OpCode(OpCode::If(If {
                test: 3,
                consequent: 2,
                alternate: 4,
            })), // 5
            ResolvedNode::OpCode(OpCode::Block(Block { body: 5 })), // 6
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 6);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 55.5);
    }

    #[test]
    fn test_nested_block_breaks_outer() {
        let nodes = vec![
            ResolvedNode::Value(123.0), // 0: break value
            ResolvedNode::Value(2.0),   // 1: break count
            ResolvedNode::OpCode(OpCode::Break(Break { value: 0, count: 1 })), // 2
            ResolvedNode::OpCode(OpCode::Block(Block { body: 2 })), // 3
            ResolvedNode::OpCode(OpCode::Block(Block { body: 3 })), // 4
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 123.0);
    }

    #[test]
    fn test_while_immediate_break() {
        // simulate: while(true) { break(1, 42.0) }
        let nodes = vec![
            ResolvedNode::Value(42.0), // 0: break value
            ResolvedNode::Value(1.0),  // 1: break count (pop 1 block)
            ResolvedNode::OpCode(OpCode::Break(Break {
                // 2: break node
                count: 1,
                value: 0,
            })),
            ResolvedNode::Value(1.0), // 3: test (true)
            ResolvedNode::OpCode(OpCode::Block(Block {
                // 4: block node, body of while
                body: 2,
            })),
            ResolvedNode::OpCode(OpCode::While(While {
                // 5: while node
                test: 3,
                body: 4,
            })),
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_while_without_break() {
        // simulate: while(get(0) != 42) { set(0, add(get(0), 1)) }
        let nodes = vec![
            ResolvedNode::Value(0.0), // 0: memory index 0
            ResolvedNode::OpCode(OpCode::Get(Get {
                block_id: 0,
                index: 0,
            })), // 1: get(0)
            ResolvedNode::Value(42.0), // 2: const 42
            ResolvedNode::OpCode(OpCode::NotEqual(NotEqual { lhs: 1, rhs: 2 })), // 3: get(0) != 42
            ResolvedNode::Value(0.0), // 4: memory index 0
            ResolvedNode::OpCode(OpCode::Get(Get {
                block_id: 0,
                index: 0,
            })), // 5: get(0)
            ResolvedNode::Value(1.0), // 6: const 1
            ResolvedNode::OpCode(OpCode::Add(Add { inputs: vec![5, 6] })), // 7: get(0) + 1
            ResolvedNode::OpCode(OpCode::Set(Set {
                block_id: 0,
                index: 4,
                value: 7,
            })), // 8: set(0, ...)
            ResolvedNode::OpCode(OpCode::Block(Block { body: 8 })), // 9: block with store
            ResolvedNode::OpCode(OpCode::While(While { test: 3, body: 9 })), // 10: while
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 10);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.0);
        assert_eq!(runtime_context.memory.read(0, 0), Some(42.0));
    }

    #[test]
    fn test_switch_integer() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // 0 - literal 0
            ResolvedNode::OpCode(OpCode::Get(Get {
                block_id: 0,
                index: 0,
            })), // 1
            ResolvedNode::Value(10.0), // 2 - body of case 0
            ResolvedNode::Value(20.0), // 3 - body of case 1
            ResolvedNode::OpCode(OpCode::SwitchInteger(SwitchInteger {
                discriminant: 1,
                consequents: vec![2, 3],
            })), // 4
        ];
        let mut runtime_context = BasicRuntimeContext::default();

        // matches case 0
        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 10.0);

        runtime_context.memory.write(0, 0, 1.0);

        // matches case 1
        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 20.0);

        runtime_context.memory.write(0, 0, 5.0);

        // no match → default
        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_switch_integer_with_default() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // 0 - literal 0
            ResolvedNode::OpCode(OpCode::Get(Get {
                block_id: 0,
                index: 0,
            })), // 1
            ResolvedNode::Value(10.0), // 2 - body of case 0
            ResolvedNode::Value(20.0), // 3 - body of case 1
            ResolvedNode::Value(99.0), // 4 - default
            ResolvedNode::OpCode(OpCode::SwitchIntegerWithDefault(SwitchIntegerWithDefault {
                discriminant: 1,
                consequents: vec![2, 3],
                default_consequent: 4,
            })), // 5
        ];
        let mut runtime_context = BasicRuntimeContext::default();

        // matches case 0
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 10.0);

        runtime_context.memory.write(0, 0, 1.0);

        // matches case 1
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 20.0);

        runtime_context.memory.write(0, 0, 5.0);

        // no match → default
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 99.0);
    }
}
