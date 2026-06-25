use crate::context::RuntimeContext;
use sonorust_ir::{IRValue, nodes::ResolvedNode};

pub mod context;
pub mod testing;

pub trait SonorustIRExecutor {
    #[allow(unused)]
    fn prepare(&mut self, nodes: &[ResolvedNode], root_index: usize) {}

    fn execute(
        &mut self,
        nodes: &[ResolvedNode],
        root_index: usize,
        context: &mut RuntimeContext,
    ) -> IRValue;
}
