use std::ops::ControlFlow;

use sonorust_ir::{IRValue, nodes::*};
use sonorust_runtime::{SonorustIRExecutor, context::RuntimeContext};

use crate::{Executable, SonorustInterpreter, int_from_float_checked};

impl Executable for Execute {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let mut value: IRValue = 0.0;

        for &node in &self.nodes {
            // Check the program flow before executing the next operation.
            if executor.control().is_break() {
                // If a break is active, stop executing further opcodes in this sequence.
                // The Block containing this sequence will handle the break state.
                // Or the break value if accessible
                break;
            }

            // Proceed to execute the current operation.
            value = executor.execute(nodes, node, context);
        }
        value
    }
}

impl Executable for Execute0 {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        for &node in &self.nodes {
            // Check the program flow before executing the next operation.
            if executor.control().is_break() {
                // If a break is active, stop executing further opcodes in this sequence.
                // The Block containing this sequence will handle the break state.
                // Or the break value if accessible
                break;
            }

            // Proceed to execute the current operation.
            executor.execute(nodes, node, context);
        }
        0.0
    }
}

impl Executable for Block {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        match executor.take_control() {
            ControlFlow::Continue(()) => executor.execute(nodes, self.body, context),
            ControlFlow::Break(stack) if stack.len() == 1 => 0.0,
            ControlFlow::Break(mut stack) => {
                let last = stack.pop().unwrap_or_default();
                executor.set_control(ControlFlow::Break(stack));
                last
            }
        }
    }
}

impl Executable for Break {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let count = executor.execute(nodes, self.count, context);
        let count = int_from_float_checked(count)
            .unwrap_or_else(|| panic!("Expected break count to be valid usize, found {count}"));

        let value = executor.execute(nodes, self.value, context);

        let mut stack = match executor.take_control() {
            ControlFlow::Break(stack) => stack,
            ControlFlow::Continue(()) => vec![],
        };

        for _ in 0..count {
            stack.push(value);
        }

        executor.set_control(ControlFlow::Break(stack));
        value
    }
}

impl Executable for If {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let test = executor.execute(nodes, self.test, context);
        let target = if test != 0.0 {
            self.consequent
        } else {
            self.alternate
        };
        executor.execute(nodes, target, context)
    }
}

impl Executable for While {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        loop {
            let test = executor.execute(nodes, self.test, context);

            if test == 0.0 {
                return 0.0;
            }

            let _ = executor.execute(nodes, self.body, context);

            match executor.control_mut() {
                ControlFlow::Break(stack) => {
                    return stack.pop().unwrap_or_default();
                }
                ControlFlow::Continue(()) => {}
            }
        }
    }
}

impl Executable for Switch {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let discriminant = executor.execute(nodes, self.discriminant, context);
        let discriminant: usize = int_from_float_checked(discriminant).unwrap_or_else(|| {
            panic!("Expected Switch discriminant to be valid usize, found {discriminant}")
        });

        let mut result: IRValue;

        for chunk in self.tests_and_consequents.chunks_exact(2) {
            let test = chunk[0];
            let consequent = chunk[1];
            result = executor.execute(nodes, test, context);
            let result: usize = int_from_float_checked(result)
                .unwrap_or_else(|| panic!("Expected SwitchWithDefault discriminant to be valid usize, found {discriminant}"));
            if result == discriminant {
                return executor.execute(nodes, consequent, context);
            }
        }

        0.0
    }
}

impl Executable for SwitchWithDefault {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let discriminant = executor.execute(nodes, self.discriminant, context);
        let discriminant = discriminant.round();
        let discriminant: usize = int_from_float_checked(discriminant).unwrap_or_else(|| {
            panic!(
                "Expected SwitchWithDefault discriminant to be valid usize, found {discriminant}"
            )
        });

        let mut result: IRValue;

        for chunk in self.tests_and_consequents.chunks_exact(2) {
            let test = chunk[0];
            let consequent = chunk[1];
            result = executor.execute(nodes, test, context);
            let result: usize = int_from_float_checked(result)
                .unwrap_or_else(|| panic!("Expected SwitchWithDefault discriminant to be valid usize, found {discriminant}"));
            if result == discriminant {
                return executor.execute(nodes, consequent, context);
            }
        }

        executor.execute(nodes, self.default_consequent, context)
    }
}

impl Executable for SwitchInteger {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let discriminant = executor.execute(nodes, self.discriminant, context);
        let discriminant: usize = int_from_float_checked(discriminant).unwrap_or_else(|| {
            panic!(
                "Expected SwitchIntegerWith discriminant to be valid usize, found {discriminant}"
            )
        });

        if let Some(consequent_index) = self.consequents.get(discriminant) {
            executor.execute(nodes, *consequent_index, context)
        } else {
            0.0
        }
    }
}

impl Executable for SwitchIntegerWithDefault {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let discriminant = executor.execute(nodes, self.discriminant, context);
        let discriminant = discriminant.round();
        let Some(discriminant) = int_from_float_checked::<usize>(discriminant) else {
            return executor.execute(nodes, self.default_consequent, context);
        };

        let consequent_index = *self
            .consequents
            .get(discriminant)
            .unwrap_or(&self.default_consequent);

        executor.execute(nodes, consequent_index, context)
    }
}
