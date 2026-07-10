use rand::RngExt;
use sonorust_ir::{IRValue, modulo};

use crate::vm::VM;

impl VM {
    #[inline(always)]
    pub(crate) fn execute_add(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack.push(lhs + rhs);
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_divide(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack.push(lhs / rhs);
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_multiply(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack.push(lhs * rhs);
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_subtract(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack.push(lhs - rhs);
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_abs(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.abs());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_frac(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.fract());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_trunc(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.trunc());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_negate(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(-value);
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_mod(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack.push(modulo(lhs, rhs));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_rem(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack.push(lhs % rhs);
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_power(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack.push(lhs.powf(rhs));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_clamp(&mut self) {
        let b = self.pop_value("b");
        let a = self.pop_value("a");
        let x = self.pop_value("x");
        self.stack.push(x.clamp(a, b));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_lerp(&mut self) {
        let s = self.pop_value("s");
        let y = self.pop_value("y");
        let x = self.pop_value("x");
        self.stack.push(lerp(x, y, s));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_lerpclamped(&mut self) {
        let s = self.pop_value("s");
        let y = self.pop_value("y");
        let x = self.pop_value("x");
        self.stack.push(lerp(x, y, s.clamp(0.0, 1.0)));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_unlerp(&mut self) {
        let x = self.pop_value("x");
        let b = self.pop_value("b");
        let a = self.pop_value("a");
        self.stack.push(unlerp(x, a, b));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_unlerpclamped(&mut self) {
        let x = self.pop_value("x");
        let b = self.pop_value("b");
        let a = self.pop_value("a");
        self.stack.push(unlerp(x, a, b).clamp(0.0, 1.0));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_min(&mut self) {
        let y = self.pop_value("y");
        let x = self.pop_value("x");
        self.stack.push(x.min(y));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_max(&mut self) {
        let y = self.pop_value("y");
        let x = self.pop_value("x");
        self.stack.push(x.max(y));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_remap(&mut self) {
        let x = self.pop_value("x");
        let d = self.pop_value("d");
        let c = self.pop_value("c");
        let b = self.pop_value("b");
        let a = self.pop_value("a");
        self.stack.push(remap(x, a, b, c, d));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_remapclamped(&mut self) {
        let x = self.pop_value("x");
        let d = self.pop_value("d");
        let c = self.pop_value("c");
        let b = self.pop_value("b");
        let a = self.pop_value("a");
        self.stack.push(remap_clamped(x, a, b, c, d));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_round(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.round());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_floor(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.floor());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_ceil(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.ceil());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_sin(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.sin());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_sinh(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.sinh());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_cos(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.cos());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_cosh(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.cosh());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_tan(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.tan());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_tanh(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.tanh());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_arcsin(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.asin());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_arccos(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.acos());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_arctan(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.atan());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_arctan2(&mut self) {
        let x = self.pop_value("x");
        let y = self.pop_value("y");
        self.stack.push(y.atan2(x));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_degree(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.to_degrees());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_radian(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.to_radians());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_log(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(value.ln());
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_sign(&mut self) {
        let value = self.pop_value("value");

        let sign = {
            if value.is_nan() {
                value
            } else if value == 0.0 {
                0.0
            } else if value.is_sign_positive() {
                1.0
            } else {
                -1.0
            }
        };
        self.stack.push(sign);
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_random(&mut self) {
        let max = self.pop_value("max");
        let min = self.pop_value("min");
        self.stack.push(rand::rng().random_range(min..max));
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_randominteger(&mut self) {
        let max = self.pop_value("max");
        let min = self.pop_value("min");
        self.stack
            .push(rand::rng().random_range(min..max).round() as IRValue);
        self.pc += 1;
    }
}

#[inline(always)]
fn lerp(x: IRValue, y: IRValue, s: IRValue) -> IRValue {
    x + (y - x) * s
}

#[inline(always)]
fn unlerp(x: IRValue, a: IRValue, b: IRValue) -> IRValue {
    let denom = b - a;
    if denom == 0.0 { 0.0 } else { (x - a) / denom }
}

#[inline(always)]
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

#[inline(always)]
fn remap_clamped(
    value: IRValue,
    in_min: IRValue,
    in_max: IRValue,
    out_min: IRValue,
    out_max: IRValue,
) -> IRValue {
    if in_max == in_min {
        return out_min;
    }

    lerp(
        unlerp(value, in_min, in_max).clamp(0.0, 1.0),
        out_min,
        out_max,
    )
}
