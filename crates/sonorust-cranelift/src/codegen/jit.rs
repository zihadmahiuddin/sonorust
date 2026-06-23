use std::collections::HashMap;

use crate::codegen::build_cranelift_function;
use crate::codegen::create_signature_for;
use cranelift::{
    codegen::ir::{Function, UserFuncName},
    jit::{JITBuilder, JITModule},
    module::{Linkage, Module},
    prelude::*,
};
use sonorust_ir::nodes::*;
use sonorust_runtime::context::RuntimeContext;
use sonorust_runtime::context::get_external_functions;

pub fn build_and_return_function(
    nodes: &[ResolvedNode],
    root_index: usize,
) -> fn(*mut RuntimeContext) -> f32 {
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
    unsafe { std::mem::transmute::<*const u8, fn(*mut RuntimeContext) -> f32>(code_ptr) }
}
