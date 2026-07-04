use crate::codegen::CodegenContext;

use cranelift::prelude::*;
use sonorust_ir::{IRIndex, nodes::*};

impl<'s, 'b> CodegenContext<'s, 'b> {
    fn build_read_mem(&mut self, block_id: Value, index: Value) -> Value {
        let fn_ref = self.externals_func_refs["read_mem"];
        let fn_call = self
            .builder
            .ins()
            .call(fn_ref, &[self.ctx_param, block_id, index]);
        self.builder.inst_results(fn_call)[0]
    }

    fn build_write_mem(&mut self, block_id: Value, index: Value, value: Value) -> Value {
        let fn_ref = self.externals_func_refs["write_mem"];
        let fn_call = self
            .builder
            .ins()
            .call(fn_ref, &[self.ctx_param, block_id, index, value]);
        self.builder.inst_results(fn_call)[0]
    }

    fn build_pointed_addr(
        &mut self,
        block_id_node: IRIndex,
        index_node: IRIndex,
        offset_node: IRIndex,
    ) -> (Value, Value) {
        let block_id_float = self.build_node_ir(block_id_node);
        let block_id = self.builder.ins().fcvt_to_sint(types::I64, block_id_float);
        let index_float = self.build_node_ir(index_node);
        let index = self.builder.ins().fcvt_to_sint(types::I64, index_float);
        let offset_float = self.build_node_ir(offset_node);
        let offset = self.builder.ins().fcvt_to_sint(types::I64, offset_float);

        // pointed_block_id = Get(block_id, index)
        let pointed_block_id_float = self.build_read_mem(block_id, index);
        let pointed_block_id = self
            .builder
            .ins()
            .fcvt_to_sint(types::I64, pointed_block_id_float);

        // pointed_index = Get(block_id, index + 1)
        let one = self.builder.ins().iconst(types::I64, 1);
        let index_plus_one = self.builder.ins().iadd(index, one);
        let pointed_base_float = self.build_read_mem(block_id, index_plus_one);
        let pointed_base = self
            .builder
            .ins()
            .fcvt_to_sint(types::I64, pointed_base_float);

        // final_index = pointed_index + offset
        let final_index = self.builder.ins().iadd(pointed_base, offset);

        (pointed_block_id, final_index)
    }

    fn build_shifted_addr(
        &mut self,
        block_id_node: IRIndex,
        x_node: IRIndex,
        y_node: IRIndex,
        s_node: IRIndex,
    ) -> (Value, Value) {
        let block_id_float = self.build_node_ir(block_id_node);
        let block_id = self.builder.ins().fcvt_to_sint(types::I64, block_id_float);

        let x_float = self.build_node_ir(x_node);
        let x = self.builder.ins().fcvt_to_sint(types::I64, x_float);

        let y_float = self.build_node_ir(y_node);
        let y = self.builder.ins().fcvt_to_sint(types::I64, y_float);

        let s_float = self.build_node_ir(s_node);
        let s = self.builder.ins().fcvt_to_sint(types::I64, s_float);

        let y_mul_s = self.builder.ins().imul(y, s);
        let shifted_index = self.builder.ins().iadd(x, y_mul_s);

        (block_id, shifted_index)
    }

    fn build_set_like_op<F>(
        &mut self,
        read: impl Fn(&mut Self) -> (Value, Value),
        value_node: IRIndex,
        op: F,
    ) -> Value
    where
        F: Fn(&mut Self, Value, Value) -> Value,
    {
        let value = self.build_node_ir(value_node);
        let (block_id, index) = read(self);
        let old_value = self.build_read_mem(block_id, index);
        let new_value = op(self, old_value, value);
        self.build_write_mem(block_id, index, new_value)
    }

    fn build_set_op_ir<F>(
        &mut self,
        block_id: IRIndex,
        index: IRIndex,
        value: IRIndex,
        op: F,
    ) -> Value
    where
        F: FnOnce(&mut Self, Value, Value) -> Value,
    {
        let block_id_float = self.build_node_ir(block_id);
        let block_id = self.builder.ins().fcvt_to_sint(types::I64, block_id_float);

        let index_float = self.build_node_ir(index);
        let index = self.builder.ins().fcvt_to_sint(types::I64, index_float);

        let val = self.build_node_ir(value);
        let current = self.build_read_mem(block_id, index);
        let result = op(self, current, val);

        self.build_write_mem(block_id, index, result)
    }

    pub(crate) fn build_get_ir(&mut self, node: &Get) -> Value {
        let block_id_float = self.build_node_ir(node.block_id);
        let block_id = self.builder.ins().fcvt_to_sint(types::I64, block_id_float);
        let index_float = self.build_node_ir(node.index);
        let index = self.builder.ins().fcvt_to_sint(types::I64, index_float);
        self.build_read_mem(block_id, index)
    }

    pub(crate) fn build_get_pointed_ir(&mut self, node: &GetPointed) -> Value {
        let (final_block_id, final_index) =
            self.build_pointed_addr(node.block_id, node.index, node.offset);
        self.build_read_mem(final_block_id, final_index)
    }

    pub(crate) fn build_get_shifted_ir(&mut self, node: &GetShifted) -> Value {
        let (final_block_id, final_index) =
            self.build_shifted_addr(node.block_id, node.x, node.y, node.s);

        self.build_read_mem(final_block_id, final_index)
    }

    pub(crate) fn build_set_ir(&mut self, node: &Set) -> Value {
        let block_id_float = self.build_node_ir(node.block_id);
        let block_id = self.builder.ins().fcvt_to_sint(types::I64, block_id_float);
        let index_float = self.build_node_ir(node.index);
        let index = self.builder.ins().fcvt_to_sint(types::I64, index_float);
        let value = self.build_node_ir(node.value);
        self.build_write_mem(block_id, index, value)
    }

    pub(crate) fn build_set_pointed_ir(&mut self, node: &SetPointed) -> Value {
        let (final_block_id, final_index) =
            self.build_pointed_addr(node.block_id, node.index, node.offset);
        let value = self.build_node_ir(node.value);
        self.build_write_mem(final_block_id, final_index, value)
    }

    pub(crate) fn build_set_shifted_ir(&mut self, node: &SetShifted) -> Value {
        let (final_block_id, final_index) =
            self.build_shifted_addr(node.block_id, node.x, node.y, node.s);
        let value = self.build_node_ir(node.value);
        self.build_write_mem(final_block_id, final_index, value)
    }

    pub(crate) fn build_set_add_ir(&mut self, node: &SetAdd) -> Value {
        self.build_set_op_ir(node.block_id, node.index, node.value, |s, a, b| {
            s.builder.ins().fadd(a, b)
        })
    }

    pub(crate) fn build_set_add_pointed_ir(&mut self, node: &SetAddPointed) -> Value {
        self.build_set_like_op(
            |s| s.build_pointed_addr(node.block_id, node.index, node.offset),
            node.value,
            |s, old, val| s.builder.ins().fadd(old, val),
        )
    }

    pub(crate) fn build_set_add_shifted_ir(&mut self, node: &SetAddShifted) -> Value {
        self.build_set_like_op(
            |s| s.build_shifted_addr(node.block_id, node.x, node.y, node.s),
            node.value,
            |s, old, val| s.builder.ins().fadd(old, val),
        )
    }

    pub(crate) fn build_set_subtract_ir(&mut self, node: &SetSubtract) -> Value {
        self.build_set_op_ir(node.block_id, node.index, node.value, |s, a, b| {
            s.builder.ins().fsub(a, b)
        })
    }

    pub(crate) fn build_set_subtract_pointed_ir(&mut self, node: &SetSubtractPointed) -> Value {
        self.build_set_like_op(
            |s| s.build_pointed_addr(node.block_id, node.index, node.offset),
            node.value,
            |s, old, val| s.builder.ins().fsub(old, val),
        )
    }

    pub(crate) fn build_set_subtract_shifted_ir(&mut self, node: &SetSubtractShifted) -> Value {
        self.build_set_like_op(
            |s| s.build_shifted_addr(node.block_id, node.x, node.y, node.s),
            node.value,
            |s, old, val| s.builder.ins().fsub(old, val),
        )
    }

    pub(crate) fn build_set_multiply_ir(&mut self, node: &SetMultiply) -> Value {
        self.build_set_op_ir(node.block_id, node.index, node.value, |s, a, b| {
            s.builder.ins().fmul(a, b)
        })
    }

    pub(crate) fn build_set_multiply_pointed_ir(&mut self, node: &SetMultiplyPointed) -> Value {
        self.build_set_like_op(
            |s| s.build_pointed_addr(node.block_id, node.index, node.offset),
            node.value,
            |s, old, val| s.builder.ins().fmul(old, val),
        )
    }

    pub(crate) fn build_set_multiply_shifted_ir(&mut self, node: &SetMultiplyShifted) -> Value {
        self.build_set_like_op(
            |s| s.build_shifted_addr(node.block_id, node.x, node.y, node.s),
            node.value,
            |s, old, val| s.builder.ins().fmul(old, val),
        )
    }

    pub(crate) fn build_set_divide_ir(&mut self, node: &SetDivide) -> Value {
        self.build_set_op_ir(node.block_id, node.index, node.value, |s, a, b| {
            s.builder.ins().fdiv(a, b)
        })
    }

    pub(crate) fn build_set_divide_pointed_ir(&mut self, node: &SetDividePointed) -> Value {
        self.build_set_like_op(
            |s| s.build_pointed_addr(node.block_id, node.index, node.offset),
            node.value,
            |s, old, val| s.builder.ins().fdiv(old, val),
        )
    }

    pub(crate) fn build_set_divide_shifted_ir(&mut self, node: &SetDivideShifted) -> Value {
        self.build_set_like_op(
            |s| s.build_shifted_addr(node.block_id, node.x, node.y, node.s),
            node.value,
            |s, old, val| s.builder.ins().fdiv(old, val),
        )
    }

    pub(crate) fn build_set_power_ir(&mut self, node: &SetPower) -> Value {
        self.build_set_like_op(
            |s| {
                let block_id_float = s.build_node_ir(node.block_id);
                let block_id = s.builder.ins().fcvt_to_sint(types::I64, block_id_float);
                let index_float = s.build_node_ir(node.index);
                let index = s.builder.ins().fcvt_to_sint(types::I64, index_float);
                (block_id, index)
            },
            node.value,
            |s, old_value, value| s.build_power(old_value, value),
        )
    }

    pub(crate) fn build_set_power_pointed_ir(&mut self, node: &SetPowerPointed) -> Value {
        self.build_set_like_op(
            |s| s.build_pointed_addr(node.block_id, node.index, node.offset),
            node.value,
            |s, old_value, value| s.build_power(old_value, value),
        )
    }

    pub(crate) fn build_set_power_shifted_ir(&mut self, node: &SetPowerShifted) -> Value {
        self.build_set_like_op(
            |s| s.build_shifted_addr(node.block_id, node.x, node.y, node.s),
            node.value,
            |s, old_value, value| s.build_power(old_value, value),
        )
    }

    pub(crate) fn build_set_rem_ir(&mut self, node: &SetRem) -> Value {
        self.build_set_like_op(
            |s| {
                let block_id_float = s.build_node_ir(node.block_id);
                let block_id = s.builder.ins().fcvt_to_sint(types::I64, block_id_float);
                let index_float = s.build_node_ir(node.index);
                let index = s.builder.ins().fcvt_to_sint(types::I64, index_float);
                (block_id, index)
            },
            node.value,
            |s, old_value, value| s.build_rem(old_value, value),
        )
    }

    pub(crate) fn build_set_rem_pointed_ir(&mut self, node: &SetRemPointed) -> Value {
        self.build_set_like_op(
            |s| s.build_pointed_addr(node.block_id, node.index, node.offset),
            node.value,
            |s, old_value, value| s.build_rem(old_value, value),
        )
    }

    pub(crate) fn build_set_rem_shifted_ir(&mut self, node: &SetRemShifted) -> Value {
        self.build_set_like_op(
            |s| s.build_shifted_addr(node.block_id, node.x, node.y, node.s),
            node.value,
            |s, old_value, value| s.build_rem(old_value, value),
        )
    }

    pub(crate) fn build_set_mod_ir(&mut self, node: &SetMod) -> Value {
        self.build_set_like_op(
            |s| {
                let block_id_float = s.build_node_ir(node.block_id);
                let block_id = s.builder.ins().fcvt_to_sint(types::I64, block_id_float);
                let index_float = s.build_node_ir(node.index);
                let index = s.builder.ins().fcvt_to_sint(types::I64, index_float);
                (block_id, index)
            },
            node.value,
            |s, old_value, value| s.build_mod(old_value, value),
        )
    }

    pub(crate) fn build_set_mod_pointed_ir(&mut self, node: &SetModPointed) -> Value {
        self.build_set_like_op(
            |s| s.build_pointed_addr(node.block_id, node.index, node.offset),
            node.value,
            |s, old_value, value| s.build_mod(old_value, value),
        )
    }

    pub(crate) fn build_set_mod_shifted_ir(&mut self, node: &SetModShifted) -> Value {
        self.build_set_like_op(
            |s| s.build_shifted_addr(node.block_id, node.x, node.y, node.s),
            node.value,
            |s, old_value, value| s.build_mod(old_value, value),
        )
    }

    pub(crate) fn build_copy_ir(&mut self, node: &Copy) -> Value {
        let src_block_id_float = self.build_node_ir(node.src_block_id);
        let src_block_id = self
            .builder
            .ins()
            .fcvt_to_sint(types::I64, src_block_id_float);
        let src_index_float = self.build_node_ir(node.src_index);
        let src_index = self.builder.ins().fcvt_to_sint(types::I64, src_index_float);
        let dst_block_id_float = self.build_node_ir(node.dst_block_id);
        let dst_block_id = self
            .builder
            .ins()
            .fcvt_to_sint(types::I64, dst_block_id_float);
        let dst_index_float = self.build_node_ir(node.dst_index);
        let dst_index = self.builder.ins().fcvt_to_sint(types::I64, dst_index_float);
        let count_float = self.build_node_ir(node.count);
        let count = self.builder.ins().fcvt_to_sint(types::I64, count_float);

        let fn_ref = self.externals_func_refs["copy_mem"];
        let fn_call = self.builder.ins().call(
            fn_ref,
            &[
                self.ctx_param,
                src_block_id,
                src_index,
                dst_block_id,
                dst_index,
                count,
            ],
        );
        self.builder.inst_results(fn_call)[0]
    }
}
