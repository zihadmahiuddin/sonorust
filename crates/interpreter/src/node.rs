use sonorust_ir::{IRValue, nodes::IRNode};
use sonorust_runtime::context::RuntimeContext;

use crate::{Executable, SonorustInterpreter};

impl Executable for IRNode {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        match self {
            IRNode::Value(value) => *value,
            IRNode::OpCode(op_code) => op_code.execute(context, nodes, executor),
        }
    }
}
