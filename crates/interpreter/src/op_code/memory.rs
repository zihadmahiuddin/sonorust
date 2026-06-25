use sonorust_ir::{IRValue, modulo, nodes::*};
use sonorust_runtime::{SonorustIRExecutor, context::RuntimeContext};

use crate::{Executable, SonorustInterpreter, int_from_f64_checked};

impl Executable for Copy {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let src_block_id = executor.execute(nodes, self.src_block_id, context);
        let src_block_id = int_from_f64_checked(src_block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {src_block_id}"));
        let src_index = executor.execute(nodes, self.src_index, context);
        let src_index = int_from_f64_checked(src_index)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {src_index}"));

        let dst_block_id = executor.execute(nodes, self.dst_block_id, context);
        let dst_block_id = int_from_f64_checked(dst_block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {dst_block_id}"));
        let dst_index = executor.execute(nodes, self.dst_index, context);
        let dst_index = int_from_f64_checked(dst_index)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {dst_index}"));

        let count = executor.execute(nodes, self.count, context);
        let count = int_from_f64_checked(count)
            .unwrap_or_else(|| panic!("Expected count to be valid usize, found {count}"));

        context.copy_memory(src_block_id, src_index, dst_block_id, dst_index, count)
    }
}

impl Executable for Get {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        context.memory.read(block_id, index).unwrap_or_default()
    }
}

impl Executable for GetPointed {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let offset = executor.execute(nodes, self.offset, context);

        let final_block_id = context.memory.read(block_id, index).unwrap_or_default();
        let final_block_id = int_from_f64_checked(final_block_id).unwrap_or_else(|| {
            panic!("Expected final_block_id to be valid usize, found {final_block_id}")
        });

        let final_index = context.memory.read(block_id, index + 1).unwrap_or_default() + offset;
        let final_index = int_from_f64_checked(final_index).unwrap_or_else(|| {
            panic!("Expected final_index to be valid usize, found {final_index}")
        });

        context
            .memory
            .read(final_block_id, final_index)
            .unwrap_or_default()
    }
}

impl Executable for GetShifted {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let x = executor.execute(nodes, self.x, context);
        let y = executor.execute(nodes, self.y, context);
        let s = executor.execute(nodes, self.s, context);

        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        context.memory.read(block_id, index).unwrap_or_default()
    }
}

impl Executable for Set {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        context
            .memory
            .write(block_id, index, value)
            .unwrap_or_default()
    }
}

impl Executable for SetPointed {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let offset = executor.execute(nodes, self.offset, context);

        let final_block_id = context.memory.read(block_id, index).unwrap_or_default();
        let final_block_id = int_from_f64_checked(final_block_id).unwrap_or_else(|| {
            panic!("Expected final_block_id to be valid usize, found {final_block_id}")
        });

        let final_index = context.memory.read(block_id, index + 1).unwrap_or_default() + offset;
        let final_index = int_from_f64_checked(final_index).unwrap_or_else(|| {
            panic!("Expected final_index to be valid usize, found {final_index}")
        });

        let value = executor.execute(nodes, self.value, context);

        context
            .memory
            .write(final_block_id, final_index, value)
            .unwrap_or_default()
    }
}

impl Executable for SetShifted {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let x = executor.execute(nodes, self.x, context);
        let y = executor.execute(nodes, self.y, context);
        let s = executor.execute(nodes, self.s, context);

        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        context
            .memory
            .write(block_id, index, value)
            .unwrap_or_default()
    }
}

impl Executable for SetAdd {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, old_value + value)
            .unwrap_or_default()
    }
}

impl Executable for SetAddPointed {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let offset = executor.execute(nodes, self.offset, context);

        let final_block_id = context.memory.read(block_id, index).unwrap_or_default();
        let final_block_id = int_from_f64_checked(final_block_id).unwrap_or_else(|| {
            panic!("Expected final_block_id to be valid usize, found {final_block_id}")
        });

        let final_index = context.memory.read(block_id, index + 1).unwrap_or_default() + offset;
        let final_index = int_from_f64_checked(final_index).unwrap_or_else(|| {
            panic!("Expected final_index to be valid usize, found {final_index}")
        });

        let value = executor.execute(nodes, self.value, context);

        let old_value = context
            .memory
            .read(final_block_id, final_index)
            .unwrap_or_default();

        context
            .memory
            .write(final_block_id, final_index, old_value + value)
            .unwrap_or_default()
    }
}

impl Executable for SetAddShifted {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let x = executor.execute(nodes, self.x, context);
        let y = executor.execute(nodes, self.y, context);
        let s = executor.execute(nodes, self.s, context);

        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, old_value + value)
            .unwrap_or_default()
    }
}

impl Executable for SetDivide {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, old_value / value)
            .unwrap_or_default()
    }
}

impl Executable for SetDividePointed {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let offset = executor.execute(nodes, self.offset, context);

        let final_block_id = context.memory.read(block_id, index).unwrap_or_default();
        let final_block_id = int_from_f64_checked(final_block_id).unwrap_or_else(|| {
            panic!("Expected final_block_id to be valid usize, found {final_block_id}")
        });

        let final_index = context.memory.read(block_id, index + 1).unwrap_or_default() + offset;
        let final_index = int_from_f64_checked(final_index).unwrap_or_else(|| {
            panic!("Expected final_index to be valid usize, found {final_index}")
        });

        let value = executor.execute(nodes, self.value, context);

        let old_value = context
            .memory
            .read(final_block_id, final_index)
            .unwrap_or_default();

        context
            .memory
            .write(final_block_id, final_index, old_value / value)
            .unwrap_or_default()
    }
}

impl Executable for SetDivideShifted {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let x = executor.execute(nodes, self.x, context);
        let y = executor.execute(nodes, self.y, context);
        let s = executor.execute(nodes, self.s, context);

        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, old_value / value)
            .unwrap_or_default()
    }
}

impl Executable for SetMultiply {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, old_value * value)
            .unwrap_or_default()
    }
}

impl Executable for SetMultiplyPointed {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let offset = executor.execute(nodes, self.offset, context);

        let final_block_id = context.memory.read(block_id, index).unwrap_or_default();
        let final_block_id = int_from_f64_checked(final_block_id).unwrap_or_else(|| {
            panic!("Expected final_block_id to be valid usize, found {final_block_id}")
        });

        let final_index = context.memory.read(block_id, index + 1).unwrap_or_default() + offset;
        let final_index = int_from_f64_checked(final_index).unwrap_or_else(|| {
            panic!("Expected final_index to be valid usize, found {final_index}")
        });

        let value = executor.execute(nodes, self.value, context);

        let old_value = context
            .memory
            .read(final_block_id, final_index)
            .unwrap_or_default();

        context
            .memory
            .write(final_block_id, final_index, old_value * value)
            .unwrap_or_default()
    }
}

impl Executable for SetMultiplyShifted {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let x = executor.execute(nodes, self.x, context);
        let y = executor.execute(nodes, self.y, context);
        let s = executor.execute(nodes, self.s, context);

        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, old_value * value)
            .unwrap_or_default()
    }
}

impl Executable for SetMod {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, modulo(old_value, value))
            .unwrap_or_default()
    }
}

impl Executable for SetModPointed {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let offset = executor.execute(nodes, self.offset, context);

        let final_block_id = context.memory.read(block_id, index).unwrap_or_default();
        let final_block_id = int_from_f64_checked(final_block_id).unwrap_or_else(|| {
            panic!("Expected final_block_id to be valid usize, found {final_block_id}")
        });

        let final_index = context.memory.read(block_id, index + 1).unwrap_or_default() + offset;
        let final_index = int_from_f64_checked(final_index).unwrap_or_else(|| {
            panic!("Expected final_index to be valid usize, found {final_index}")
        });

        let value = executor.execute(nodes, self.value, context);

        let old_value = context
            .memory
            .read(final_block_id, final_index)
            .unwrap_or_default();

        context
            .memory
            .write(final_block_id, final_index, modulo(old_value, value))
            .unwrap_or_default()
    }
}

impl Executable for SetModShifted {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let x = executor.execute(nodes, self.x, context);
        let y = executor.execute(nodes, self.y, context);
        let s = executor.execute(nodes, self.s, context);

        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, modulo(old_value, value))
            .unwrap_or_default()
    }
}

impl Executable for SetRem {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, old_value % value)
            .unwrap_or_default()
    }
}

impl Executable for SetRemPointed {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let offset = executor.execute(nodes, self.offset, context);

        let final_block_id = context.memory.read(block_id, index).unwrap_or_default();
        let final_block_id = int_from_f64_checked(final_block_id).unwrap_or_else(|| {
            panic!("Expected final_block_id to be valid usize, found {final_block_id}")
        });

        let final_index = context.memory.read(block_id, index + 1).unwrap_or_default() + offset;
        let final_index = int_from_f64_checked(final_index).unwrap_or_else(|| {
            panic!("Expected final_index to be valid usize, found {final_index}")
        });

        let value = executor.execute(nodes, self.value, context);

        let old_value = context
            .memory
            .read(final_block_id, final_index)
            .unwrap_or_default();

        context
            .memory
            .write(final_block_id, final_index, old_value % value)
            .unwrap_or_default()
    }
}

impl Executable for SetRemShifted {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let x = executor.execute(nodes, self.x, context);
        let y = executor.execute(nodes, self.y, context);
        let s = executor.execute(nodes, self.s, context);

        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, old_value % value)
            .unwrap_or_default()
    }
}

impl Executable for SetPower {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, old_value.powf(value))
            .unwrap_or_default()
    }
}

impl Executable for SetPowerPointed {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let offset = executor.execute(nodes, self.offset, context);

        let final_block_id = context.memory.read(block_id, index).unwrap_or_default();
        let final_block_id = int_from_f64_checked(final_block_id).unwrap_or_else(|| {
            panic!("Expected final_block_id to be valid usize, found {final_block_id}")
        });

        let final_index = context.memory.read(block_id, index + 1).unwrap_or_default() + offset;
        let final_index = int_from_f64_checked(final_index).unwrap_or_else(|| {
            panic!("Expected final_index to be valid usize, found {final_index}")
        });

        let value = executor.execute(nodes, self.value, context);

        let old_value = context
            .memory
            .read(final_block_id, final_index)
            .unwrap_or_default();

        context
            .memory
            .write(final_block_id, final_index, old_value.powf(value))
            .unwrap_or_default()
    }
}

impl Executable for SetPowerShifted {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let x = executor.execute(nodes, self.x, context);
        let y = executor.execute(nodes, self.y, context);
        let s = executor.execute(nodes, self.s, context);

        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, old_value.powf(value))
            .unwrap_or_default()
    }
}

impl Executable for SetSubtract {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, old_value - value)
            .unwrap_or_default()
    }
}

impl Executable for SetSubtractPointed {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let index = executor.execute(nodes, self.index, context);
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let offset = executor.execute(nodes, self.offset, context);

        let final_block_id = context.memory.read(block_id, index).unwrap_or_default();
        let final_block_id = int_from_f64_checked(final_block_id).unwrap_or_else(|| {
            panic!("Expected final_block_id to be valid usize, found {final_block_id}")
        });

        let final_index = context.memory.read(block_id, index + 1).unwrap_or_default() + offset;
        let final_index = int_from_f64_checked(final_index).unwrap_or_else(|| {
            panic!("Expected final_index to be valid usize, found {final_index}")
        });

        let value = executor.execute(nodes, self.value, context);

        let old_value = context
            .memory
            .read(final_block_id, final_index)
            .unwrap_or_default();

        context
            .memory
            .write(final_block_id, final_index, old_value - value)
            .unwrap_or_default()
    }
}

impl Executable for SetSubtractShifted {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let block_id = executor.execute(nodes, self.block_id, context);
        let block_id = int_from_f64_checked(block_id)
            .unwrap_or_else(|| panic!("Expected block ID to be valid u16, found {block_id}"));

        let x = executor.execute(nodes, self.x, context);
        let y = executor.execute(nodes, self.y, context);
        let s = executor.execute(nodes, self.s, context);

        let index = x + y * s;
        let index = int_from_f64_checked(index)
            .unwrap_or_else(|| panic!("Expected index to be valid usize, found {index}"));

        let value = executor.execute(nodes, self.value, context);

        let old_value = context.memory.read(block_id, index).unwrap_or_default();

        context
            .memory
            .write(block_id, index, old_value - value)
            .unwrap_or_default()
    }
}
