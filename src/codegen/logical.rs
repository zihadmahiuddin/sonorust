use cranelift::prelude::*;

use crate::codegen::CodegenContext;
use crate::nodes::*;

impl<'s, 'b> CodegenContext<'s, 'b> {
    pub(crate) fn build_not_equal_ir(&mut self, node: &NotEqual) -> Value {
        let lhs = self.build_node_ir(node.lhs);
        let rhs = self.build_node_ir(node.rhs);
        let compare_result = self.builder.ins().fcmp(FloatCC::NotEqual, lhs, rhs);
        self.builder
            .ins()
            .fcvt_from_sint(types::F32, compare_result)
    }

    pub(crate) fn build_equal_ir(&mut self, node: &Equal) -> Value {
        let lhs = self.build_node_ir(node.lhs);
        let rhs = self.build_node_ir(node.rhs);
        let compare_result = self.builder.ins().fcmp(FloatCC::Equal, lhs, rhs);
        self.builder
            .ins()
            .fcvt_from_sint(types::F32, compare_result)
    }
}

#[cfg(test)]
mod tests {
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
}
