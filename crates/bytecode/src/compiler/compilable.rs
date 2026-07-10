use sonorust_ir::nodes::{IRNode, OpCode};

use crate::{compiler::Compiler, instruction::Instruction};

pub trait Compilable {
    fn compile(&self, compiler: &mut Compiler);
}

impl Compilable for IRNode {
    fn compile(&self, compiler: &mut Compiler) {
        use OpCode::*;

        match self {
            IRNode::Value(val) => compiler.add_inst(Instruction::Push(*val)),
            IRNode::OpCode(op_code) => match op_code {
                // Control Flow
                Execute(node) => node.compile(compiler),
                Execute0(node) => node.compile(compiler),
                Block(node) => node.compile(compiler),
                Break(node) => node.compile(compiler),
                If(node) => node.compile(compiler),
                While(node) => node.compile(compiler),
                Switch(node) => node.compile(compiler),
                SwitchWithDefault(node) => node.compile(compiler),
                SwitchInteger(node) => node.compile(compiler),
                SwitchIntegerWithDefault(node) => node.compile(compiler),
                JumpLoop(node) => node.compile(compiler),

                // Debug
                DebugLog(node) => node.compile(compiler),
                DebugPause(node) => node.compile(compiler),

                // Logical
                Equal(node) => node.compile(compiler),
                NotEqual(node) => node.compile(compiler),
                Greater(node) => node.compile(compiler),
                GreaterOr(node) => node.compile(compiler),
                Less(node) => node.compile(compiler),
                LessOr(node) => node.compile(compiler),
                And(node) => node.compile(compiler),
                Or(node) => node.compile(compiler),
                Not(node) => node.compile(compiler),

                // Math
                Add(node) => node.compile(compiler),
                Divide(node) => node.compile(compiler),
                Multiply(node) => node.compile(compiler),
                Subtract(node) => node.compile(compiler),

                Abs(node) => node.compile(compiler),
                Frac(node) => node.compile(compiler),
                Trunc(node) => node.compile(compiler),
                Negate(node) => node.compile(compiler),

                Mod(node) => node.compile(compiler),
                Rem(node) => node.compile(compiler),
                Power(node) => node.compile(compiler),

                Clamp(node) => node.compile(compiler),
                Lerp(node) => node.compile(compiler),
                LerpClamped(node) => node.compile(compiler),
                Unlerp(node) => node.compile(compiler),
                UnlerpClamped(node) => node.compile(compiler),
                Min(node) => node.compile(compiler),
                Max(node) => node.compile(compiler),
                Remap(node) => node.compile(compiler),
                RemapClamped(node) => node.compile(compiler),

                Round(node) => node.compile(compiler),
                Floor(node) => node.compile(compiler),
                Ceil(node) => node.compile(compiler),

                Sin(node) => node.compile(compiler),
                Sinh(node) => node.compile(compiler),
                Cos(node) => node.compile(compiler),
                Cosh(node) => node.compile(compiler),
                Tan(node) => node.compile(compiler),
                Tanh(node) => node.compile(compiler),
                Arcsin(node) => node.compile(compiler),
                Arccos(node) => node.compile(compiler),
                Arctan(node) => node.compile(compiler),
                Arctan2(node) => node.compile(compiler),
                Degree(node) => node.compile(compiler),
                Radian(node) => node.compile(compiler),

                Log(node) => node.compile(compiler),
                Sign(node) => node.compile(compiler),
                Random(node) => node.compile(compiler),
                RandomInteger(node) => node.compile(compiler),

                // Memory
                Get(node) => node.compile(compiler),
                GetPointed(node) => node.compile(compiler),
                GetShifted(node) => node.compile(compiler),
                Set(node) => node.compile(compiler),
                SetPointed(node) => node.compile(compiler),
                SetShifted(node) => node.compile(compiler),
                SetAdd(node) => node.compile(compiler),
                SetAddPointed(node) => node.compile(compiler),
                SetAddShifted(node) => node.compile(compiler),
                SetSubtract(node) => node.compile(compiler),
                SetSubtractPointed(node) => node.compile(compiler),
                SetSubtractShifted(node) => node.compile(compiler),
                SetMultiply(node) => node.compile(compiler),
                SetMultiplyPointed(node) => node.compile(compiler),
                SetMultiplyShifted(node) => node.compile(compiler),
                SetDivide(node) => node.compile(compiler),
                SetDividePointed(node) => node.compile(compiler),
                SetDivideShifted(node) => node.compile(compiler),
                SetMod(node) => node.compile(compiler),
                SetModPointed(node) => node.compile(compiler),
                SetModShifted(node) => node.compile(compiler),
                SetPower(node) => node.compile(compiler),
                SetPowerPointed(node) => node.compile(compiler),
                SetPowerShifted(node) => node.compile(compiler),
                SetRem(node) => node.compile(compiler),
                SetRemPointed(node) => node.compile(compiler),
                SetRemShifted(node) => node.compile(compiler),
                Copy(node) => node.compile(compiler),
                Spawn(node) => node.compile(compiler),
                Draw(node) => node.compile(compiler),
                BeatToTime(node) => node.compile(compiler),

                HasEffectClip(node) => node.compile(compiler),
                Play(node) => node.compile(compiler),
                PlayLooped(node) => node.compile(compiler),
                PlayLoopedScheduled(node) => node.compile(compiler),
                PlayScheduled(node) => node.compile(compiler),
                StopLooped(node) => node.compile(compiler),
                StopLoopedScheduled(node) => node.compile(compiler),
                DestroyParticleEffect(node) => node.compile(compiler),
                HasParticleEffect(node) => node.compile(compiler),
                SpawnParticleEffect(node) => node.compile(compiler),
                Judge(node) => node.compile(compiler),
                ExportValue(node) => node.compile(compiler),
                StreamSet(node) => node.compile(compiler),
                #[allow(unreachable_patterns)]
                other => todo!("{:?} is not yet implemented", other),
            },
        }
    }
}

macro_rules! todo_node {
    ($compiler:ident,) => {};
    ($compiler:ident, $($name:ident),+ => $val:expr; $($rest:tt)*) => {
        $(
            impl Compilable for $name {
                fn compile(&self, $compiler: &mut super::Compiler) {
                    tracing::warn!("Compiling unimplemented node: {}", stringify!($name));
                    $compiler.add_inst(Instruction::Push($val));
                }
            }
        )*
        todo_node!($compiler, $($rest)*);
    };
}

use sonorust_ir::nodes::*;

todo_node!(compiler,
    GetPointed, SetPointed, SetShifted, SetAddPointed, SetAddShifted,
    SetSubtract, SetSubtractPointed, SetSubtractShifted,
    SetMultiplyPointed,SetMultiplyShifted,
    SetDivide, SetDividePointed, SetDivideShifted,
    SetMod, SetModPointed, SetModShifted,
    SetPower, SetPowerPointed, SetPowerShifted,
    SetRem, SetRemPointed,SetRemShifted,
    Copy,
    Play, PlayScheduled, StopLooped, StopLoopedScheduled,
    DestroyParticleEffect, Judge, ExportValue, StreamSet => 0.0;
    HasEffectClip, HasParticleEffect, PlayLooped, PlayLoopedScheduled, SpawnParticleEffect => 1.0;
);
