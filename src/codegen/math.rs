use cranelift::prelude::*;

use crate::codegen::CodegenContext;
use crate::nodes::*;

impl<'s, 'b> CodegenContext<'s, 'b> {
    pub(crate) fn build_add_ir(&mut self, node: &Add) -> Value {
        let mut result = self.builder.ins().f32const(0.0);
        for &input in &node.inputs {
            let val = self.build_node_ir(input);
            result = self.builder.ins().fadd(result, val);
        }
        result
    }

    pub(crate) fn build_subtract_ir(&mut self, node: &Subtract) -> Value {
        let mut inputs_iter = node.inputs.iter();
        let mut result = self.build_node_ir(
            *inputs_iter
                .next()
                .expect("At least 2 values required for subtraction."),
        );
        for &input in inputs_iter {
            let val = self.build_node_ir(input);
            result = self.builder.ins().fsub(result, val);
        }
        result
    }

    pub(crate) fn build_multiply_ir(&mut self, node: &Multiply) -> Value {
        let mut result = self.builder.ins().f32const(1.0);
        for &input in &node.inputs {
            let val = self.build_node_ir(input);
            result = self.builder.ins().fmul(result, val);
        }
        result
    }

    pub(crate) fn build_divide_ir(&mut self, node: &Divide) -> Value {
        let mut inputs_iter = node.inputs.iter();
        let mut result = self.build_node_ir(
            *inputs_iter
                .next()
                .expect("At least 2 values required for subtraction."),
        );
        for &input in inputs_iter {
            let val = self.build_node_ir(input);
            result = self.builder.ins().fdiv(result, val);
        }
        result
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
    fn test_add() {
        let nodes = vec![
            ResolvedNode::Value(2.0),                                      // 0
            ResolvedNode::Value(3.5),                                      // 1
            ResolvedNode::OpCode(OpCode::Add(Add { inputs: vec![0, 1] })), // 2
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 5.5);
    }

    #[test]
    fn test_add_chain() {
        let nodes = vec![
            ResolvedNode::Value(1.0),                                      // 0
            ResolvedNode::Value(2.0),                                      // 1
            ResolvedNode::Value(3.0),                                      // 2
            ResolvedNode::OpCode(OpCode::Add(Add { inputs: vec![0, 1] })), // 3 = 3.0
            ResolvedNode::OpCode(OpCode::Add(Add { inputs: vec![3, 2] })), // 4 = 6.0
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_subtract() {
        let nodes = vec![
            ResolvedNode::Value(10.0),
            ResolvedNode::Value(4.0),
            ResolvedNode::OpCode(OpCode::Subtract(Subtract { inputs: vec![0, 1] })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_subtract_negative() {
        let nodes = vec![
            ResolvedNode::Value(5.0),                                                // 0
            ResolvedNode::Value(10.0),                                               // 1
            ResolvedNode::OpCode(OpCode::Subtract(Subtract { inputs: vec![0, 1] })), // 2 = -5.0
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_multiply() {
        let nodes = vec![
            ResolvedNode::Value(3.0),
            ResolvedNode::Value(4.0),
            ResolvedNode::OpCode(OpCode::Multiply(Multiply { inputs: vec![0, 1] })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 12.0);
    }

    #[test]
    fn test_multiply_zero() {
        let nodes = vec![
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(999.0),
            ResolvedNode::OpCode(OpCode::Multiply(Multiply { inputs: vec![0, 1] })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_divide() {
        let nodes = vec![
            ResolvedNode::Value(9.0),
            ResolvedNode::Value(3.0),
            ResolvedNode::OpCode(OpCode::Divide(Divide { inputs: vec![0, 1] })),
        ];
        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 3.0);
    }
}
