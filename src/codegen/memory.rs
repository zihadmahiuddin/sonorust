use crate::{codegen::CodegenContext, nodes::*};

use cranelift::prelude::*;

impl<'s, 'b> CodegenContext<'s, 'b> {
    pub(crate) fn build_get_ir(&mut self, node: &Get) -> Value {
        let block_id_f32 = self.build_node_ir(node.block_id);
        let block_id = self.builder.ins().fcvt_to_sint(types::I64, block_id_f32);
        let index_f32 = self.build_node_ir(node.index);
        let index = self.builder.ins().fcvt_to_sint(types::I64, index_f32);

        let fn_ref = self.externals_func_refs["read_mem"];
        let fn_call = self
            .builder
            .ins()
            .call(fn_ref, &[self.ctx_param, block_id, index]);
        self.builder.inst_results(fn_call)[0]
    }

    pub(crate) fn build_set_ir(&mut self, node: &Set) -> Value {
        let block_id_f32 = self.build_node_ir(node.block_id);
        let block_id = self.builder.ins().fcvt_to_sint(types::I64, block_id_f32);
        let index_f32 = self.build_node_ir(node.index);
        let index = self.builder.ins().fcvt_to_sint(types::I64, index_f32);
        let value = self.build_node_ir(node.value);

        let fn_ref = self.externals_func_refs["write_mem"];
        let fn_call = self
            .builder
            .ins()
            .call(fn_ref, &[self.ctx_param, block_id, index, value]);
        self.builder.inst_results(fn_call)[0]
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
    fn test_set_and_get() {
        let nodes = vec![
            ResolvedNode::Value(0.0),
            ResolvedNode::Value(7.0),
            ResolvedNode::OpCode(OpCode::Set(Set {
                block_id: 0,
                index: 0,
                value: 1,
            })),
            ResolvedNode::OpCode(OpCode::Get(Get {
                block_id: 0,
                index: 0,
            })),
            ResolvedNode::OpCode(OpCode::Execute(Execute { nodes: vec![2, 3] })),
        ];

        let mut runtime_context = RuntimeContext {
            memory: &BasicMemory::default(),
        };
        let func = build_and_return_function(&nodes, 4);
        let result = func(&mut runtime_context as _);
        assert_eq!(result, 7.0);
    }
}
