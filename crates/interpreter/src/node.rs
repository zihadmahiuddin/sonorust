use sonorust_ir::{IRValue, nodes::ResolvedNode};
use sonorust_runtime::context::RuntimeContext;

use crate::{Executable, SonorustInterpreter};

impl Executable for ResolvedNode {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[ResolvedNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        match self {
            ResolvedNode::Value(value) => *value,
            ResolvedNode::OpCode(op_code) => op_code.execute(context, nodes, executor),
        }
    }
}
