pub mod control_flow;
pub mod jit;
pub mod logical;
pub mod math;
pub mod memory;

use std::collections::HashMap;

use cranelift::{
    codegen::ir::{FuncRef, Function},
    prelude::{
        AbiParam, Block, FunctionBuilder, FunctionBuilderContext, InstBuilder, Signature, Value,
        isa::CallConv, types,
    },
};

use crate::nodes::{OpCode, ResolvedNode};

pub(crate) fn create_signature_for(name: &str, call_conv: CallConv) -> Signature {
    let mut sig = Signature::new(call_conv);
    match name {
        "read_mem" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::I64)); // block_id
            sig.params.push(AbiParam::new(types::I64)); // index

            sig.returns.push(AbiParam::new(types::F32));
        }
        "write_mem" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::I64)); // block_id
            sig.params.push(AbiParam::new(types::I64)); // index
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        _ => panic!("Unknown external function: {name}"),
    }
    sig
}

pub fn build_cranelift_function(
    func: &mut Function,
    externals_func_refs: &HashMap<&str, FuncRef>,
    nodes: &[ResolvedNode],
    root_index: usize,
) {
    let mut ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(func, &mut ctx);

    let entry_block = builder.create_block();
    builder.append_block_params_for_function_params(entry_block);
    builder.switch_to_block(entry_block);
    builder.ensure_inserted_block();
    let ctx_param = builder.block_params(entry_block)[0];

    let mut codegen_context =
        CodegenContext::new(&mut builder, externals_func_refs, ctx_param, nodes);
    let result = codegen_context.build_node_ir(root_index);
    builder.ins().return_(&[result]);
    builder.seal_block(entry_block);
    builder.finalize();
}

#[derive(Debug, Clone)]
pub enum BlockKind {
    Regular(Block),
    While {
        head: Block,
        body: Block,
        exit: Block,
    },
}

impl BlockKind {
    pub fn exit_block(&self) -> Block {
        *match self {
            BlockKind::Regular(block) => block,
            BlockKind::While { exit, .. } => exit,
        }
    }
}

pub struct CodegenContext<'s, 'b> {
    builder: &'s mut FunctionBuilder<'b>,
    externals_func_refs: &'s HashMap<&'s str, FuncRef>,
    ctx_param: Value,
    nodes: &'s [ResolvedNode],
    block_stack: Vec<BlockKind>,
    pending_block_stack: Option<BlockKind>,
    current_block_terminated: bool,
}

impl<'s, 'b> CodegenContext<'s, 'b> {
    pub fn new(
        builder: &'s mut FunctionBuilder<'b>,
        externals_func_refs: &'s HashMap<&'s str, FuncRef>,
        ctx_param: Value,
        nodes: &'s [ResolvedNode],
    ) -> Self {
        Self {
            builder,
            externals_func_refs,
            ctx_param,
            nodes,
            block_stack: Vec::new(),
            pending_block_stack: None,
            current_block_terminated: false,
        }
    }

    // Helper to build arbitrary nodes recursively
    pub fn build_node_ir(&mut self, node_index: usize) -> Value {
        match &self.nodes[node_index] {
            ResolvedNode::Value(value) => self.builder.ins().f32const(*value as f32),
            ResolvedNode::OpCode(opcode) => match opcode {
                OpCode::Add(node) => self.build_add_ir(node),
                OpCode::Subtract(node) => self.build_subtract_ir(node),
                OpCode::Multiply(node) => self.build_multiply_ir(node),
                OpCode::Divide(node) => self.build_divide_ir(node),
                OpCode::Execute(node) => self.build_execute_ir(node),
                OpCode::Block(node) => self.build_block_ir(node),
                OpCode::Break(node) => self.build_break_ir(node),
                OpCode::If(node) => self.build_if_ir(node),
                OpCode::While(node) => self.build_while_ir(node),
                OpCode::SwitchInteger(node) => self.build_switch_integer_ir(node),
                OpCode::SwitchIntegerWithDefault(node) => {
                    self.build_switch_integer_with_default_ir(node)
                }
                OpCode::Get(node) => self.build_get_ir(node),
                OpCode::Set(node) => self.build_set_ir(node),
                OpCode::Equal(node) => self.build_equal_ir(node),
                OpCode::NotEqual(node) => self.build_not_equal_ir(node),
                other => todo!("Implement {other:?}"),
            },
        }
    }
}
