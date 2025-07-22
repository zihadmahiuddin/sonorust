use cranelift::codegen::ir::BlockArg;
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

    pub(crate) fn build_abs_ir(&mut self, node: &Abs) -> Value {
        let value = self.build_node_ir(node.value);
        self.builder.ins().fabs(value)
    }

    pub(crate) fn build_frac_ir(&mut self, node: &Frac) -> Value {
        let value = self.build_node_ir(node.value);
        let truncated = self.builder.ins().trunc(value);
        self.builder.ins().fsub(value, truncated)
    }

    pub(crate) fn build_trunc_ir(&mut self, node: &Trunc) -> Value {
        let value = self.build_node_ir(node.value);
        self.builder.ins().trunc(value)
    }

    pub(crate) fn build_negate_ir(&mut self, node: &Negate) -> Value {
        let value = self.build_node_ir(node.value);
        self.builder.ins().fneg(value)
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

    pub(crate) fn build_mod_ir(&mut self, node: &Mod) -> Value {
        assert!(node.inputs.len() >= 2, "Mod requires at least 2 inputs");

        let mut iter = node.inputs.iter();
        let first = self.build_node_ir(*iter.next().unwrap());
        let second = self.build_node_ir(*iter.next().unwrap());

        // Initial mod(a, b)
        let mut acc = self.build_mod(first, second);

        for &input in iter {
            let next_val = self.build_node_ir(input);
            acc = self.build_mod(acc, next_val);
        }

        acc
    }

    pub(crate) fn build_rem_ir(&mut self, node: &Rem) -> Value {
        assert!(node.inputs.len() >= 2, "Rem requires at least 2 inputs");

        let mut iter = node.inputs.iter();
        let first = self.build_node_ir(*iter.next().unwrap());
        let second = self.build_node_ir(*iter.next().unwrap());

        // Initial rem(a, b)
        let mut acc = self.build_rem(first, second);

        for &input in iter {
            let next_val = self.build_node_ir(input);
            acc = self.build_rem(acc, next_val);
        }

        acc
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

    pub(crate) fn build_clamp_ir(&mut self, node: &Clamp) -> Value {
        let min = self.build_node_ir(node.min);
        let max = self.build_node_ir(node.max);
        let value = self.build_node_ir(node.value);

        let block_result = self.builder.create_block();
        let final_result = self.builder.append_block_param(block_result, types::F32);

        let block_min = self.builder.create_block();
        let block_max = self.builder.create_block();
        let block_value = self.builder.create_block();

        let lt_min = self.builder.ins().fcmp(FloatCC::LessThan, value, min);
        self.builder
            .ins()
            .brif(lt_min, block_min, &[], block_max, &[]);

        self.builder.switch_to_block(block_min);
        self.builder
            .ins()
            .jump(block_result, &[BlockArg::Value(min)]);

        self.builder.switch_to_block(block_max);
        let gt_max = self.builder.ins().fcmp(FloatCC::GreaterThan, value, max);
        self.builder.ins().brif(
            gt_max,
            block_result,
            &[BlockArg::Value(max)],
            block_value,
            &[],
        );

        self.builder.switch_to_block(block_value);
        self.builder
            .ins()
            .jump(block_result, &[BlockArg::Value(value)]);

        self.builder.switch_to_block(block_result);

        self.builder.seal_block(block_result);
        self.builder.seal_block(block_min);
        self.builder.seal_block(block_max);
        self.builder.seal_block(block_value);

        final_result
    }

    pub(crate) fn build_lerp_ir(&mut self, node: &Lerp) -> Value {
        let min = self.build_node_ir(node.min);
        let max = self.build_node_ir(node.max);
        let value = self.build_node_ir(node.value);

        let one = self.builder.ins().f32const(1.0);
        let one_minus_value = self.builder.ins().fsub(one, value);

        let min_mul_one_minus_value = self.builder.ins().fmul(min, one_minus_value);
        let max_mul_value = self.builder.ins().fmul(max, value);

        self.builder
            .ins()
            .fadd(min_mul_one_minus_value, max_mul_value)
    }

    pub(crate) fn build_unlerp_ir(&mut self, node: &Unlerp) -> Value {
        let min = self.build_node_ir(node.min);
        let max = self.build_node_ir(node.max);
        let value = self.build_node_ir(node.value);

        let value_minus_min = self.builder.ins().fsub(value, min);
        let max_minus_min = self.builder.ins().fsub(max, min);

        self.builder.ins().fdiv(value_minus_min, max_minus_min)
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
    fn test_abs_negative() {
        let nodes = vec![
            ResolvedNode::Value(-3.5),                           // 0
            ResolvedNode::OpCode(OpCode::Abs(Abs { value: 0 })), // 1
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_abs_positive() {
        let nodes = vec![
            ResolvedNode::Value(3.5),                            // 0
            ResolvedNode::OpCode(OpCode::Abs(Abs { value: 0 })), // 1
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_frac_negative() {
        let nodes = vec![
            ResolvedNode::Value(-5.75),                            // 0
            ResolvedNode::OpCode(OpCode::Frac(Frac { value: 0 })), // 1
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, -0.75);
    }

    #[test]
    fn test_frac_positive() {
        let nodes = vec![
            ResolvedNode::Value(5.75),                             // 0
            ResolvedNode::OpCode(OpCode::Frac(Frac { value: 0 })), // 1
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.75);
    }

    #[test]
    fn test_trunc_negative() {
        let nodes = vec![
            ResolvedNode::Value(-4.8),                               // 0
            ResolvedNode::OpCode(OpCode::Trunc(Trunc { value: 0 })), // 1
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, -4.0);
    }

    #[test]
    fn test_trunc_positive() {
        let nodes = vec![
            ResolvedNode::Value(4.8),                                // 0
            ResolvedNode::OpCode(OpCode::Trunc(Trunc { value: 0 })), // 1
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_negate_negative() {
        let nodes = vec![
            ResolvedNode::Value(-6.25),                                // 0
            ResolvedNode::OpCode(OpCode::Negate(Negate { value: 0 })), // 1
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 6.25);
    }

    #[test]
    fn test_negate_positive() {
        let nodes = vec![
            ResolvedNode::Value(6.25),                                 // 0
            ResolvedNode::OpCode(OpCode::Negate(Negate { value: 0 })), // 1
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, -6.25);
    }

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
    fn test_mod() {
        let nodes = vec![
            ResolvedNode::Value(-5.3),                                     // 0
            ResolvedNode::Value(2.0),                                      // 1
            ResolvedNode::OpCode(OpCode::Mod(Mod { inputs: vec![0, 1] })), // 2
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        let expected = 0.7;
        assert!(
            (result - expected).abs() < 1e-6,
            "got {result}, expected {expected}",
        );
    }

    #[test]
    fn test_rem() {
        let nodes = vec![
            ResolvedNode::Value(-5.3),                                     // 0
            ResolvedNode::Value(2.0),                                      // 1
            ResolvedNode::OpCode(OpCode::Rem(Rem { inputs: vec![0, 1] })), // 2
        ];
        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        let expected = -1.3;
        assert!(
            (result - expected).abs() < 1e-6,
            "got {result}, expected {expected}",
        );
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

    #[test]
    fn test_clamp_within_bounds() {
        let nodes = vec![
            ResolvedNode::Value(2.0), // 0 = min
            ResolvedNode::Value(5.0), // 1 = max
            ResolvedNode::Value(3.5), // 2 = value
            ResolvedNode::OpCode(OpCode::Clamp(Clamp {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_clamp_below_min() {
        let nodes = vec![
            ResolvedNode::Value(2.0), // 0 = min
            ResolvedNode::Value(5.0), // 1 = max
            ResolvedNode::Value(1.0), // 2 = value
            ResolvedNode::OpCode(OpCode::Clamp(Clamp {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_clamp_above_max() {
        let nodes = vec![
            ResolvedNode::Value(2.0), // 0 = min
            ResolvedNode::Value(5.0), // 1 = max
            ResolvedNode::Value(6.0), // 2 = value
            ResolvedNode::OpCode(OpCode::Clamp(Clamp {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_lerp_zero() {
        let nodes = vec![
            ResolvedNode::Value(10.0), // 0 = min
            ResolvedNode::Value(20.0), // 1 = max
            ResolvedNode::Value(0.0),  // 2 = value
            ResolvedNode::OpCode(OpCode::Lerp(Lerp {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_lerp_one() {
        let nodes = vec![
            ResolvedNode::Value(10.0), // 0 = min
            ResolvedNode::Value(20.0), // 1 = max
            ResolvedNode::Value(1.0),  // 2 = value
            ResolvedNode::OpCode(OpCode::Lerp(Lerp {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 20.0);
    }

    #[test]
    fn test_lerp_half() {
        let nodes = vec![
            ResolvedNode::Value(10.0), // 0 = min
            ResolvedNode::Value(20.0), // 1 = max
            ResolvedNode::Value(0.5),  // 2 = value
            ResolvedNode::OpCode(OpCode::Lerp(Lerp {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_lerp_past_one() {
        let nodes = vec![
            ResolvedNode::Value(10.0), // 0 = min
            ResolvedNode::Value(20.0), // 1 = max
            ResolvedNode::Value(1.5),  // 2 = value
            ResolvedNode::OpCode(OpCode::Lerp(Lerp {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 25.0);
    }

    #[test]
    fn test_unlerp_zero() {
        let nodes = vec![
            ResolvedNode::Value(10.0), // 0 = min
            ResolvedNode::Value(20.0), // 1 = max
            ResolvedNode::Value(10.0), // 2 = value
            ResolvedNode::OpCode(OpCode::Unlerp(Unlerp {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_unlerp_one() {
        let nodes = vec![
            ResolvedNode::Value(10.0), // 0 = min
            ResolvedNode::Value(20.0), // 1 = max
            ResolvedNode::Value(20.0), // 2 = value
            ResolvedNode::OpCode(OpCode::Unlerp(Unlerp {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_unlerp_half() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // 0 = min
            ResolvedNode::Value(2.0), // 1 = max
            ResolvedNode::Value(1.0), // 2 = value
            ResolvedNode::OpCode(OpCode::Unlerp(Unlerp {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_unlerp_below_min() {
        let nodes = vec![
            ResolvedNode::Value(5.0),  // 0 = min
            ResolvedNode::Value(10.0), // 1 = max
            ResolvedNode::Value(0.0),  // 2 = value
            ResolvedNode::OpCode(OpCode::Unlerp(Unlerp {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, -1.0);
    }

    #[test]
    fn test_unlerp_above_max() {
        let nodes = vec![
            ResolvedNode::Value(5.0),  // 0 = min
            ResolvedNode::Value(10.0), // 1 = max
            ResolvedNode::Value(15.0), // 2 = value
            ResolvedNode::OpCode(OpCode::Unlerp(Unlerp {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 2.0);
    }
}
