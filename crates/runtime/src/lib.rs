#![feature(sync_nonpoison)]
#![feature(nonpoison_rwlock)]

use crate::context::RuntimeContext;
use sonorust_ir::{IRIndex, IRValue, nodes::IRNode};

pub mod access;
pub mod context;
pub mod memory;
pub mod side_effects;
pub mod testing;

pub trait SonorustIRExecutor {
    #[allow(unused)]
    fn prepare(&mut self, nodes: &[IRNode], root_index: IRIndex) {}

    fn execute(
        &mut self,
        nodes: &[IRNode],
        root_index: IRIndex,
        context: &mut RuntimeContext,
    ) -> IRValue;
}
