use std::collections::HashMap;

use cranelift::{
    codegen::ir::{FuncRef, Function, UserFuncName},
    frontend::FuncInstBuilder,
    jit::{JITBuilder, JITModule},
    module::{Linkage, Module},
    prelude::{isa::CallConv, *},
};
use sonorust_ir::{IRValue, nodes::ResolvedNode};
use sonorust_runtime::{
    SonorustIRExecutor,
    context::{RuntimeContext, get_external_functions},
};

use crate::codegen::CodegenContext;

pub mod codegen;

const IR_VALUE_CRANELIFT_TYPE: Type = types::F32;

fn ir_value_cranelift_const(ins: FuncInstBuilder, value: IRValue) -> Value {
    ins.f32const(value)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct NodeIndex(usize);

type CompiledFunctionPointer = fn(*mut RuntimeContext) -> IRValue;

#[derive(Debug, Default)]
pub struct CraneliftJitExecutor {
    prepared_functions: HashMap<NodeIndex, CompiledFunctionPointer>,
}

impl CraneliftJitExecutor {
    fn build_function(nodes: &[ResolvedNode], root_index: usize) -> CompiledFunctionPointer {
        let externals_addrs = get_external_functions();
        let mut externals_func_refs = HashMap::new();

        let isa = {
            let builder = settings::builder();
            let flags = settings::Flags::new(builder);
            let isa_builder = cranelift::native::builder().unwrap_or_else(|msg| {
                panic!("host machine is not supported: {msg}");
            });
            isa_builder.finish(flags).unwrap()
        };
        let mut builder = JITBuilder::with_isa(isa, cranelift::module::default_libcall_names());

        for (&name, &addr) in &externals_addrs {
            builder.symbol(name, addr);
        }

        let mut module = JITModule::new(builder);

        let mut sig = Signature::new(module.isa().default_call_conv());
        sig.params.push(AbiParam::new(types::I64));
        sig.returns.push(AbiParam::new(types::F32));
        let mut func = Function::with_name_signature(UserFuncName::user(0, 0), sig.clone());

        for &name in externals_addrs.keys() {
            let sig = create_signature_for(name, module.isa().default_call_conv());
            let func_id = module
                .declare_function(name, Linkage::Import, &sig)
                .unwrap_or_else(|_| panic!("declaring external function: {name}"));
            let func_ref = module.declare_func_in_func(func_id, &mut func);
            externals_func_refs.insert(name, func_ref);
        }

        let mut ctx = module.make_context();
        ctx.func = func;
        build_cranelift_function(&mut ctx.func, &externals_func_refs, nodes, root_index);

        let func_id = module
            .declare_function("main", Linkage::Export, &ctx.func.signature)
            .unwrap();

        module.define_function(func_id, &mut ctx).unwrap();
        module.clear_context(&mut ctx);
        module.finalize_definitions().unwrap();

        let code_ptr = module.get_finalized_function(func_id);
        unsafe { std::mem::transmute::<*const u8, fn(*mut RuntimeContext) -> IRValue>(code_ptr) }
    }
}

impl SonorustIRExecutor for CraneliftJitExecutor {
    fn prepare(&mut self, nodes: &[ResolvedNode], root_index: usize) {
        let func = Self::build_function(nodes, root_index);
        self.prepared_functions.insert(NodeIndex(root_index), func);
    }

    fn execute(
        &mut self,
        nodes: &[ResolvedNode],
        root_index: usize,
        runtime_context: &mut RuntimeContext,
    ) -> f32 {
        let func = self
            .prepared_functions
            .get(&NodeIndex(root_index))
            .copied()
            .unwrap_or_else(|| Self::build_function(nodes, root_index));

        func(runtime_context as _)
    }
}

fn build_cranelift_function(
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

fn create_signature_for(name: &str, call_conv: CallConv) -> Signature {
    let mut sig = Signature::new(call_conv);
    // TODO: typesafe?
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
        "copy_mem" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::I64)); // src_block_id
            sig.params.push(AbiParam::new(types::I64)); // src_index
            sig.params.push(AbiParam::new(types::I64)); // dst_block_id
            sig.params.push(AbiParam::new(types::I64)); // dst_index
            sig.params.push(AbiParam::new(types::I64)); // count

            sig.returns.push(AbiParam::new(types::F32));
        }
        "pow" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // a
            sig.params.push(AbiParam::new(types::F32)); // b

            sig.returns.push(AbiParam::new(types::F32));
        }
        "sin" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        "cos" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        "tan" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        "sinh" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        "cosh" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        "tanh" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        "arcsin" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        "arccos" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        "arctan" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        "arctan2" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // a
            sig.params.push(AbiParam::new(types::F32)); // b

            sig.returns.push(AbiParam::new(types::F32));
        }
        "degree" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        "radian" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        "log" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // value

            sig.returns.push(AbiParam::new(types::F32));
        }
        "random" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // min
            sig.params.push(AbiParam::new(types::F32)); // max

            sig.returns.push(AbiParam::new(types::F32));
        }
        "random_integer" => {
            sig.params.push(AbiParam::new(types::I64)); // ctx ptr
            sig.params.push(AbiParam::new(types::F32)); // min
            sig.params.push(AbiParam::new(types::F32)); // max

            sig.returns.push(AbiParam::new(types::F32));
        }
        _ => panic!("Unknown external function: {name}"),
    }
    sig
}
