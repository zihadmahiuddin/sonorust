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
        "pow" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // a
            sig.params.push(AbiParam::new(types::F32)); // b

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
            ResolvedNode::Value(value) => self.builder.ins().f32const(*value),
            ResolvedNode::OpCode(opcode) => match opcode {
                // Control Flow
                OpCode::Execute(node) => self.build_execute_ir(node),
                OpCode::Execute0(node) => self.build_execute0_ir(node),
                OpCode::Block(node) => self.build_block_ir(node),
                OpCode::Break(node) => self.build_break_ir(node),
                OpCode::If(node) => self.build_if_ir(node),
                OpCode::While(node) => self.build_while_ir(node),
                OpCode::SwitchInteger(node) => self.build_switch_integer_ir(node),
                OpCode::SwitchIntegerWithDefault(node) => {
                    self.build_switch_integer_with_default_ir(node)
                }
                // Math
                OpCode::Abs(node) => self.build_abs_ir(node),
                OpCode::Frac(node) => self.build_frac_ir(node),
                OpCode::Trunc(node) => self.build_trunc_ir(node),
                OpCode::Negate(node) => self.build_negate_ir(node),
                OpCode::Add(node) => self.build_add_ir(node),
                OpCode::Subtract(node) => self.build_subtract_ir(node),
                OpCode::Multiply(node) => self.build_multiply_ir(node),
                OpCode::Divide(node) => self.build_divide_ir(node),
                OpCode::Mod(node) => self.build_mod_ir(node),
                OpCode::Rem(node) => self.build_rem_ir(node),
                OpCode::Power(node) => self.build_power_ir(node),
                OpCode::Clamp(node) => self.build_clamp_ir(node),
                OpCode::Lerp(node) => self.build_lerp_ir(node),
                OpCode::LerpClamped(node) => self.build_lerp_clamped_ir(node),
                OpCode::Unlerp(node) => self.build_unlerp_ir(node),
                OpCode::UnlerpClamped(node) => self.build_unlerp_clamped_ir(node),
                // Logical
                OpCode::Equal(node) => self.build_equal_ir(node),
                OpCode::NotEqual(node) => self.build_not_equal_ir(node),
                OpCode::Greater(node) => self.build_greater_ir(node),
                OpCode::GreaterOr(node) => self.build_greater_or_ir(node),
                OpCode::Less(node) => self.build_less_ir(node),
                OpCode::LessOr(node) => self.build_less_or_ir(node),
                OpCode::And(node) => self.build_and_ir(node),
                OpCode::Or(node) => self.build_or_ir(node),
                OpCode::Not(node) => self.build_not_ir(node),
                // Memory
                OpCode::Get(node) => self.build_get_ir(node),
                OpCode::GetPointed(node) => self.build_get_pointed_ir(node),
                OpCode::GetShifted(node) => self.build_get_shifted_ir(node),
                OpCode::Set(node) => self.build_set_ir(node),
                OpCode::SetPointed(node) => self.build_set_pointed_ir(node),
                OpCode::SetShifted(node) => self.build_set_shifted_ir(node),
                OpCode::SetAdd(node) => self.build_set_add_ir(node),
                OpCode::SetAddPointed(node) => self.build_set_add_pointed_ir(node),
                OpCode::SetAddShifted(node) => self.build_set_add_shifted_ir(node),
                OpCode::SetSubtract(node) => self.build_set_subtract_ir(node),
                OpCode::SetSubtractPointed(node) => self.build_set_subtract_pointed_ir(node),
                OpCode::SetSubtractShifted(node) => self.build_set_subtract_shifted_ir(node),
                OpCode::SetMultiply(node) => self.build_set_multiply_ir(node),
                OpCode::SetMultiplyPointed(node) => self.build_set_multiply_pointed_ir(node),
                OpCode::SetMultiplyShifted(node) => self.build_set_multiply_shifted_ir(node),
                OpCode::SetDivide(node) => self.build_set_divide_ir(node),
                OpCode::SetDividePointed(node) => self.build_set_divide_pointed_ir(node),
                OpCode::SetDivideShifted(node) => self.build_set_divide_shifted_ir(node),
                OpCode::SetPower(node) => self.build_set_power_ir(node),
                OpCode::SetPowerPointed(node) => self.build_set_power_pointed_ir(node),
                OpCode::SetPowerShifted(node) => self.build_set_power_shifted_ir(node),
                OpCode::SetRem(node) => self.build_set_rem_ir(node),
                OpCode::SetRemPointed(node) => self.build_set_rem_pointed_ir(node),
                OpCode::SetRemShifted(node) => self.build_set_rem_shifted_ir(node),
                OpCode::SetMod(node) => self.build_set_mod_ir(node),
                OpCode::SetModPointed(node) => self.build_set_mod_pointed_ir(node),
                OpCode::SetModShifted(node) => self.build_set_mod_shifted_ir(node),
            },
        }
    }
}
