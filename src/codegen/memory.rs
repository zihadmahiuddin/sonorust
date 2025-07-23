use crate::{codegen::CodegenContext, nodes::*};

use cranelift::prelude::*;

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
        block_id_node: usize,
        index_node: usize,
        offset_node: usize,
    ) -> (Value, Value) {
        let block_id_f32 = self.build_node_ir(block_id_node);
        let block_id = self.builder.ins().fcvt_to_sint(types::I64, block_id_f32);
        let index_f32 = self.build_node_ir(index_node);
        let index = self.builder.ins().fcvt_to_sint(types::I64, index_f32);
        let offset_f32 = self.build_node_ir(offset_node);
        let offset = self.builder.ins().fcvt_to_sint(types::I64, offset_f32);

        // pointed_block_id = Get(block_id, index)
        let pointed_block_id_f32 = self.build_read_mem(block_id, index);
        let pointed_block_id = self
            .builder
            .ins()
            .fcvt_to_sint(types::I64, pointed_block_id_f32);

        // pointed_index = Get(block_id, index + 1)
        let one = self.builder.ins().iconst(types::I64, 1);
        let index_plus_one = self.builder.ins().iadd(index, one);
        let pointed_base_f32 = self.build_read_mem(block_id, index_plus_one);
        let pointed_base = self
            .builder
            .ins()
            .fcvt_to_sint(types::I64, pointed_base_f32);

        // final_index = pointed_index + offset
        let final_index = self.builder.ins().iadd(pointed_base, offset);

        (pointed_block_id, final_index)
    }

    fn build_shifted_addr(
        &mut self,
        block_id_node: usize,
        x_node: usize,
        y_node: usize,
        s_node: usize,
    ) -> (Value, Value) {
        let block_id_f32 = self.build_node_ir(block_id_node);
        let block_id = self.builder.ins().fcvt_to_sint(types::I64, block_id_f32);

        let x_f32 = self.build_node_ir(x_node);
        let x = self.builder.ins().fcvt_to_sint(types::I64, x_f32);

        let y_f32 = self.build_node_ir(y_node);
        let y = self.builder.ins().fcvt_to_sint(types::I64, y_f32);

        let s_f32 = self.build_node_ir(s_node);
        let s = self.builder.ins().fcvt_to_sint(types::I64, s_f32);

        let y_mul_s = self.builder.ins().imul(y, s);
        let shifted_index = self.builder.ins().iadd(x, y_mul_s);

        (block_id, shifted_index)
    }

    fn build_set_like_op<F>(
        &mut self,
        read: impl Fn(&mut Self) -> (Value, Value),
        value_node: usize,
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

    fn build_set_op_ir<F>(&mut self, block_id: usize, index: usize, value: usize, op: F) -> Value
    where
        F: FnOnce(&mut Self, Value, Value) -> Value,
    {
        let block_id_f32 = self.build_node_ir(block_id);
        let block_id = self.builder.ins().fcvt_to_sint(types::I64, block_id_f32);

        let index_f32 = self.build_node_ir(index);
        let index = self.builder.ins().fcvt_to_sint(types::I64, index_f32);

        let val = self.build_node_ir(value);
        let current = self.build_read_mem(block_id, index);
        let result = op(self, current, val);

        self.build_write_mem(block_id, index, result)
    }

    pub(crate) fn build_get_ir(&mut self, node: &Get) -> Value {
        let block_id_f32 = self.build_node_ir(node.block_id);
        let block_id = self.builder.ins().fcvt_to_sint(types::I64, block_id_f32);
        let index_f32 = self.build_node_ir(node.index);
        let index = self.builder.ins().fcvt_to_sint(types::I64, index_f32);
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
        let block_id_f32 = self.build_node_ir(node.block_id);
        let block_id = self.builder.ins().fcvt_to_sint(types::I64, block_id_f32);
        let index_f32 = self.build_node_ir(node.index);
        let index = self.builder.ins().fcvt_to_sint(types::I64, index_f32);
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
                let block_id_f32 = s.build_node_ir(node.block_id);
                let block_id = s.builder.ins().fcvt_to_sint(types::I64, block_id_f32);
                let index_f32 = s.build_node_ir(node.index);
                let index = s.builder.ins().fcvt_to_sint(types::I64, index_f32);
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
                let block_id_f32 = s.build_node_ir(node.block_id);
                let block_id = s.builder.ins().fcvt_to_sint(types::I64, block_id_f32);
                let index_f32 = s.build_node_ir(node.index);
                let index = s.builder.ins().fcvt_to_sint(types::I64, index_f32);
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
                let block_id_f32 = s.build_node_ir(node.block_id);
                let block_id = s.builder.ins().fcvt_to_sint(types::I64, block_id_f32);
                let index_f32 = s.build_node_ir(node.index);
                let index = s.builder.ins().fcvt_to_sint(types::I64, index_f32);
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
        let src_block_id_f32 = self.build_node_ir(node.src_block_id);
        let src_block_id = self
            .builder
            .ins()
            .fcvt_to_sint(types::I64, src_block_id_f32);
        let src_index_f32 = self.build_node_ir(node.src_index);
        let src_index = self.builder.ins().fcvt_to_sint(types::I64, src_index_f32);
        let dst_block_id_f32 = self.build_node_ir(node.dst_block_id);
        let dst_block_id = self
            .builder
            .ins()
            .fcvt_to_sint(types::I64, dst_block_id_f32);
        let dst_index_f32 = self.build_node_ir(node.dst_index);
        let dst_index = self.builder.ins().fcvt_to_sint(types::I64, dst_index_f32);
        let count_f32 = self.build_node_ir(node.count);
        let count = self.builder.ins().fcvt_to_sint(types::I64, count_f32);

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

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{
        codegen::jit::build_and_return_function,
        nodes::*,
        runtime::{
            basic::BasicMemory,
            context::{MemoryAccess, RuntimeContext},
        },
    };

    #[test]
    fn test_get() {
        let nodes = vec![
            ResolvedNode::Value(0.0),
            ResolvedNode::OpCode(OpCode::Get(Get {
                block_id: 0,
                index: 0,
            })),
        ];

        let memory = BasicMemory::default();
        let mut runtime_context = RuntimeContext { memory: &memory };
        memory.write(0, 0, 7.0);
        let func = build_and_return_function(&nodes, 1);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_get_pointed() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // 0: block_id
            ResolvedNode::Value(0.0), // 1: index
            ResolvedNode::Value(2.0), // 2: offset
            ResolvedNode::OpCode(OpCode::GetPointed(GetPointed {
                block_id: 0,
                index: 1,
                offset: 2,
            })),
        ];

        let mut memory = BasicMemory::default();
        memory.writable.insert(3, RefCell::new(vec![0.0; 4096]));
        memory.write(0, 0, 3.0);
        memory.write(0, 1, 5.0);
        memory.write(3, 7, 123.45);

        let mut runtime_context = RuntimeContext { memory: &memory };

        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut runtime_context as _);

        assert_eq!(result, 123.45);
    }

    #[test]
    fn test_get_shifted() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // 0: block_id
            ResolvedNode::Value(2.0), // 1: x
            ResolvedNode::Value(3.0), // 2: y
            ResolvedNode::Value(4.0), // 3: s
            ResolvedNode::OpCode(OpCode::GetShifted(GetShifted {
                block_id: 0,
                x: 1,
                y: 2,
                s: 3,
            })),
        ];

        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![0.0; 4096]));
        memory.write(0, 2 + 3 * 4, 999.99); // index = 14

        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut runtime_context as _);

        assert_eq!(result, 999.99);
    }

    #[test]
    fn test_set() {
        let nodes = vec![
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(7.0),
            ResolvedNode::OpCode(OpCode::Set(Set {
                block_id: 0,
                index: 0,
                value: 1,
            })),
        ];

        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 2);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 7.0);
        assert_eq!(Some(result), runtime_context.memory.read(0, 0));
    }

    #[test]
    fn test_set_pointed() {
        let nodes = vec![
            ResolvedNode::Value(0.0),    // 0: block_id
            ResolvedNode::Value(0.0),    // 1: index
            ResolvedNode::Value(2.0),    // 2: offset
            ResolvedNode::Value(123.45), // 3: value
            ResolvedNode::OpCode(OpCode::SetPointed(SetPointed {
                block_id: 0,
                index: 1,
                offset: 2,
                value: 3,
            })),
        ];

        let mut memory = BasicMemory::default();
        memory.writable.insert(3, RefCell::new(vec![0.0; 4096]));
        memory.write(0, 0, 3.0);
        memory.write(0, 1, 5.0);

        let mut runtime_context = RuntimeContext { memory: &memory };

        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut runtime_context as _);

        assert_eq!(result, 123.45);
        assert_eq!(Some(result), runtime_context.memory.read(3, 7));
    }

    #[test]
    fn test_set_shifted() {
        let nodes = vec![
            ResolvedNode::Value(0.0),    // 0: block_id
            ResolvedNode::Value(2.0),    // 1: x
            ResolvedNode::Value(3.0),    // 2: y
            ResolvedNode::Value(4.0),    // 3: s
            ResolvedNode::Value(123.45), // 4: value
            ResolvedNode::OpCode(OpCode::SetShifted(SetShifted {
                block_id: 0,
                x: 1,
                y: 2,
                s: 3,
                value: 4,
            })),
        ];

        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![0.0; 4096]));

        let mut runtime_context = RuntimeContext { memory: &memory };
        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut runtime_context as _);

        let index = 2 + 3 * 4; // 14
        assert_eq!(result, 123.45);
        assert_eq!(Some(result), runtime_context.memory.read(0, index));
    }

    #[test]
    fn test_set_add() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(5.0), // index
            ResolvedNode::Value(3.0), // value
            ResolvedNode::OpCode(OpCode::SetAdd(SetAdd {
                block_id: 0,
                index: 1,
                value: 2,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![0.0; 4096]));
        memory.write(0, 5, 7.0);

        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 10.0);
        assert_eq!(Some(10.0), memory.read(0, 5));
    }

    #[test]
    fn test_set_add_pointed() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(0.0), // index
            ResolvedNode::Value(2.0), // offset
            ResolvedNode::Value(3.0), // value
            ResolvedNode::OpCode(OpCode::SetAddPointed(SetAddPointed {
                block_id: 0,
                index: 1,
                offset: 2,
                value: 3,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(3, RefCell::new(vec![0.0; 4096]));
        memory.write(0, 0, 3.0);
        memory.write(0, 1, 5.0);
        memory.write(3, 7, 7.0);

        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 10.0);
        assert_eq!(Some(10.0), memory.read(3, 7));
    }

    #[test]
    fn test_set_add_shifted() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(2.0), // x
            ResolvedNode::Value(3.0), // y
            ResolvedNode::Value(4.0), // s
            ResolvedNode::Value(5.0), // value
            ResolvedNode::OpCode(OpCode::SetAddShifted(SetAddShifted {
                block_id: 0,
                x: 1,
                y: 2,
                s: 3,
                value: 4,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![0.0; 4096]));
        memory.write(0, 14, 10.0); // 2 + 3*4 = 14

        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 15.0);
        assert_eq!(Some(15.0), memory.read(0, 14));
    }

    #[test]
    fn test_set_subtract() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(5.0), // index
            ResolvedNode::Value(3.0), // value (subtract)
            ResolvedNode::OpCode(OpCode::SetSubtract(SetSubtract {
                block_id: 0,
                index: 1,
                value: 2,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![10.0; 4096]));

        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 7.0);
        assert_eq!(Some(7.0), memory.read(0, 5));
    }

    #[test]
    fn test_set_subtract_pointed() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(0.0), // index
            ResolvedNode::Value(2.0), // offset
            ResolvedNode::Value(3.0), // value (subtract)
            ResolvedNode::OpCode(OpCode::SetSubtractPointed(SetSubtractPointed {
                block_id: 0,
                index: 1,
                offset: 2,
                value: 3,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(3, RefCell::new(vec![10.0; 4096]));
        memory.write(0, 0, 3.0);
        memory.write(0, 1, 5.0);
        memory.write(3, 7, 10.0);

        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 7.0);
        assert_eq!(Some(7.0), memory.read(3, 7));
    }

    #[test]
    fn test_set_subtract_shifted() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(2.0), // x
            ResolvedNode::Value(3.0), // y
            ResolvedNode::Value(4.0), // s
            ResolvedNode::Value(3.0), // value (subtract)
            ResolvedNode::OpCode(OpCode::SetSubtractShifted(SetSubtractShifted {
                block_id: 0,
                x: 1,
                y: 2,
                s: 3,
                value: 4,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![10.0; 4096]));
        memory.write(0, 14, 10.0); // 2 + 3*4 = 14

        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 7.0);
        assert_eq!(Some(7.0), memory.read(0, 14));
    }

    #[test]
    fn test_set_multiply() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(5.0), // index
            ResolvedNode::Value(2.0), // value (multiplier)
            ResolvedNode::OpCode(OpCode::SetMultiply(SetMultiply {
                block_id: 0,
                index: 1,
                value: 2,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![10.0; 4096]));

        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 20.0);
        assert_eq!(Some(20.0), memory.read(0, 5));
    }

    #[test]
    fn test_set_multiply_pointed() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(0.0), // index
            ResolvedNode::Value(2.0), // offset
            ResolvedNode::Value(2.0), // value (multiplier)
            ResolvedNode::OpCode(OpCode::SetMultiplyPointed(SetMultiplyPointed {
                block_id: 0,
                index: 1,
                offset: 2,
                value: 3,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(3, RefCell::new(vec![10.0; 4096]));
        memory.write(0, 0, 3.0);
        memory.write(0, 1, 5.0);
        memory.write(3, 7, 10.0);

        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 20.0);
        assert_eq!(Some(20.0), memory.read(3, 7));
    }

    #[test]
    fn test_set_multiply_shifted() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(2.0), // x
            ResolvedNode::Value(3.0), // y
            ResolvedNode::Value(4.0), // s
            ResolvedNode::Value(2.0), // value (multiplier)
            ResolvedNode::OpCode(OpCode::SetMultiplyShifted(SetMultiplyShifted {
                block_id: 0,
                x: 1,
                y: 2,
                s: 3,
                value: 4,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![10.0; 4096]));
        memory.write(0, 14, 10.0); // 2 + 3*4 = 14

        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 20.0);
        assert_eq!(Some(20.0), memory.read(0, 14));
    }

    #[test]
    fn test_set_divide() {
        let nodes = vec![
            ResolvedNode::Value(0.0),  // block_id
            ResolvedNode::Value(5.0),  // index
            ResolvedNode::Value(10.0), // value (divisor)
            ResolvedNode::OpCode(OpCode::SetDivide(SetDivide {
                block_id: 0,
                index: 1,
                value: 2,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![20.0; 4096]));

        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 2.0);
        assert_eq!(Some(2.0), memory.read(0, 5));
    }

    #[test]
    fn test_set_divide_pointed() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(0.0), // index
            ResolvedNode::Value(2.0), // offset
            ResolvedNode::Value(2.0), // value (divisor)
            ResolvedNode::OpCode(OpCode::SetDividePointed(SetDividePointed {
                block_id: 0,
                index: 1,
                offset: 2,
                value: 3,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(3, RefCell::new(vec![10.0; 4096]));
        memory.write(0, 0, 3.0);
        memory.write(0, 1, 5.0);
        memory.write(3, 7, 20.0);

        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 10.0);
        assert_eq!(Some(10.0), memory.read(3, 7));
    }

    #[test]
    fn test_set_divide_shifted() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(2.0), // x
            ResolvedNode::Value(3.0), // y
            ResolvedNode::Value(4.0), // s
            ResolvedNode::Value(2.0), // value (divisor)
            ResolvedNode::OpCode(OpCode::SetDivideShifted(SetDivideShifted {
                block_id: 0,
                x: 1,
                y: 2,
                s: 3,
                value: 4,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![40.0; 4096]));
        memory.write(0, 14, 40.0); // 2 + 3*4 = 14

        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 20.0);
        assert_eq!(Some(20.0), memory.read(0, 14));
    }

    #[test]
    fn test_set_power() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(5.0), // index
            ResolvedNode::Value(3.0), // value (exponent)
            ResolvedNode::OpCode(OpCode::SetPower(SetPower {
                block_id: 0,
                index: 1,
                value: 2,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![2.0; 4096]));

        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 8.0);
        assert_eq!(Some(8.0), memory.read(0, 5));
    }

    #[test]
    fn test_set_power_pointed() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(0.0), // index
            ResolvedNode::Value(2.0), // offset
            ResolvedNode::Value(3.0), // value (exponent)
            ResolvedNode::OpCode(OpCode::SetPowerPointed(SetPowerPointed {
                block_id: 0,
                index: 1,
                offset: 2,
                value: 3,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(3, RefCell::new(vec![2.0; 4096]));
        memory.write(0, 0, 3.0);
        memory.write(0, 1, 5.0);
        memory.write(3, 7, 2.0);

        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 8.0);
        assert_eq!(Some(8.0), memory.read(3, 7));
    }

    #[test]
    fn test_set_power_shifted() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(2.0), // x
            ResolvedNode::Value(3.0), // y
            ResolvedNode::Value(4.0), // s
            ResolvedNode::Value(3.0), // value (exponent)
            ResolvedNode::OpCode(OpCode::SetPowerShifted(SetPowerShifted {
                block_id: 0,
                x: 1,
                y: 2,
                s: 3,
                value: 4,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![2.0; 4096]));
        memory.write(0, 14, 2.0); // 2 + 3*4 = 14

        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 8.0);
        assert_eq!(Some(8.0), memory.read(0, 14));
    }

    #[test]
    fn test_set_rem() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(5.0), // index
            ResolvedNode::Value(3.0), // value (divisor)
            ResolvedNode::OpCode(OpCode::SetRem(SetRem {
                block_id: 0,
                index: 1,
                value: 2,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![10.0; 4096]));

        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 1.0);
        assert_eq!(Some(1.0), memory.read(0, 5));
    }

    #[test]
    fn test_set_rem_pointed() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(0.0), // index
            ResolvedNode::Value(2.0), // offset
            ResolvedNode::Value(3.0), // value (divisor)
            ResolvedNode::OpCode(OpCode::SetRemPointed(SetRemPointed {
                block_id: 0,
                index: 1,
                offset: 2,
                value: 3,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(3, RefCell::new(vec![10.0; 4096]));
        memory.write(0, 0, 3.0);
        memory.write(0, 1, 5.0);
        memory.write(3, 7, 10.0);

        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 1.0);
        assert_eq!(Some(1.0), memory.read(3, 7));
    }

    #[test]
    fn test_set_rem_shifted() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(2.0), // x
            ResolvedNode::Value(3.0), // y
            ResolvedNode::Value(4.0), // s
            ResolvedNode::Value(3.0), // value (divisor)
            ResolvedNode::OpCode(OpCode::SetRemShifted(SetRemShifted {
                block_id: 0,
                x: 1,
                y: 2,
                s: 3,
                value: 4,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![10.0; 4096]));
        memory.write(0, 14, 10.0); // 2 + 3*4 = 14

        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 1.0);
        assert_eq!(Some(1.0), memory.read(0, 14));
    }

    #[test]
    fn test_set_mod() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(5.0), // index
            ResolvedNode::Value(3.0), // value (mod divisor)
            ResolvedNode::OpCode(OpCode::SetMod(SetMod {
                block_id: 0,
                index: 1,
                value: 2,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![10.0; 4096]));

        let func = build_and_return_function(&nodes, 3);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 1.0);
        assert_eq!(Some(1.0), memory.read(0, 5));
    }

    #[test]
    fn test_set_mod_pointed() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(0.0), // index
            ResolvedNode::Value(2.0), // offset
            ResolvedNode::Value(3.0), // value (mod divisor)
            ResolvedNode::OpCode(OpCode::SetModPointed(SetModPointed {
                block_id: 0,
                index: 1,
                offset: 2,
                value: 3,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(3, RefCell::new(vec![10.0; 4096]));
        memory.write(0, 0, 3.0);
        memory.write(0, 1, 5.0);
        memory.write(3, 7, 10.0);

        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 1.0);
        assert_eq!(Some(1.0), memory.read(3, 7));
    }

    #[test]
    fn test_set_mod_shifted() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // block_id
            ResolvedNode::Value(2.0), // x
            ResolvedNode::Value(3.0), // y
            ResolvedNode::Value(4.0), // s
            ResolvedNode::Value(3.0), // value (mod divisor)
            ResolvedNode::OpCode(OpCode::SetModShifted(SetModShifted {
                block_id: 0,
                x: 1,
                y: 2,
                s: 3,
                value: 4,
            })),
        ];
        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![10.0; 4096]));
        memory.write(0, 14, 10.0); // 2 + 3*4 = 14

        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 1.0);
        assert_eq!(Some(1.0), memory.read(0, 14));
    }

    #[test]
    fn test_copy_basic() {
        let nodes = vec![
            ResolvedNode::Value(0.0), // src_block_id
            ResolvedNode::Value(0.0), // src_index
            ResolvedNode::Value(1.0), // dst_block_id
            ResolvedNode::Value(1.0), // dst_index
            ResolvedNode::Value(3.0), // count
            ResolvedNode::OpCode(OpCode::Copy(Copy {
                src_block_id: 0,
                src_index: 1,
                dst_block_id: 2,
                dst_index: 3,
                count: 4,
            })),
        ];

        let mut memory = BasicMemory::default();
        memory
            .writable
            .insert(0, RefCell::new(vec![11.0, 22.0, 33.0, 0.0, 0.0]));
        memory.writable.insert(1, RefCell::new(vec![0.0; 5]));

        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 0.0);
        assert_eq!(memory.read(1, 1), Some(11.0));
        assert_eq!(memory.read(1, 2), Some(22.0));
        assert_eq!(memory.read(1, 3), Some(33.0));
    }

    #[test]
    fn test_copy_overlap_forward() {
        let nodes = vec![
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(1.0),
            ResolvedNode::Value(3.0),
            ResolvedNode::OpCode(OpCode::Copy(Copy {
                src_block_id: 0,
                src_index: 1,
                dst_block_id: 2,
                dst_index: 3,
                count: 4,
            })),
        ];

        let mut memory = BasicMemory::default();
        memory
            .writable
            .insert(0, RefCell::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]));

        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 0.0);
        assert_eq!(memory.read(0, 1), Some(1.0));
        assert_eq!(memory.read(0, 2), Some(2.0));
        assert_eq!(memory.read(0, 3), Some(3.0));
        assert_eq!(memory.read(0, 4), Some(5.0));
    }

    #[test]
    fn test_copy_overlap_backward() {
        let nodes = vec![
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(2.0),
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(3.0),
            ResolvedNode::OpCode(OpCode::Copy(Copy {
                src_block_id: 0,
                src_index: 1,
                dst_block_id: 2,
                dst_index: 3,
                count: 4,
            })),
        ];

        let mut memory = BasicMemory::default();
        memory
            .writable
            .insert(0, RefCell::new(vec![10.0, 11.0, 12.0, 13.0, 14.0]));

        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 0.0);
        assert_eq!(memory.read(0, 0), Some(12.0));
        assert_eq!(memory.read(0, 1), Some(13.0));
        assert_eq!(memory.read(0, 2), Some(14.0));
    }
    #[test]
    fn test_copy_zero_count() {
        let nodes = vec![
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(1.0),
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(0.0),
            ResolvedNode::OpCode(OpCode::Copy(Copy {
                src_block_id: 0,
                src_index: 1,
                dst_block_id: 2,
                dst_index: 3,
                count: 4,
            })),
        ];

        let mut memory = BasicMemory::default();
        memory.writable.insert(0, RefCell::new(vec![5.0]));
        memory.writable.insert(1, RefCell::new(vec![9.0]));

        let func = build_and_return_function(&nodes, 5);
        let result = func(&mut RuntimeContext { memory: &memory });

        assert_eq!(result, 0.0);
        assert_eq!(memory.read(1, 0), Some(9.0));
    }
}
