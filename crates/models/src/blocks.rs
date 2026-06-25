use sonorust_ir::IRValue;

pub mod common;
pub mod play;

pub trait ReadableBlock {
    fn read(&self, index: usize) -> Option<IRValue>;
}

pub trait WritableBlock: ReadableBlock {
    fn write(&mut self, index: usize, value: IRValue) -> bool;
}
