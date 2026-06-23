use sonorust_ir::{IRValue, nodes::*};
use sonorust_runtime::{SonorustIRExecutor, context::RuntimeContext};

use crate::{Executable, SonorustInterpreter};

impl Executable for Equal {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[ResolvedNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let lhs = executor.execute(nodes, self.lhs, context);
        let rhs = executor.execute(nodes, self.rhs, context);
        if (lhs - rhs).abs() < IRValue::EPSILON {
            1.0
        } else {
            0.0
        }
    }
}

impl Executable for NotEqual {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[ResolvedNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let lhs = executor.execute(nodes, self.lhs, context);
        let rhs = executor.execute(nodes, self.rhs, context);
        if (lhs - rhs).abs() >= IRValue::EPSILON {
            1.0
        } else {
            0.0
        }
    }
}

impl Executable for Greater {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[ResolvedNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let lhs = executor.execute(nodes, self.lhs, context);
        let rhs = executor.execute(nodes, self.rhs, context);
        if lhs > rhs { 1.0 } else { 0.0 }
    }
}

impl Executable for GreaterOr {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[ResolvedNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let lhs = executor.execute(nodes, self.lhs, context);
        let rhs = executor.execute(nodes, self.rhs, context);
        if lhs >= rhs { 1.0 } else { 0.0 }
    }
}

impl Executable for Less {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[ResolvedNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let lhs = executor.execute(nodes, self.lhs, context);
        let rhs = executor.execute(nodes, self.rhs, context);
        if lhs < rhs { 1.0 } else { 0.0 }
    }
}

impl Executable for LessOr {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[ResolvedNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let lhs = executor.execute(nodes, self.lhs, context);
        let rhs = executor.execute(nodes, self.rhs, context);
        if lhs <= rhs { 1.0 } else { 0.0 }
    }
}

impl Executable for And {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[ResolvedNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let mut last_value = 0.0;

        for &input in &self.inputs {
            let value = executor.execute(nodes, input, context);

            if value == 0.0
            // && self.inputs[0] != 1171
            {
                return 0.0;
            }

            last_value = value;
        }

        last_value

        // let mut inputs_iter = self.inputs.iter();

        // let first_value = if let Some(first_node) = inputs_iter.next() {
        //     executor.execute(*first_node)
        // } else {
        //     return (executor, 0.0);
        // };

        // inputs_iter.fold((executor, first_value), |(executor, acc), &idx| {
        //     let value = executor.execute(idx);
        //     (executor, (acc as i64 & value as i64) as f64)
        // })
    }
}

impl Executable for Or {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[ResolvedNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        for &input in &self.inputs {
            let value = executor.execute(nodes, input, context);
            if value != 0.0 {
                return value;
            }
        }
        0.0

        // let mut inputs_iter = self.inputs.iter();

        // let first_value = if let Some(first_node) = inputs_iter.next() {
        //     executor.execute(*first_node)
        // } else {
        //     return (executor, 0.0);
        // };

        // inputs_iter.fold((executor, first_value), |(executor, acc), &idx| {
        //     let value = executor.execute(idx);
        //     (executor, (acc as i64 | value as i64) as f64)
        // })
    }
}

impl Executable for Not {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[ResolvedNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        if value == 0.0 { 1.0 } else { 0.0 }
    }
}
