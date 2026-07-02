#![feature(sync_nonpoison)]
#![feature(nonpoison_rwlock)]

use crate::context::RuntimeContext;
use sonorust_ir::{IRValue, nodes::IRNode};

pub mod context;
pub mod memory;
pub mod testing;

pub trait SonorustIRExecutor {
    #[allow(unused)]
    fn prepare(&mut self, nodes: &[IRNode], root_index: usize) {}

    fn execute(
        &mut self,
        nodes: &[IRNode],
        root_index: usize,
        context: &mut RuntimeContext,
    ) -> IRValue;
}
