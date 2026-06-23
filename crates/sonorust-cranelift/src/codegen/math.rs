use cranelift::codegen::ir::BlockArg;
use cranelift::prelude::*;

use crate::codegen::CodegenContext;
use sonorust_ir::nodes::*;

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
        let final_result = self
            .builder
            .append_block_param(block_result, crate::IR_VALUE_CRANELIFT_TYPE);

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
        let one = crate::ir_value_cranelift_const(self.builder.ins(), 1.0);
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
        let block_join_param = self
            .builder
            .append_block_param(block_join, crate::IR_VALUE_CRANELIFT_TYPE);

        let zero = crate::ir_value_cranelift_const(self.builder.ins(), 0.0);
        let one = crate::ir_value_cranelift_const(self.builder.ins(), 1.0);

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
        let mut result = crate::ir_value_cranelift_const(self.builder.ins(), 0.0);
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
        let mut result = crate::ir_value_cranelift_const(self.builder.ins(), 1.0);
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
        if node.inputs.is_empty() {
            return crate::ir_value_cranelift_const(self.builder.ins(), 0.0);
        }

        let values: Vec<Value> = node
            .inputs
            .iter()
            .map(|&input| self.build_node_ir(input))
            .collect();

        let mut iter = values.into_iter();
        let first = iter.next().unwrap();

        iter.fold(first, |acc, exponent| self.build_power(acc, exponent))
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
        let zero = crate::ir_value_cranelift_const(self.builder.ins(), 0.0);
        let one = crate::ir_value_cranelift_const(self.builder.ins(), 1.0);
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

        let zero = crate::ir_value_cranelift_const(self.builder.ins(), 0.0);
        let one = crate::ir_value_cranelift_const(self.builder.ins(), 1.0);
        let neg_one = crate::ir_value_cranelift_const(self.builder.ins(), -1.0);
        let nan_val = crate::ir_value_cranelift_const(self.builder.ins(), f32::NAN);

        let gt_zero = self.builder.ins().fcmp(FloatCC::GreaterThan, value, zero);
        let lt_zero = self.builder.ins().fcmp(FloatCC::LessThan, value, zero);

        let sign_or_zero = self.builder.ins().select(lt_zero, neg_one, zero);
        let normal_result = self.builder.ins().select(gt_zero, one, sign_or_zero);

        let is_nan = self.builder.ins().fcmp(FloatCC::NotEqual, value, value);
        self.builder.ins().select(is_nan, nan_val, normal_result)
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
