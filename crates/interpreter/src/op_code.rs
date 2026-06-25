use sonorust_ir::{
    IRValue,
    nodes::{OpCode, IRNode},
};
use sonorust_runtime::context::RuntimeContext;

use crate::{Executable, SonorustInterpreter};

mod control_flow;
mod logical;
mod math;
mod memory;

macro_rules! match_opcodes {
    ($val:expr, $ctx:expr, $nodes:expr, $exec:expr; $($variant:ident),* $(,)?) => {
        #[allow(unused)] // sometimes they're all implemented :D
        match $val {
            $(
                OpCode::$variant(node) => node.execute($ctx, $nodes, $exec),
            )*
            other => todo!("OpCode {:?} not yet implemented", other),
        }
    };
}

impl Executable for OpCode {
    fn execute(
        &self,
        context: &mut RuntimeContext,
        nodes: &[IRNode],
        executor: &mut SonorustInterpreter,
    ) -> IRValue {
        match_opcodes!(
            self, context, nodes, executor;

            Execute,
            Execute0,
            Block,
            Break,
            If,
            While,
            Switch,
            SwitchWithDefault,
            SwitchInteger,
            SwitchIntegerWithDefault,

            Abs,
            Frac,
            Trunc,
            Negate,
            Add,
            Subtract,
            Multiply,
            Divide,
            Mod,
            Rem,
            Power,
            Clamp,
            Lerp,
            LerpClamped,
            Unlerp,
            UnlerpClamped,
            Min,
            Max,
            Remap,
            RemapClamped,
            Round,
            Floor,
            Ceil,
            Sin,
            Sinh,
            Cos,
            Cosh,
            Tan,
            Tanh,
            Arcsin,
            Arccos,
            Arctan,
            Arctan2,
            Degree,
            Radian,
            Log,
            Sign,
            Random,
            RandomInteger,

            Equal,
            NotEqual,
            Greater,
            GreaterOr,
            Less,
            LessOr,
            And,
            Or,
            Not,

            Copy,
            Get,
            GetPointed,
            GetShifted,
            Set,
            SetPointed,
            SetShifted,
            SetAdd,
            SetAddPointed,
            SetAddShifted,
            SetDivide,
            SetDividePointed,
            SetDivideShifted,
            SetMultiply,
            SetMultiplyPointed,
            SetMultiplyShifted,
            SetMod,
            SetModPointed,
            SetModShifted,
            SetRem,
            SetRemPointed,
            SetRemShifted,
            SetPower,
            SetPowerPointed,
            SetPowerShifted,
            SetSubtract,
            SetSubtractPointed,
            SetSubtractShifted,
        )
    }
}
