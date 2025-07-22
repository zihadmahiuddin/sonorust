use cranelift::codegen::ir::BlockArg;
use cranelift::prelude::*;

use crate::codegen::CodegenContext;
use crate::nodes::*;

impl<'s, 'b> CodegenContext<'s, 'b> {
    fn build_comparison_ir(&mut self, cond: FloatCC, lhs: usize, rhs: usize) -> Value {
        let lhs = self.build_node_ir(lhs);
        let rhs = self.build_node_ir(rhs);
        let compare_result = self.builder.ins().fcmp(cond, lhs, rhs);
        self.builder
            .ins()
            .fcvt_from_sint(types::F32, compare_result)
    }

    pub(crate) fn build_equal_ir(&mut self, node: &Equal) -> Value {
        self.build_comparison_ir(FloatCC::Equal, node.lhs, node.rhs)
    }

    pub(crate) fn build_not_equal_ir(&mut self, node: &NotEqual) -> Value {
        self.build_comparison_ir(FloatCC::NotEqual, node.lhs, node.rhs)
    }

    pub(crate) fn build_greater_ir(&mut self, node: &Greater) -> Value {
        self.build_comparison_ir(FloatCC::GreaterThan, node.lhs, node.rhs)
    }

    pub(crate) fn build_greater_or_ir(&mut self, node: &GreaterOr) -> Value {
        self.build_comparison_ir(FloatCC::GreaterThanOrEqual, node.lhs, node.rhs)
    }

    pub(crate) fn build_less_ir(&mut self, node: &Less) -> Value {
        self.build_comparison_ir(FloatCC::LessThan, node.lhs, node.rhs)
    }

    pub(crate) fn build_less_or_ir(&mut self, node: &LessOr) -> Value {
        self.build_comparison_ir(FloatCC::LessThanOrEqual, node.lhs, node.rhs)
    }

    pub(crate) fn build_not_ir(&mut self, node: &Not) -> Value {
        let input = self.build_node_ir(node.value);
        let zero = self.builder.ins().f32const(0.0);
        let cmp = self.builder.ins().fcmp(FloatCC::Equal, input, zero);
        self.builder.ins().fcvt_from_sint(types::F32, cmp)
    }

    pub(crate) fn build_and_ir(&mut self, node: &And) -> Value {
        assert!(
            !node.inputs.is_empty(),
            "And requires at least one argument"
        );

        let result_block = self.builder.create_block();
        let result_param = self.builder.append_block_param(result_block, types::F32);

        let zero = self.builder.ins().f32const(0.0);

        let mut done = false;
        for (i, arg) in node.inputs.iter().enumerate() {
            let val = self.build_node_ir(*arg);

            let cond = self.builder.ins().fcmp(FloatCC::Equal, val, zero);

            if i == node.inputs.len() - 1 {
                self.builder
                    .ins()
                    .jump(result_block, &[BlockArg::Value(val)]);
                done = true;
                break;
            }

            let next_block = self.builder.create_block();

            self.builder.ins().brif(
                cond,
                result_block,
                &[BlockArg::Value(zero)],
                next_block,
                &[],
            );
            self.builder.seal_block(next_block);
            self.builder.switch_to_block(next_block);
        }

        if !done {
            self.builder
                .ins()
                .jump(result_block, &[BlockArg::Value(zero)]);
        }

        self.builder.switch_to_block(result_block);
        self.builder.seal_block(result_block);
        result_param
    }

    pub(crate) fn build_or_ir(&mut self, node: &Or) -> Value {
        assert!(!node.inputs.is_empty(), "Or requires at least one argument");

        let result_block = self.builder.create_block();
        let result_param = self.builder.append_block_param(result_block, types::F32);

        let zero = self.builder.ins().f32const(0.0);

        let mut done = false;
        for (i, arg) in node.inputs.iter().enumerate() {
            let val = self.build_node_ir(*arg);

            let cond = self.builder.ins().fcmp(FloatCC::NotEqual, val, zero);

            if i == node.inputs.len() - 1 {
                self.builder
                    .ins()
                    .jump(result_block, &[BlockArg::Value(val)]);
                done = true;
                break;
            }

            let next_block = self.builder.create_block();

            self.builder
                .ins()
                .brif(cond, result_block, &[BlockArg::Value(val)], next_block, &[]);
            self.builder.seal_block(next_block);
            self.builder.switch_to_block(next_block);
        }

        if !done {
            self.builder
                .ins()
                .jump(result_block, &[BlockArg::Value(zero)]);
        }

        self.builder.switch_to_block(result_block);
        self.builder.seal_block(result_block);
        result_param
    }
}

#[cfg(test)]
mod tests {
    use core::f32;

    use crate::{
        codegen::jit::build_and_return_function,
        nodes::*,
        runtime::{basic::BasicMemory, context::RuntimeContext},
    };

    #[test]
    fn test_not_equal_true() {
        let nodes = vec![
            ResolvedNode::Value(2.0),                                            // 0
            ResolvedNode::Value(3.5),                                            // 1
            ResolvedNode::OpCode(OpCode::NotEqual(NotEqual { lhs: 0, rhs: 1 })), // 2
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_not_equal_false() {
        let nodes = vec![
            ResolvedNode::Value(4.5),                                            // 0
            ResolvedNode::Value(4.5),                                            // 1
            ResolvedNode::OpCode(OpCode::NotEqual(NotEqual { lhs: 0, rhs: 1 })), // 2
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_equal_true() {
        let nodes = vec![
            ResolvedNode::Value(2.0),                                      // 0
            ResolvedNode::Value(2.0),                                      // 1
            ResolvedNode::OpCode(OpCode::Equal(Equal { lhs: 0, rhs: 1 })), // 2
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_equal_false() {
        let nodes = vec![
            ResolvedNode::Value(2.5),                                      // 0
            ResolvedNode::Value(4.5),                                      // 1
            ResolvedNode::OpCode(OpCode::Equal(Equal { lhs: 0, rhs: 1 })), // 2
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_greater_true() {
        let nodes = vec![
            ResolvedNode::Value(5.0),                                          // 0
            ResolvedNode::Value(2.0),                                          // 1
            ResolvedNode::OpCode(OpCode::Greater(Greater { lhs: 0, rhs: 1 })), // 2
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_greater_false() {
        let nodes = vec![
            ResolvedNode::Value(1.0),
            ResolvedNode::Value(10.0),
            ResolvedNode::OpCode(OpCode::Greater(Greater { lhs: 0, rhs: 1 })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_greater_or_true_equal() {
        let nodes = vec![
            ResolvedNode::Value(3.3),
            ResolvedNode::Value(3.3),
            ResolvedNode::OpCode(OpCode::GreaterOr(GreaterOr { lhs: 0, rhs: 1 })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_greater_or_true_gt() {
        let nodes = vec![
            ResolvedNode::Value(9.0),
            ResolvedNode::Value(7.5),
            ResolvedNode::OpCode(OpCode::GreaterOr(GreaterOr { lhs: 0, rhs: 1 })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_greater_or_false() {
        let nodes = vec![
            ResolvedNode::Value(1.0),
            ResolvedNode::Value(2.0),
            ResolvedNode::OpCode(OpCode::GreaterOr(GreaterOr { lhs: 0, rhs: 1 })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.0);
    }
    #[test]
    fn test_less_true() {
        let nodes = vec![
            ResolvedNode::Value(-5.0),
            ResolvedNode::Value(3.0),
            ResolvedNode::OpCode(OpCode::Less(Less { lhs: 0, rhs: 1 })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_less_false() {
        let nodes = vec![
            ResolvedNode::Value(10.0),
            ResolvedNode::Value(1.0),
            ResolvedNode::OpCode(OpCode::Less(Less { lhs: 0, rhs: 1 })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.0);
    }
    #[test]
    fn test_less_or_true_eq() {
        let nodes = vec![
            ResolvedNode::Value(4.4),
            ResolvedNode::Value(4.4),
            ResolvedNode::OpCode(OpCode::LessOr(LessOr { lhs: 0, rhs: 1 })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_less_or_true_lt() {
        let nodes = vec![
            ResolvedNode::Value(1.5),
            ResolvedNode::Value(2.0),
            ResolvedNode::OpCode(OpCode::LessOr(LessOr { lhs: 0, rhs: 1 })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_less_or_false() {
        let nodes = vec![
            ResolvedNode::Value(5.0),
            ResolvedNode::Value(3.0),
            ResolvedNode::OpCode(OpCode::LessOr(LessOr { lhs: 0, rhs: 1 })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_not_zero() {
        let nodes = vec![
            ResolvedNode::Value(0.0),                            // 0
            ResolvedNode::OpCode(OpCode::Not(Not { value: 0 })), // 1
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_not_nonzero() {
        let nodes = vec![
            ResolvedNode::Value(f32::consts::PI),                // 0
            ResolvedNode::OpCode(OpCode::Not(Not { value: 0 })), // 1
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_and_all_nonzero() {
        let nodes = vec![
            ResolvedNode::Value(1.0),                                      // 0
            ResolvedNode::Value(2.0),                                      // 1
            ResolvedNode::OpCode(OpCode::And(And { inputs: vec![0, 1] })), // 2
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 2.0); // Last input returned
    }

    #[test]
    fn test_and_with_zero_first() {
        let nodes = vec![
            ResolvedNode::Value(0.0),                                      // 0
            ResolvedNode::Value(2.0),                                      // 1
            ResolvedNode::OpCode(OpCode::And(And { inputs: vec![0, 1] })), // 2
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_and_with_zero_last() {
        let nodes = vec![
            ResolvedNode::Value(2.0),                                      // 0
            ResolvedNode::Value(0.0),                                      // 1
            ResolvedNode::OpCode(OpCode::And(And { inputs: vec![0, 1] })), // 2
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_or_all_zero() {
        let nodes = vec![
            ResolvedNode::Value(0.0),                                    // 0
            ResolvedNode::Value(0.0),                                    // 1
            ResolvedNode::OpCode(OpCode::Or(Or { inputs: vec![0, 1] })), // 2
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_or_first_nonzero() {
        let nodes = vec![
            ResolvedNode::Value(5.0),                                    // 0
            ResolvedNode::Value(0.0),                                    // 1
            ResolvedNode::OpCode(OpCode::Or(Or { inputs: vec![0, 1] })), // 2
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_or_second_nonzero() {
        let nodes = vec![
            ResolvedNode::Value(0.0),                                    // 0
            ResolvedNode::Value(8.0),                                    // 1
            ResolvedNode::OpCode(OpCode::Or(Or { inputs: vec![0, 1] })), // 2
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 8.0);
    }
}
