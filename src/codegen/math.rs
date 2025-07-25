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

    fn build_clamp(&mut self, min: Value, max: Value, value: Value) -> Value {
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

    fn build_lerp(&mut self, min: Value, max: Value, value: Value) -> Value {
        let one = self.builder.ins().f32const(1.0);
        let one_minus_value = self.builder.ins().fsub(one, value);

        let min_mul_one_minus_value = self.builder.ins().fmul(min, one_minus_value);
        let max_mul_value = self.builder.ins().fmul(max, value);

        self.builder
            .ins()
            .fadd(min_mul_one_minus_value, max_mul_value)
    }

    fn build_unlerp(&mut self, min: Value, max: Value, value: Value) -> Value {
        let value_minus_min = self.builder.ins().fsub(value, min);
        let max_minus_min = self.builder.ins().fsub(max, min);

        self.builder.ins().fdiv(value_minus_min, max_minus_min)
    }

    fn build_unlerp_clamped(&mut self, min: Value, max: Value, value: Value) -> Value {
        let block_return_zero = self.builder.create_block();
        let block_return_unlerp_clamped = self.builder.create_block();
        let block_join = self.builder.create_block();
        let block_join_param = self.builder.append_block_param(block_join, types::F32);

        let zero = self.builder.ins().f32const(0.0);
        let one = self.builder.ins().f32const(1.0);

        let min_eq_max = self.builder.ins().fcmp(FloatCC::Equal, min, max);
        self.builder.ins().brif(
            min_eq_max,
            block_return_zero,
            [],
            block_return_unlerp_clamped,
            [],
        );

        self.builder.switch_to_block(block_return_zero);
        self.builder
            .ins()
            .jump(block_join, &[BlockArg::Value(zero)]);

        self.builder.switch_to_block(block_return_unlerp_clamped);
        let lerped = self.build_unlerp(min, max, value);
        let clamped = self.build_clamp(zero, one, lerped);
        self.builder
            .ins()
            .jump(block_join, &[BlockArg::Value(clamped)]);

        self.builder.switch_to_block(block_join);
        self.builder.seal_block(block_join);
        self.builder.seal_block(block_return_unlerp_clamped);
        self.builder.seal_block(block_return_zero);
        block_join_param
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

        self.build_clamp(min, max, value)
    }

    pub(crate) fn build_lerp_ir(&mut self, node: &Lerp) -> Value {
        let min = self.build_node_ir(node.min);
        let max = self.build_node_ir(node.max);
        let value = self.build_node_ir(node.value);

        self.build_lerp(min, max, value)
    }

    pub(crate) fn build_lerp_clamped_ir(&mut self, node: &LerpClamped) -> Value {
        let zero = self.builder.ins().f32const(0.0);
        let one = self.builder.ins().f32const(1.0);
        let value = self.build_node_ir(node.value);

        let clamped = self.build_clamp(zero, one, value);

        let min = self.build_node_ir(node.min);
        let max = self.build_node_ir(node.max);

        self.build_lerp(min, max, clamped)
    }

    pub(crate) fn build_unlerp_ir(&mut self, node: &Unlerp) -> Value {
        let min = self.build_node_ir(node.min);
        let max = self.build_node_ir(node.max);
        let value = self.build_node_ir(node.value);

        self.build_unlerp(min, max, value)
    }

    pub(crate) fn build_unlerp_clamped_ir(&mut self, node: &UnlerpClamped) -> Value {
        let min = self.build_node_ir(node.min);
        let max = self.build_node_ir(node.max);
        let value = self.build_node_ir(node.value);
        self.build_unlerp_clamped(min, max, value)
    }

    pub(crate) fn build_min_ir(&mut self, node: &Min) -> Value {
        let x = self.build_node_ir(node.x);
        let y = self.build_node_ir(node.y);
        self.builder.ins().fmin(x, y)
    }

    pub(crate) fn build_max_ir(&mut self, node: &Max) -> Value {
        let x = self.build_node_ir(node.x);
        let y = self.build_node_ir(node.y);
        self.builder.ins().fmax(x, y)
    }

    pub(crate) fn build_remap_ir(&mut self, node: &Remap) -> Value {
        let from_min = self.build_node_ir(node.from_min);
        let from_max = self.build_node_ir(node.from_max);
        let to_min = self.build_node_ir(node.to_min);
        let to_max = self.build_node_ir(node.to_max);
        let value = self.build_node_ir(node.value);

        let unlerped = self.build_unlerp(from_min, from_max, value);
        self.build_lerp(to_min, to_max, unlerped)
    }

    pub(crate) fn build_remap_clamped_ir(&mut self, node: &RemapClamped) -> Value {
        let from_min = self.build_node_ir(node.from_min);
        let from_max = self.build_node_ir(node.from_max);
        let to_min = self.build_node_ir(node.to_min);
        let to_max = self.build_node_ir(node.to_max);
        let value = self.build_node_ir(node.value);

        let unlerped = self.build_unlerp_clamped(from_min, from_max, value);
        self.build_lerp(to_min, to_max, unlerped)
    }

    pub(crate) fn build_round_ir(&mut self, node: &Round) -> Value {
        let value = self.build_node_ir(node.value);
        self.builder.ins().nearest(value)
    }

    pub(crate) fn build_floor_ir(&mut self, node: &Floor) -> Value {
        let value = self.build_node_ir(node.value);
        self.builder.ins().floor(value)
    }

    pub(crate) fn build_ceil_ir(&mut self, node: &Ceil) -> Value {
        let value = self.build_node_ir(node.value);
        self.builder.ins().ceil(value)
    }

    pub(crate) fn build_sin_ir(&mut self, node: &Sin) -> Value {
        let value = self.build_node_ir(node.value);
        let fn_ref = self.externals_func_refs["sin"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, value]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_cos_ir(&mut self, node: &Cos) -> Value {
        let value = self.build_node_ir(node.value);
        let fn_ref = self.externals_func_refs["cos"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, value]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_tan_ir(&mut self, node: &Tan) -> Value {
        let value = self.build_node_ir(node.value);
        let fn_ref = self.externals_func_refs["tan"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, value]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_sinh_ir(&mut self, node: &Sinh) -> Value {
        let value = self.build_node_ir(node.value);
        let fn_ref = self.externals_func_refs["sinh"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, value]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_cosh_ir(&mut self, node: &Cosh) -> Value {
        let value = self.build_node_ir(node.value);
        let fn_ref = self.externals_func_refs["cosh"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, value]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_tanh_ir(&mut self, node: &Tanh) -> Value {
        let value = self.build_node_ir(node.value);
        let fn_ref = self.externals_func_refs["tanh"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, value]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_arcsin_ir(&mut self, node: &Arcsin) -> Value {
        let value = self.build_node_ir(node.value);
        let fn_ref = self.externals_func_refs["arcsin"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, value]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_arccos_ir(&mut self, node: &Arccos) -> Value {
        let value = self.build_node_ir(node.value);
        let fn_ref = self.externals_func_refs["arccos"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, value]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_arctan_ir(&mut self, node: &Arctan) -> Value {
        let value = self.build_node_ir(node.value);
        let fn_ref = self.externals_func_refs["arctan"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, value]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_arctan2_ir(&mut self, node: &Arctan2) -> Value {
        let x = self.build_node_ir(node.x);
        let y = self.build_node_ir(node.y);
        let fn_ref = self.externals_func_refs["arctan2"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, x, y]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_degree_ir(&mut self, node: &Degree) -> Value {
        let value = self.build_node_ir(node.value);
        let fn_ref = self.externals_func_refs["degree"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, value]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_radian_ir(&mut self, node: &Radian) -> Value {
        let value = self.build_node_ir(node.value);
        let fn_ref = self.externals_func_refs["radian"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, value]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_log_ir(&mut self, node: &Log) -> Value {
        let value = self.build_node_ir(node.value);
        let fn_ref = self.externals_func_refs["log"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, value]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_sign_ir(&mut self, node: &Sign) -> Value {
        let value = self.build_node_ir(node.value);

        let zero = self.builder.ins().f32const(0.0);
        let one = self.builder.ins().f32const(1.0);
        let neg_one = self.builder.ins().f32const(-1.0);

        let block_nan = self.builder.create_block();
        let block_pos = self.builder.create_block();
        let block_neg = self.builder.create_block();
        let block_zero = self.builder.create_block();
        let block_merge = self.builder.create_block();
        let result = self.builder.append_block_param(block_merge, types::F32);

        let is_nan = self.builder.ins().fcmp(FloatCC::NotEqual, value, value);
        self.builder
            .ins()
            .brif(is_nan, block_nan, &[], block_pos, &[]);

        self.builder.switch_to_block(block_nan);
        let nan_val = self.builder.ins().f32const(f32::NAN);
        self.builder
            .ins()
            .jump(block_merge, &[BlockArg::Value(nan_val)]);

        self.builder.switch_to_block(block_pos);
        let gt_zero = self.builder.ins().fcmp(FloatCC::GreaterThan, value, zero);
        self.builder.ins().brif(
            gt_zero,
            block_merge,
            &[BlockArg::Value(one)],
            block_neg,
            &[],
        );

        self.builder.switch_to_block(block_neg);
        let lt_zero = self.builder.ins().fcmp(FloatCC::LessThan, value, zero);
        self.builder.ins().brif(
            lt_zero,
            block_merge,
            &[BlockArg::Value(neg_one)],
            block_zero,
            &[],
        );

        self.builder.switch_to_block(block_zero);
        self.builder
            .ins()
            .jump(block_merge, &[BlockArg::Value(zero)]);

        self.builder.switch_to_block(block_merge);
        self.builder.seal_block(block_nan);
        self.builder.seal_block(block_pos);
        self.builder.seal_block(block_neg);
        self.builder.seal_block(block_zero);
        self.builder.seal_block(block_merge);

        result
    }

    pub(crate) fn build_random_ir(&mut self, node: &Random) -> Value {
        let min = self.build_node_ir(node.min);
        let max = self.build_node_ir(node.max);
        let fn_ref = self.externals_func_refs["random"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, min, max]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_random_integer_ir(&mut self, node: &RandomInteger) -> Value {
        let min = self.build_node_ir(node.min);
        let max = self.build_node_ir(node.max);
        let fn_ref = self.externals_func_refs["random_integer"];
        let fn_call = self.builder.ins().call(fn_ref, &[self.ctx_param, min, max]);
        self.builder.inst_results(fn_call)[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        codegen::jit::build_and_return_function, nodes::*, runtime::basic::BasicRuntimeContext,
    };

    #[test]
    fn test_abs_negative() {
        let nodes = vec![
            ResolvedNode::Value(-3.5),                           // 0
            ResolvedNode::OpCode(OpCode::Abs(Abs { value: 0 })), // 1
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_abs_positive() {
        let nodes = vec![
            ResolvedNode::Value(3.5),                            // 0
            ResolvedNode::OpCode(OpCode::Abs(Abs { value: 0 })), // 1
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_frac_negative() {
        let nodes = vec![
            ResolvedNode::Value(-5.75),                            // 0
            ResolvedNode::OpCode(OpCode::Frac(Frac { value: 0 })), // 1
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, -0.75);
    }

    #[test]
    fn test_frac_positive() {
        let nodes = vec![
            ResolvedNode::Value(5.75),                             // 0
            ResolvedNode::OpCode(OpCode::Frac(Frac { value: 0 })), // 1
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.75);
    }

    #[test]
    fn test_trunc_negative() {
        let nodes = vec![
            ResolvedNode::Value(-4.8),                               // 0
            ResolvedNode::OpCode(OpCode::Trunc(Trunc { value: 0 })), // 1
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, -4.0);
    }

    #[test]
    fn test_trunc_positive() {
        let nodes = vec![
            ResolvedNode::Value(4.8),                                // 0
            ResolvedNode::OpCode(OpCode::Trunc(Trunc { value: 0 })), // 1
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_negate_negative() {
        let nodes = vec![
            ResolvedNode::Value(-6.25),                                // 0
            ResolvedNode::OpCode(OpCode::Negate(Negate { value: 0 })), // 1
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 6.25);
    }

    #[test]
    fn test_negate_positive() {
        let nodes = vec![
            ResolvedNode::Value(6.25),                                 // 0
            ResolvedNode::OpCode(OpCode::Negate(Negate { value: 0 })), // 1
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, -6.25);
    }

    #[test]
    fn test_add() {
        let nodes = vec![
            ResolvedNode::Value(2.0),                                      // 0
            ResolvedNode::Value(3.5),                                      // 1
            ResolvedNode::OpCode(OpCode::Add(Add { inputs: vec![0, 1] })), // 2
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
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
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_subtract() {
        let nodes = vec![
            ResolvedNode::Value(10.0),
            ResolvedNode::Value(4.0),
            ResolvedNode::OpCode(OpCode::Subtract(Subtract { inputs: vec![0, 1] })),
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_subtract_negative() {
        let nodes = vec![
            ResolvedNode::Value(5.0),                                                // 0
            ResolvedNode::Value(10.0),                                               // 1
            ResolvedNode::OpCode(OpCode::Subtract(Subtract { inputs: vec![0, 1] })), // 2 = -5.0
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_multiply() {
        let nodes = vec![
            ResolvedNode::Value(3.0),
            ResolvedNode::Value(4.0),
            ResolvedNode::OpCode(OpCode::Multiply(Multiply { inputs: vec![0, 1] })),
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 12.0);
    }

    #[test]
    fn test_multiply_zero() {
        let nodes = vec![
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(999.0),
            ResolvedNode::OpCode(OpCode::Multiply(Multiply { inputs: vec![0, 1] })),
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_divide() {
        let nodes = vec![
            ResolvedNode::Value(9.0),
            ResolvedNode::Value(3.0),
            ResolvedNode::OpCode(OpCode::Divide(Divide { inputs: vec![0, 1] })),
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_mod() {
        let nodes = vec![
            ResolvedNode::Value(-5.3),                                     // 0
            ResolvedNode::Value(2.0),                                      // 1
            ResolvedNode::OpCode(OpCode::Mod(Mod { inputs: vec![0, 1] })), // 2
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
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
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
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
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
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
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert!((result - 64.0).abs() < 1e-6);
    }

    #[test]
    #[should_panic]
    fn test_power_single_panic() {
        let nodes = vec![
            ResolvedNode::Value(5.0),                                       // 0
            ResolvedNode::OpCode(OpCode::Power(Power { inputs: vec![0] })), // 1 = 5
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let _result = func(&mut runtime_context.as_ctx() as _);
    }

    #[test]
    fn test_power_negative_base_even_exponent() {
        let nodes = vec![
            ResolvedNode::Value(-2.0),                                         // 0
            ResolvedNode::Value(2.0),                                          // 1
            ResolvedNode::OpCode(OpCode::Power(Power { inputs: vec![0, 1] })), // 2 = (-2)^2 = 4
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert!((result - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_power_zero_base() {
        let nodes = vec![
            ResolvedNode::Value(0.0),                                          // 0
            ResolvedNode::Value(5.0),                                          // 1
            ResolvedNode::OpCode(OpCode::Power(Power { inputs: vec![0, 1] })), // 2 = 0^5 = 0
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert!((result - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_power_negative_exponent() {
        let nodes = vec![
            ResolvedNode::Value(4.0),                                          // 0
            ResolvedNode::Value(-1.0),                                         // 1
            ResolvedNode::OpCode(OpCode::Power(Power { inputs: vec![0, 1] })), // 2 = 4^-1 = 0.25
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
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

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
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

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
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

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
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

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
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

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
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

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
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

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 25.0);
    }

    #[test]
    fn test_lerp_clamped_below_zero() {
        let nodes = vec![
            ResolvedNode::Value(100.0), // 0 = min
            ResolvedNode::Value(200.0), // 1 = max
            ResolvedNode::Value(-1.0),  // 2 = value (t < 0)
            ResolvedNode::OpCode(OpCode::LerpClamped(LerpClamped {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 100.0); // clamped to 0.0 → lerp(min, max, 0.0) = min
    }

    #[test]
    fn test_lerp_clamped_above_one() {
        let nodes = vec![
            ResolvedNode::Value(100.0), // 0 = min
            ResolvedNode::Value(200.0), // 1 = max
            ResolvedNode::Value(2.0),   // 2 = value (t > 1)
            ResolvedNode::OpCode(OpCode::LerpClamped(LerpClamped {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 200.0); // clamped to 1.0 → lerp(min, max, 1.0) = max
    }

    #[test]
    fn test_lerp_clamped_midpoint() {
        let nodes = vec![
            ResolvedNode::Value(100.0), // 0 = min
            ResolvedNode::Value(200.0), // 1 = max
            ResolvedNode::Value(0.5),   // 2 = value (t = 0.5)
            ResolvedNode::OpCode(OpCode::LerpClamped(LerpClamped {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 150.0); // lerp(min, max, 0.5)
    }

    #[test]
    fn test_lerp_clamped_exact_zero() {
        let nodes = vec![
            ResolvedNode::Value(10.0), // 0 = min
            ResolvedNode::Value(20.0), // 1 = max
            ResolvedNode::Value(0.0),  // 2 = value
            ResolvedNode::OpCode(OpCode::LerpClamped(LerpClamped {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 10.0); // no clamping needed
    }

    #[test]
    fn test_lerp_clamped_exact_one() {
        let nodes = vec![
            ResolvedNode::Value(10.0), // 0 = min
            ResolvedNode::Value(20.0), // 1 = max
            ResolvedNode::Value(1.0),  // 2 = value
            ResolvedNode::OpCode(OpCode::LerpClamped(LerpClamped {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 20.0); // no clamping needed
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

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
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

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
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

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
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

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
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

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_unlerp_clamped_within_range() {
        let nodes = vec![
            ResolvedNode::Value(0.0),  // 0 = min
            ResolvedNode::Value(10.0), // 1 = max
            ResolvedNode::Value(5.0),  // 2 = value
            ResolvedNode::OpCode(OpCode::UnlerpClamped(UnlerpClamped {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_unlerp_clamped_below_min() {
        let nodes = vec![
            ResolvedNode::Value(0.0),  // 0 = min
            ResolvedNode::Value(10.0), // 1 = max
            ResolvedNode::Value(-5.0), // 2 = value
            ResolvedNode::OpCode(OpCode::UnlerpClamped(UnlerpClamped {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_unlerp_clamped_above_max() {
        let nodes = vec![
            ResolvedNode::Value(0.0),  // 0 = min
            ResolvedNode::Value(10.0), // 1 = max
            ResolvedNode::Value(15.0), // 2 = value
            ResolvedNode::OpCode(OpCode::UnlerpClamped(UnlerpClamped {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_unlerp_clamped_min_equals_max() {
        let nodes = vec![
            ResolvedNode::Value(10.0), // 0 = min
            ResolvedNode::Value(10.0), // 1 = max
            ResolvedNode::Value(15.0), // 2 = value (irrelevant)
            ResolvedNode::OpCode(OpCode::UnlerpClamped(UnlerpClamped {
                min: 0,
                max: 1,
                value: 2,
            })), // 3
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_min_left_smaller() {
        let nodes = vec![
            ResolvedNode::Value(2.0),                              // 0 = x
            ResolvedNode::Value(5.0),                              // 1 = y
            ResolvedNode::OpCode(OpCode::Min(Min { x: 0, y: 1 })), // 2
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_min_right_smaller() {
        let nodes = vec![
            ResolvedNode::Value(7.0),                              // 0 = x
            ResolvedNode::Value(3.0),                              // 1 = y
            ResolvedNode::OpCode(OpCode::Min(Min { x: 0, y: 1 })), // 2
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_min_equal() {
        let nodes = vec![
            ResolvedNode::Value(4.0),                              // 0 = x
            ResolvedNode::Value(4.0),                              // 1 = y
            ResolvedNode::OpCode(OpCode::Min(Min { x: 0, y: 1 })), // 2
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_max_left_greater() {
        let nodes = vec![
            ResolvedNode::Value(8.0),                              // 0 = x
            ResolvedNode::Value(6.0),                              // 1 = y
            ResolvedNode::OpCode(OpCode::Max(Max { x: 0, y: 1 })), // 2
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_max_right_greater() {
        let nodes = vec![
            ResolvedNode::Value(1.0),                              // 0 = x
            ResolvedNode::Value(9.0),                              // 1 = y
            ResolvedNode::OpCode(OpCode::Max(Max { x: 0, y: 1 })), // 2
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 9.0);
    }

    #[test]
    fn test_max_equal() {
        let nodes = vec![
            ResolvedNode::Value(7.0),                              // 0 = x
            ResolvedNode::Value(7.0),                              // 1 = y
            ResolvedNode::OpCode(OpCode::Max(Max { x: 0, y: 1 })), // 2
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_remap_basic() {
        let nodes = vec![
            ResolvedNode::Value(0.0),   // 0 = from_min
            ResolvedNode::Value(10.0),  // 1 = from_max
            ResolvedNode::Value(0.0),   // 2 = to_min
            ResolvedNode::Value(100.0), // 3 = to_max
            ResolvedNode::Value(5.0),   // 4 = value
            ResolvedNode::OpCode(OpCode::Remap(Remap {
                from_min: 0,
                from_max: 1,
                to_min: 2,
                to_max: 3,
                value: 4,
            })), // 5
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 50.0);
    }

    #[test]
    fn test_remap_value_below_min() {
        let nodes = vec![
            ResolvedNode::Value(0.0),   // 0 = from_min
            ResolvedNode::Value(10.0),  // 1 = from_max
            ResolvedNode::Value(0.0),   // 2 = to_min
            ResolvedNode::Value(100.0), // 3 = to_max
            ResolvedNode::Value(-5.0),  // 4 = value
            ResolvedNode::OpCode(OpCode::Remap(Remap {
                from_min: 0,
                from_max: 1,
                to_min: 2,
                to_max: 3,
                value: 4,
            })), // 5
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, -50.0);
    }

    #[test]
    fn test_remap_from_min_equals_max() {
        let nodes = vec![
            ResolvedNode::Value(1.0), // 0 = from_min
            ResolvedNode::Value(1.0), // 1 = from_max
            ResolvedNode::Value(2.0), // 2 = to_min
            ResolvedNode::Value(4.0), // 3 = to_max
            ResolvedNode::Value(1.0), // 4 = value
            ResolvedNode::OpCode(OpCode::Remap(Remap {
                from_min: 0,
                from_max: 1,
                to_min: 2,
                to_max: 3,
                value: 4,
            })), // 5
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        // Up to you how to handle div-by-zero, this test assumes you return 0.0
        assert!(result.is_nan())
    }

    #[test]
    fn test_remap_clamped_in_range() {
        let nodes = vec![
            ResolvedNode::Value(0.0),   // 0 = from_min
            ResolvedNode::Value(10.0),  // 1 = from_max
            ResolvedNode::Value(0.0),   // 2 = to_min
            ResolvedNode::Value(100.0), // 3 = to_max
            ResolvedNode::Value(5.0),   // 4 = value
            ResolvedNode::OpCode(OpCode::RemapClamped(RemapClamped {
                from_min: 0,
                from_max: 1,
                to_min: 2,
                to_max: 3,
                value: 4,
            })), // 5
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 50.0);
    }

    #[test]
    fn test_remap_clamped_below_min() {
        let nodes = vec![
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(10.0),
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(100.0),
            ResolvedNode::Value(-5.0),
            ResolvedNode::OpCode(OpCode::RemapClamped(RemapClamped {
                from_min: 0,
                from_max: 1,
                to_min: 2,
                to_max: 3,
                value: 4,
            })),
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_remap_clamped_above_max() {
        let nodes = vec![
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(10.0),
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(100.0),
            ResolvedNode::Value(20.0),
            ResolvedNode::OpCode(OpCode::RemapClamped(RemapClamped {
                from_min: 0,
                from_max: 1,
                to_min: 2,
                to_max: 3,
                value: 4,
            })),
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 100.0);
    }

    #[test]
    fn test_remap_clamped_min_equals_max() {
        let nodes = vec![
            ResolvedNode::Value(1.0),
            ResolvedNode::Value(1.0),
            ResolvedNode::Value(10.0),
            ResolvedNode::Value(20.0),
            ResolvedNode::Value(1.0),
            ResolvedNode::OpCode(OpCode::RemapClamped(RemapClamped {
                from_min: 0,
                from_max: 1,
                to_min: 2,
                to_max: 3,
                value: 4,
            })),
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_round_half_up_even() {
        let nodes = vec![
            ResolvedNode::Value(2.5),                                // 0
            ResolvedNode::OpCode(OpCode::Round(Round { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        assert_eq!(func(&mut runtime_context.as_ctx() as _), 2.0); // ties to even
    }

    #[test]
    fn test_round_half_up_odd() {
        let nodes = vec![
            ResolvedNode::Value(3.5),                                // 0
            ResolvedNode::OpCode(OpCode::Round(Round { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        assert_eq!(func(&mut runtime_context.as_ctx() as _), 4.0);
    }

    #[test]
    fn test_round_negative_half() {
        let nodes = vec![
            ResolvedNode::Value(-1.5),                               // 0
            ResolvedNode::OpCode(OpCode::Round(Round { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        assert_eq!(func(&mut runtime_context.as_ctx() as _), -2.0);
    }

    #[test]
    fn test_floor_positive() {
        let nodes = vec![
            ResolvedNode::Value(3.7),                                // 0
            ResolvedNode::OpCode(OpCode::Floor(Floor { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        assert_eq!(func(&mut runtime_context.as_ctx() as _), 3.0);
    }

    #[test]
    fn test_floor_negative() {
        let nodes = vec![
            ResolvedNode::Value(-1.2),                               // 0
            ResolvedNode::OpCode(OpCode::Floor(Floor { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        assert_eq!(func(&mut runtime_context.as_ctx() as _), -2.0);
    }

    #[test]
    fn test_ceil_positive() {
        let nodes = vec![
            ResolvedNode::Value(2.1),                              // 0
            ResolvedNode::OpCode(OpCode::Ceil(Ceil { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        assert_eq!(func(&mut runtime_context.as_ctx() as _), 3.0);
    }

    #[test]
    fn test_ceil_negative() {
        let nodes = vec![
            ResolvedNode::Value(-3.9),                             // 0
            ResolvedNode::OpCode(OpCode::Ceil(Ceil { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        assert_eq!(func(&mut runtime_context.as_ctx() as _), -3.0);
    }

    #[test]
    fn test_sin() {
        let pi = std::f32::consts::PI;
        let inputs = [0.0, pi / 2.0, pi, -pi / 2.0];
        let expected = [0.0, 1.0, 0.0, -1.0];

        for (&x, &y) in inputs.iter().zip(expected.iter()) {
            let nodes = vec![
                ResolvedNode::Value(x),
                ResolvedNode::OpCode(OpCode::Sin(Sin { value: 0 })),
            ];
            let mut runtime_context = BasicRuntimeContext::default();
            let func = build_and_return_function(&nodes, 1);
            let result = func(&mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "sin({x}) = {result}, expected {y}",
            );
        }
    }

    #[test]
    fn test_cos() {
        let pi = std::f32::consts::PI;
        let inputs = [0.0, pi / 2.0, pi, -pi / 2.0];
        let expected = [1.0, 0.0, -1.0, 0.0];

        for (&x, &y) in inputs.iter().zip(expected.iter()) {
            let nodes = vec![
                ResolvedNode::Value(x),
                ResolvedNode::OpCode(OpCode::Cos(Cos { value: 0 })),
            ];
            let mut runtime_context = BasicRuntimeContext::default();
            let func = build_and_return_function(&nodes, 1);
            let result = func(&mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "cos({x}) = {result}, expected {y}",
            );
        }
    }

    #[test]
    fn test_tan() {
        let pi = std::f32::consts::PI;
        let inputs = [0.0, pi / 4.0, -pi / 4.0];
        let expected = [0.0, 1.0, -1.0];

        for (&x, &y) in inputs.iter().zip(expected.iter()) {
            let nodes = vec![
                ResolvedNode::Value(x),
                ResolvedNode::OpCode(OpCode::Tan(Tan { value: 0 })),
            ];
            let mut runtime_context = BasicRuntimeContext::default();
            let func = build_and_return_function(&nodes, 1);
            let result = func(&mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "tan({x}) = {result}, expected {y}",
            );
        }
    }

    #[test]
    fn test_sinh() {
        let inputs = [0.0, 1.0, -1.0];
        #[allow(clippy::excessive_precision)]
        let expected = [0.0, 1.1752011936, -1.1752011936];

        for (&x, &y) in inputs.iter().zip(expected.iter()) {
            let nodes = vec![
                ResolvedNode::Value(x),
                ResolvedNode::OpCode(OpCode::Sinh(Sinh { value: 0 })),
            ];
            let mut runtime_context = BasicRuntimeContext::default();
            let func = build_and_return_function(&nodes, 1);
            let result = func(&mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "sinh({x}) = {result}, expected {y}",
            );
        }
    }

    #[test]
    fn test_cosh() {
        let inputs = [0.0, 1.0, -1.0];
        #[allow(clippy::excessive_precision)]
        let expected = [1.0, 1.5430806348, 1.5430806348];

        for (&x, &y) in inputs.iter().zip(expected.iter()) {
            let nodes = vec![
                ResolvedNode::Value(x),
                ResolvedNode::OpCode(OpCode::Cosh(Cosh { value: 0 })),
            ];
            let mut runtime_context = BasicRuntimeContext::default();
            let func = build_and_return_function(&nodes, 1);
            let result = func(&mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "cosh({x}) = {result}, expected {y}",
            );
        }
    }

    #[test]
    fn test_tanh() {
        let inputs = [0.0, 1.0, -1.0];
        #[allow(clippy::excessive_precision)]
        let expected = [0.0, 0.7615941559, -0.7615941559];

        for (&x, &y) in inputs.iter().zip(expected.iter()) {
            let nodes = vec![
                ResolvedNode::Value(x),
                ResolvedNode::OpCode(OpCode::Tanh(Tanh { value: 0 })),
            ];
            let mut runtime_context = BasicRuntimeContext::default();
            let func = build_and_return_function(&nodes, 1);
            let result = func(&mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "tanh({x}) = {result}, expected {y}",
            );
        }
    }

    #[test]
    fn test_asin() {
        let inputs = [-1.0, -0.5, 0.0, 0.5, 1.0];
        let expected = [
            -std::f32::consts::FRAC_PI_2,
            -std::f32::consts::FRAC_PI_6,
            0.0,
            std::f32::consts::FRAC_PI_6,
            std::f32::consts::FRAC_PI_2,
        ];

        for (&x, &y) in inputs.iter().zip(expected.iter()) {
            let nodes = vec![
                ResolvedNode::Value(x),
                ResolvedNode::OpCode(OpCode::Arcsin(Arcsin { value: 0 })),
            ];
            let mut runtime_context = BasicRuntimeContext::default();
            let func = build_and_return_function(&nodes, 1);
            let result = func(&mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "asin({x}) = {result}, expected {y}",
            );
        }
    }

    #[test]
    fn test_acos() {
        let inputs = [-1.0, -0.5, 0.0, 0.5, 1.0];
        let expected = [
            std::f32::consts::PI,
            #[allow(clippy::excessive_precision)]
            2.0943951024, // 2π/3
            std::f32::consts::FRAC_PI_2,
            std::f32::consts::FRAC_PI_3,
            0.0,
        ];

        for (&x, &y) in inputs.iter().zip(expected.iter()) {
            let nodes = vec![
                ResolvedNode::Value(x),
                ResolvedNode::OpCode(OpCode::Arccos(Arccos { value: 0 })),
            ];
            let mut runtime_context = BasicRuntimeContext::default();
            let func = build_and_return_function(&nodes, 1);
            let result = func(&mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "acos({x}) = {result}, expected {y}",
            );
        }
    }

    #[test]
    fn test_atan() {
        let inputs = [-1.0, -0.5, 0.0, 0.5, 1.0];
        let expected = [
            -std::f32::consts::FRAC_PI_4,
            #[allow(clippy::excessive_precision)]
            -0.4636476090,
            0.0,
            #[allow(clippy::excessive_precision)]
            0.4636476090,
            std::f32::consts::FRAC_PI_4,
        ];

        for (&x, &y) in inputs.iter().zip(expected.iter()) {
            let nodes = vec![
                ResolvedNode::Value(x),
                ResolvedNode::OpCode(OpCode::Arctan(Arctan { value: 0 })),
            ];
            let mut runtime_context = BasicRuntimeContext::default();
            let func = build_and_return_function(&nodes, 1);
            let result = func(&mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "atan({x}) = {result}, expected {y}",
            );
        }
    }

    #[test]
    fn test_atan2() {
        let pairs = [
            (0.0, 1.0),  // 0
            (1.0, 0.0),  // π/2
            (0.0, -1.0), // π
            (-1.0, 0.0), // -π/2
            (1.0, 1.0),  // π/4
        ];
        let expected = [
            0.0,
            std::f32::consts::FRAC_PI_2,
            std::f32::consts::PI,
            -std::f32::consts::FRAC_PI_2,
            std::f32::consts::FRAC_PI_4,
        ];

        for ((y, x), expected) in pairs.iter().zip(expected.iter()) {
            let nodes = vec![
                ResolvedNode::Value(*y),
                ResolvedNode::Value(*x),
                ResolvedNode::OpCode(OpCode::Arctan2(Arctan2 { y: 0, x: 1 })),
            ];
            let mut runtime_context = BasicRuntimeContext::default();
            let func = build_and_return_function(&nodes, 2);
            let result = func(&mut runtime_context.as_ctx() as _);
            assert!(
                (result - expected).abs() < 1e-6,
                "atan2({y}, {x}) = {result}, expected {expected}",
            );
        }
    }

    #[test]
    fn test_degree_pi() {
        let nodes = vec![
            ResolvedNode::Value(std::f32::consts::PI), // 0
            ResolvedNode::OpCode(OpCode::Degree(Degree { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert!((result - 180.0).abs() < 1e-6);
    }

    #[test]
    fn test_degree_zero() {
        let nodes = vec![
            ResolvedNode::Value(0.0),                                  // 0
            ResolvedNode::OpCode(OpCode::Degree(Degree { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_degree_negative() {
        let nodes = vec![
            ResolvedNode::Value(-std::f32::consts::PI / 2.0), // 0
            ResolvedNode::OpCode(OpCode::Degree(Degree { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert!((result + 90.0).abs() < 1e-6);
    }

    #[test]
    fn test_radian_180() {
        let nodes = vec![
            ResolvedNode::Value(180.0),                                // 0
            ResolvedNode::OpCode(OpCode::Radian(Radian { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert!((result - std::f32::consts::PI).abs() < 1e-6);
    }

    #[test]
    fn test_radian_zero() {
        let nodes = vec![
            ResolvedNode::Value(0.0),                                  // 0
            ResolvedNode::OpCode(OpCode::Radian(Radian { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_radian_negative() {
        let nodes = vec![
            ResolvedNode::Value(-90.0),                                // 0
            ResolvedNode::OpCode(OpCode::Radian(Radian { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert!((result + std::f32::consts::FRAC_PI_2).abs() < 1e-6);
    }

    #[test]
    fn test_log_regular_value() {
        let nodes = vec![
            ResolvedNode::Value(10.0),                           // 0 = value
            ResolvedNode::OpCode(OpCode::Log(Log { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert!((result - 10f32.ln()).abs() < 1e-6);
    }

    #[test]
    fn test_log_of_1() {
        let nodes = vec![
            ResolvedNode::Value(1.0),                            // 0 = value
            ResolvedNode::OpCode(OpCode::Log(Log { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_log_of_e() {
        let nodes = vec![
            ResolvedNode::Value(std::f32::consts::E), // 0 = value
            ResolvedNode::OpCode(OpCode::Log(Log { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert!((result - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_log_of_zero() {
        let nodes = vec![
            ResolvedNode::Value(0.0),                            // 0 = value
            ResolvedNode::OpCode(OpCode::Log(Log { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert!(result.is_infinite() && result.is_sign_negative());
    }

    #[test]
    fn test_log_of_negative() {
        let nodes = vec![
            ResolvedNode::Value(-1.0),                           // 0 = value
            ResolvedNode::OpCode(OpCode::Log(Log { value: 0 })), // 1
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert!(result.is_nan());
    }

    #[test]
    fn test_sign_positive() {
        let nodes = vec![
            ResolvedNode::Value(42.0),                             // 0
            ResolvedNode::OpCode(OpCode::Sign(Sign { value: 0 })), // 1
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_sign_negative() {
        let nodes = vec![
            ResolvedNode::Value(-std::f32::consts::PI),            // 0
            ResolvedNode::OpCode(OpCode::Sign(Sign { value: 0 })), // 1
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, -1.0);
    }

    #[test]
    fn test_sign_zero() {
        let nodes = vec![
            ResolvedNode::Value(0.0),                              // 0
            ResolvedNode::OpCode(OpCode::Sign(Sign { value: 0 })), // 1
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sign_nan() {
        let nodes = vec![
            ResolvedNode::Value(f32::NAN),                         // 0
            ResolvedNode::OpCode(OpCode::Sign(Sign { value: 0 })), // 1
        ];
        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context.as_ctx() as _);
        assert!(result.is_nan());
    }

    #[test]
    fn test_random_range_and_variance() {
        let nodes = vec![
            ResolvedNode::Value(5.0),                                        // min
            ResolvedNode::Value(10.0),                                       // max
            ResolvedNode::OpCode(OpCode::Random(Random { min: 0, max: 1 })), // random node
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);

        let mut results = std::collections::HashSet::new();
        for _ in 0..10 {
            let result = func(&mut runtime_context.as_ctx() as _);
            assert!(
                (5.0..=10.0).contains(&result),
                "Random result {result} out of range [5.0, 10.0]",
            );
            results.insert(result.to_bits()); // use bit pattern to avoid NaN weirdness
        }

        assert!(
            results.len() > 1,
            "All random results were the same: {results:?}",
        );
    }

    #[test]
    fn test_random_integer_range_and_integral() {
        let nodes = vec![
            ResolvedNode::Value(1.0), // min inclusive
            ResolvedNode::Value(5.0), // max exclusive
            ResolvedNode::OpCode(OpCode::RandomInteger(RandomInteger { min: 0, max: 1 })), // random int node
        ];

        let mut runtime_context = BasicRuntimeContext::default();
        let func = build_and_return_function(&nodes, 2);

        let mut results = std::collections::HashSet::new();
        for _ in 0..10 {
            let result = func(&mut runtime_context.as_ctx() as _);
            assert!(
                (1.0..5.0).contains(&result),
                "RandomInteger result {result} out of range [1.0, 5.0)",
            );
            assert_eq!(
                result.fract(),
                0.0,
                "RandomInteger result {result} is not an integer",
            );
            results.insert(result.to_bits());
        }

        assert!(
            results.len() > 1,
            "All RandomInteger results were the same: {results:?}",
        );
    }
}
