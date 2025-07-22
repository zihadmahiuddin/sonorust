use cranelift::prelude::*;

use crate::codegen::CodegenContext;
use crate::nodes::*;

impl<'s, 'b> CodegenContext<'s, 'b> {
    pub(crate) fn build_power(&mut self, a: Value, b: Value) -> Value {
        let fn_ref = self.externals_func_refs["pow"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, a, b]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_mod(&mut self, a: Value, b: Value) -> Value {
        let div = self.builder.ins().fdiv(a, b);
        let div_floor = self.builder.ins().floor(div);
        let mul = self.builder.ins().fmul(b, div_floor);
        self.builder.ins().fsub(a, mul)
    }

    pub(crate) fn build_rem(&mut self, a: Value, b: Value) -> Value {
        let div = self.builder.ins().fdiv(a, b);
        let div_trunc = self.builder.ins().trunc(div);
        let mul = self.builder.ins().fmul(b, div_trunc);
        self.builder.ins().fsub(a, mul)
    }

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

    pub(crate) fn build_power_ir(&mut self, node: &Power) -> Value {
        assert!(node.inputs.len() >= 2, "Power requires at least 2 inputs");

        let mut iter = node.inputs.iter();
        let first = self.build_node_ir(*iter.next().unwrap());
        let second = self.build_node_ir(*iter.next().unwrap());

        // Initial pow(ctx, a, b)
        let mut acc = self.build_power(first, second);

        for &input in iter {
            let exponent = self.build_node_ir(input);
            acc = self.build_power(acc, exponent);
        }

        acc
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
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
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
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
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
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
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
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
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
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
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
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
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
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_power_two_args() {
        let nodes = vec![
            ResolvedNode::Value(2.0),                                          // 0
            ResolvedNode::Value(3.0),                                          // 1
            ResolvedNode::OpCode(OpCode::Power(Power { inputs: vec![0, 1] })), // 2 = 2^3 = 8
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert!((result - 8.0).abs() < 1e-6);
    }

    #[test]
    fn test_power_three_args() {
        let nodes = vec![
            ResolvedNode::Value(2.0), // 0
            ResolvedNode::Value(3.0), // 1
            ResolvedNode::Value(2.0), // 2
            ResolvedNode::OpCode(OpCode::Power(Power {
                inputs: vec![0, 1, 2],
            })), // 3 = (2^3)^2 = 8^2 = 64
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert!((result - 64.0).abs() < 1e-6);
    }

    #[test]
    #[should_panic]
    fn test_power_single_panic() {
        let nodes = vec![
            ResolvedNode::Value(5.0),                                       // 0
            ResolvedNode::OpCode(OpCode::Power(Power { inputs: vec![0] })), // 1 = 5
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 1);
        let _result = func(&mut runtime_context as _);
    }

    #[test]
    fn test_power_negative_base_even_exponent() {
        let nodes = vec![
            ResolvedNode::Value(-2.0),                                         // 0
            ResolvedNode::Value(2.0),                                          // 1
            ResolvedNode::OpCode(OpCode::Power(Power { inputs: vec![0, 1] })), // 2 = (-2)^2 = 4
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert!((result - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_power_zero_base() {
        let nodes = vec![
            ResolvedNode::Value(0.0),                                          // 0
            ResolvedNode::Value(5.0),                                          // 1
            ResolvedNode::OpCode(OpCode::Power(Power { inputs: vec![0, 1] })), // 2 = 0^5 = 0
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert!((result - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_power_negative_exponent() {
        let nodes = vec![
            ResolvedNode::Value(4.0),                                          // 0
            ResolvedNode::Value(-1.0),                                         // 1
            ResolvedNode::OpCode(OpCode::Power(Power { inputs: vec![0, 1] })), // 2 = 4^-1 = 0.25
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert!((result - 0.25).abs() < 1e-6);
    }
}
