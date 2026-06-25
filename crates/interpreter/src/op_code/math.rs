use rand::Rng;
use sonorust_ir::{IRValue, modulo, nodes::*};
use sonorust_runtime::{SonorustIRExecutor, context::RuntimeContext};

use crate::{Executable, SonorustInterpreter};

impl Executable for Abs {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        executor.execute(nodes, self.value, context).abs()
    }
}

impl Executable for Frac {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        executor.execute(nodes, self.value, context).fract()
    }
}

impl Executable for Trunc {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        executor.execute(nodes, self.value, context).trunc()
    }
}

impl Executable for Negate {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        -executor.execute(nodes, self.value, context)
    }
}

impl Executable for Add {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        self.inputs
            .iter()
            .map(|idx| executor.execute(nodes, *idx, context))
            .sum()
    }
}

impl Executable for Subtract {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let mut inputs = self.inputs.iter();

        let Some(first) = inputs.next() else {
            return 0.0;
        };

        let init = executor.execute(nodes, *first, context);
        inputs.fold(init, |acc, &idx| {
            acc - executor.execute(nodes, idx, context)
        })
    }
}

impl Executable for Multiply {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        self.inputs
            .iter()
            .map(|idx| executor.execute(nodes, *idx, context))
            .product()
    }
}

impl Executable for Divide {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let mut inputs = self.inputs.iter();

        let Some(first) = inputs.next() else {
            return 0.0;
        };

        let init = executor.execute(nodes, *first, context);
        inputs.fold(init, |acc, &idx| {
            acc / executor.execute(nodes, idx, context)
        })
    }
}

impl Executable for Mod {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let mut inputs = self.inputs.iter();

        let Some(first) = inputs.next() else {
            return 0.0;
        };

        let init = executor.execute(nodes, *first, context);
        inputs.fold(init, |acc, &idx| {
            modulo(acc, executor.execute(nodes, idx, context))
        })
    }
}

impl Executable for Rem {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let mut inputs = self.inputs.iter();

        let Some(first) = inputs.next() else {
            return 0.0;
        };

        let init = executor.execute(nodes, *first, context);
        inputs.fold(init, |acc, &idx| {
            acc % executor.execute(nodes, idx, context)
        })
    }
}

impl Executable for Power {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        if self.inputs.is_empty() {
            return 0.0;
        }

        let values: Vec<f32> = self
            .inputs
            .iter()
            .map(|&idx| executor.execute(nodes, idx, context))
            .collect();

        let mut iter = values.into_iter();
        let first = iter.next().unwrap();

        iter.fold(first, |acc, exponent| acc.powf(exponent))
    }
}

impl Executable for Clamp {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let min = executor.execute(nodes, self.min, context);
        let max = executor.execute(nodes, self.max, context);

        let value = executor.execute(nodes, self.value, context);
        value.clamp(min, max)
    }
}

impl Executable for Lerp {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let min = executor.execute(nodes, self.min, context);
        let max = executor.execute(nodes, self.max, context);
        let value = executor.execute(nodes, self.value, context);
        lerp(value, min, max)
    }
}

impl Executable for LerpClamped {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let min = executor.execute(nodes, self.min, context);
        let max = executor.execute(nodes, self.max, context);
        let value = executor.execute(nodes, self.value, context);
        lerp(value.clamp(0.0, 1.0), min, max)
    }
}

impl Executable for Unlerp {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let min = executor.execute(nodes, self.min, context);
        let max = executor.execute(nodes, self.max, context);
        let value = executor.execute(nodes, self.value, context);
        unlerp(value, min, max)
    }
}

impl Executable for UnlerpClamped {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let min = executor.execute(nodes, self.min, context);
        let max = executor.execute(nodes, self.max, context);
        let value = executor.execute(nodes, self.value, context);
        unlerp(value, min, max).clamp(0.0, 1.0)
    }
}

impl Executable for Min {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let x = executor.execute(nodes, self.x, context);
        let y = executor.execute(nodes, self.y, context);
        x.min(y)
    }
}

impl Executable for Max {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let x = executor.execute(nodes, self.x, context);
        let y = executor.execute(nodes, self.y, context);
        x.max(y)
    }
}

impl Executable for Remap {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let from_min = executor.execute(nodes, self.from_min, context);
        let from_max = executor.execute(nodes, self.from_max, context);
        let to_min = executor.execute(nodes, self.to_min, context);
        let to_max = executor.execute(nodes, self.to_max, context);
        let value = executor.execute(nodes, self.value, context);
        remap(value, from_min, from_max, to_min, to_max)
    }
}

impl Executable for RemapClamped {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let from_min = executor.execute(nodes, self.from_min, context);
        let from_max = executor.execute(nodes, self.from_max, context);
        let to_min = executor.execute(nodes, self.to_min, context);
        let to_max = executor.execute(nodes, self.to_max, context);
        let value = executor.execute(nodes, self.value, context);

        let actual_min = from_min.min(from_max);
        let actual_max = from_min.max(from_max);
        let clamped_value = value.clamp(actual_min, actual_max);

        remap(clamped_value, from_min, from_max, to_min, to_max)
    }
}

impl Executable for Round {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.round_ties_even()
    }
}

impl Executable for Floor {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.floor()
    }
}

impl Executable for Ceil {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.ceil()
    }
}

impl Executable for Sin {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.sin()
    }
}

impl Executable for Sinh {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.sinh()
    }
}

impl Executable for Cos {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.cos()
    }
}

impl Executable for Cosh {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.cosh()
    }
}

impl Executable for Tan {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.tan()
    }
}

impl Executable for Tanh {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.tanh()
    }
}

impl Executable for Arcsin {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.asin()
    }
}

impl Executable for Arccos {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.acos()
    }
}

impl Executable for Arctan {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.atan()
    }
}

impl Executable for Arctan2 {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let x = executor.execute(nodes, self.x, context);
        let y = executor.execute(nodes, self.y, context);
        y.atan2(x)
    }
}

impl Executable for Degree {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let radians = executor.execute(nodes, self.value, context);
        radians.to_degrees()
    }
}

impl Executable for Radian {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let degrees = executor.execute(nodes, self.value, context);
        degrees.to_radians()
    }
}

impl Executable for Log {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);
        value.ln()
    }
}

impl Executable for Sign {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let value = executor.execute(nodes, self.value, context);

        if value.is_nan() {
            return value;
        }

        if value == 0.0 {
            return 0.0;
        }

        if value.is_sign_positive() {
            return 1.0;
        }

        -1.0
    }
}

impl Executable for Random {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let min = executor.execute(nodes, self.min, context);
        let max = executor.execute(nodes, self.max, context);
        rand::rng().random_range(min..=max)
    }
}

impl Executable for RandomInteger {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        let min = executor.execute(nodes, self.min, context);
        let max = executor.execute(nodes, self.max, context);
        let min = min.round() as i64;
        let max = max.round() as i64;
        rand::rng().random_range(min..max) as IRValue
    }
}

fn lerp(value: IRValue, min: IRValue, max: IRValue) -> IRValue {
    (1.0 - value) * min + value * max
}

fn unlerp(value: IRValue, min: IRValue, max: IRValue) -> IRValue {
    let denominator = max - min;

    if denominator.abs() < IRValue::EPSILON {
        return 0.0;
    }

    (value - min) / denominator
}

fn remap(
    value: IRValue,
    in_min: IRValue,
    in_max: IRValue,
    out_min: IRValue,
    out_max: IRValue,
) -> IRValue {
    if in_max == in_min {
        return out_min;
    }

    lerp(unlerp(value, in_min, in_max), out_min, out_max)
}
