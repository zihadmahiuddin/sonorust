use std::ops::ControlFlow;

mod node;
mod op_code;

use num_traits::FromPrimitive;
use sonorust_ir::{IRValue, nodes::ResolvedNode};
use sonorust_runtime::{SonorustIRExecutor, context::RuntimeContext};

type ControlFlowState = ControlFlow<Vec<IRValue>>;

pub trait Executable {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[ResolvedNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue;
}

pub struct SonorustInterpreter {
    control_flow: ControlFlowState,
}

impl Default for SonorustInterpreter {
    fn default() -> Self {
        Self {
            control_flow: ControlFlow::Continue(()),
        }
    }
}

impl SonorustInterpreter {
    fn set_control(&mut self, flow: ControlFlowState) {
        self.control_flow = flow;
    }

    fn control(&self) -> &ControlFlowState {
        &self.control_flow
    }

    fn control_mut(&mut self) -> &mut ControlFlowState {
        &mut self.control_flow
    }

    fn take_control(&mut self) -> ControlFlowState {
        std::mem::replace(self.control_mut(), ControlFlow::Continue(()))
    }
}

impl SonorustIRExecutor for SonorustInterpreter {
    fn execute(
        &mut self,
        nodes: &[ResolvedNode],
        root_index: usize,
        context: &mut RuntimeContext,
    ) -> IRValue {
        match &nodes[root_index] {
            ResolvedNode::Value(value) => *value,
            ResolvedNode::OpCode(op_code) => op_code.execute(context, nodes, self),
        }
    }
}

pub(crate) fn int_from_f64_checked<T>(n: IRValue) -> Option<T>
where
    T: FromPrimitive,
{
    if n.fract() != 0.0 {
        return None;
    }
    T::from_f32(n)
}
