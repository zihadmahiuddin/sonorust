use std::{collections::HashMap, fmt::Display};

use sonorust_ir::IRValue;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum Instruction {
    /// Pushes the given value on top of the stack
    Push(IRValue),
    /// Pops the top of the stack
    Pop,
    /// Duplicates the specified amount of items from the specified offset of the stack (0 = top)
    Dup { offset: usize, count: usize },

    /// Pops rhs, lhs, pushes 1.0 if lhs == rhs, 0.0 otherwise
    Equal,
    /// Pops rhs, lhs, pushes 1.0 if lhs < rhs, 0.0 otherwise
    Less,
    /// Pops rhs, lhs, pushes 1.0 if lhs <= rhs, 0.0 otherwise
    LessOr,
    /// Pops rhs, lhs, pushes 1.0 if lhs > rhs, 0.0 otherwise
    Greater,
    /// Pops rhs, lhs, pushes 1.0 if lhs >= rhs, 0.0 otherwise
    GreaterOr,
    /// Pops rhs, lhs, pushes 1.0 if lhs != 0.0 && rhs != 0.0, 0.0 otherwise
    And,
    /// Pops rhs, lhs, pushes 1.0 if lhs != 0.0 || rhs != 0.0, 0.0 otherwise
    Or,
    /// Pops value, pushes 1.0 if value == 0.0, 0.0 otherwise
    Not,

    /// Pops rhs, lhs, performs addition, pushes the result
    Add,
    /// Pops rhs, lhs, performs division, pushes the result
    Divide,
    /// Pops rhs, lhs, performs multiplication, pushes the result
    Multiply,
    /// Pops rhs, lhs, performs subtraction, pushes the result
    Subtract,
    /// Pops value, pushes value.abs()
    Abs,
    /// Pops value, pushes value.fract()
    Frac,
    /// Pops value, pushes value.trunc()
    Trunc,
    /// Pops value, pushes value.neg()
    Negate,
    /// Pops rhs, lhs, pushes modulo(lhs, rhs)
    Mod,
    /// Pops rhs, lhs, pushes lhs % rhs
    Rem,
    /// Pops rhs, lhs, pushes lhs ^ rhs
    Power,
    /// Pops value, max, min. Pushes value.clamp(min, max)
    Clamp,
    /// Pops weight, end, start. Pushes start.lerp(end, weight)
    Lerp,
    /// Pops weight, end, start. Pushes start.lerp(end, weight.clamp(0.0, 1.0))
    LerpClamped,
    /// Pops value, end, start. Pushes start.unlerp(end, value)
    Unlerp,
    /// Pops value, end, start. Pushes start.unlerp(end, value).clamp(0.0, 1.0)
    UnlerpClamped,
    /// Pops rhs, lhs. Pushes lhs.min(rhs)
    Min,
    /// Pops rhs, lhs. Pushes lhs.max(rhs)
    Max,
    /// Pops x, to_end, to_start, from_end, from_start. Pushes x.remap(...)
    Remap,
    /// Pops x, to_end, to_start, from_end, from_start. Pushes x.remap(...) clamped
    RemapClamped,
    /// Pops value, pushes value.round()
    Round,
    /// Pops value, pushes value.floor()
    Floor,
    /// Pops value, pushes value.ceil()
    Ceil,
    /// Pops value, pushes value.sin()
    Sin,
    /// Pops value, pushes value.sinh()
    Sinh,
    /// Pops value, pushes value.cos()
    Cos,
    /// Pops value, pushes value.cosh()
    Cosh,
    /// Pops value, pushes value.tan()
    Tan,
    /// Pops value, pushes value.tanh()
    Tanh,
    /// Pops value, pushes value.asin()
    Arcsin,
    /// Pops value, pushes value.acos()
    Arccos,
    /// Pops value, pushes value.atan()
    Arctan,
    /// Pops y, x, pushes y.atan2(x)
    Arctan2,
    /// Pops value, pushes value.to_degrees()
    Degree,
    /// Pops value, pushes value.to_radians()
    Radian,
    /// Pops value, pushes value.ln()
    Log,
    /// Pops value, pushes 1.0 if value > 0.0, -1.0 if value < 0, 0.0 if value == 0.0, NaN otherwise
    Sign,
    /// Pops max, min, pushes rand(min, max) (both inclusive)
    Random,
    /// Pops max, min, pushes rand(min, max).floor() (min inclusive, max exclusive)
    RandomInteger,

    /// Pops z4?, z3? z2?, alpha, z1, y4, x4, y3, x3, y2, x2, y1, x1, sprite_id, pushes 0
    Draw,
    /// Pops archetype_id, data_len and data_len amount of values, pushes 0
    Spawn,

    ///Pops beat, pushes the time at the given beat
    BeatToTime,

    /// Unconditionally jumps to given address
    Jump(usize),
    /// Pops the top of the stack, jumps to the given address if 0.0, does nothing otherwise
    JumpZero(usize),
    JumpTable {
        targets: Vec<usize>,
        default_target: usize,
    },

    /// Pops index and block_id, performs a memory read
    ReadMem,
    /// Pops value, index and block_id, performs a memory write
    WriteMem,

    /// Log the value on top of the stack
    DebugLog,
    /// Pauses the VM
    Pause,
    /// Halts the VM
    Halt,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct UsedStackIndices(pub HashMap<usize, String>);

impl Instruction {
    /// Returns true if this opcode leaves a result on the stack
    pub fn produces_result(&self) -> bool {
        !matches!(
            self,
            Instruction::Jump(_) | Instruction::JumpZero(_) | Instruction::JumpTable { .. }
        )
    }

    /// Returns the values that this instruction would consume from the stack
    pub fn used_stack_indices(&self, stack: &[IRValue]) -> UsedStackIndices {
        use Instruction::*;

        let mut map = HashMap::<usize, String>::new();

        match self {
            Push(_) | Jump(_) | JumpTable { .. } | Pause | Halt => (),
            Dup { count, offset } => {
                map.reserve(*count.min(&stack.len()));
                for i in 0..*count {
                    map.insert(offset + i, format!("dup_value_{i}"));
                }
            }
            JumpZero(_) => {
                map.insert(0, "is_zero".to_owned());
            }
            Pop | DebugLog | Not | Abs | Frac | Trunc | Negate | Round | Floor | Ceil | Sin
            | Sinh | Cos | Cosh | Tan | Tanh | Arcsin | Arccos | Arctan | Degree | Radian | Log
            | Sign | BeatToTime => {
                map.insert(0, "value".to_owned());
            }
            Equal | Less | LessOr | Greater | GreaterOr | Add | Divide | Multiply | Subtract
            | And | Or | Mod | Rem | Power => {
                map.reserve(2);
                map.insert(0, "lhs".to_owned());
                map.insert(1, "rhs".to_owned());
            }
            ReadMem => {
                map.reserve(2);
                map.insert(0, "block_id".to_owned());
                map.insert(1, "index".to_owned());
            }
            WriteMem => {
                map.reserve(3);
                map.insert(0, "block_id".to_owned());
                map.insert(1, "index".to_owned());
                map.insert(2, "value".to_owned());
            }
            Clamp => {
                map.reserve(3);
                map.insert(0, "value".to_owned());
                map.insert(1, "min".to_owned());
                map.insert(2, "max".to_owned());
            }
            Lerp | LerpClamped => {
                map.reserve(3);
                map.insert(0, "x".to_owned());
                map.insert(1, "y".to_owned());
                map.insert(2, "s".to_owned());
            }
            Unlerp | UnlerpClamped => {
                map.reserve(3);
                map.insert(0, "a".to_owned());
                map.insert(1, "b".to_owned());
                map.insert(2, "x".to_owned());
            }
            Min | Max | Arctan2 => {
                map.reserve(2);
                map.insert(0, "x".to_owned());
                map.insert(1, "y".to_owned());
            }
            Remap | RemapClamped => {
                map.reserve(5);
                map.insert(0, "a".to_owned());
                map.insert(1, "b".to_owned());
                map.insert(2, "c".to_owned());
                map.insert(3, "d".to_owned());
                map.insert(4, "x".to_owned());
            }
            Random | RandomInteger => {
                map.reserve(2);
                map.insert(0, "min".to_owned());
                map.insert(1, "max".to_owned());
            }
            Draw => {
                map.reserve(14);
                map.insert(0, "sprite_id".to_owned());
                map.insert(1, "x1".to_owned());
                map.insert(2, "y1".to_owned());
                map.insert(3, "x2".to_owned());
                map.insert(4, "y2".to_owned());
                map.insert(5, "x3".to_owned());
                map.insert(6, "y3".to_owned());
                map.insert(7, "x4".to_owned());
                map.insert(8, "y4".to_owned());
                map.insert(9, "z1".to_owned());
                map.insert(10, "alpha".to_owned());
                if stack.len() >= 12 {
                    map.insert(11, "z2".to_owned());
                }
                if stack.len() >= 13 {
                    map.insert(12, "z3".to_owned());
                }
                if stack.len() >= 12 {
                    map.insert(13, "z4".to_owned());
                }
            }
            Spawn => {
                map.reserve(2);
                map.insert(0, "archetype_id".to_owned());
                map.insert(1, "data_count".to_owned());
                if let Some(data_count) = stack.iter().rev().nth(2) {
                    for i in 0..(*data_count as usize) {
                        map.insert(1, format!("spawn_data_{i}"));
                    }
                }
            }
        };
        UsedStackIndices(map.into_iter().collect())
    }

    /// Returns the mnemonic of this operand as a string for disassembly
    pub fn mnemonic(&self) -> &'static str {
        use Instruction::*;

        match self {
            Push(_) => "push",
            Pop => "pop",
            Dup { .. } => "dup",
            Equal => "eq",
            Less => "lt",
            LessOr => "lte",
            Greater => "gt",
            GreaterOr => "gte",
            And => "and",
            Or => "or",
            Not => "not",
            Add => "add",
            Divide => "div",
            Multiply => "mul",
            Subtract => "sub",
            Jump(_) => "jmp",
            JumpZero(_) => "jz",
            JumpTable { .. } => "jtbl",
            ReadMem => "rmem",
            WriteMem => "wmem",
            DebugLog => "dbgl",
            Pause => "dbgp",
            Abs => "abs",
            Frac => "frac",
            Trunc => "trunc",
            Negate => "neg",
            Mod => "mod",
            Rem => "rem",
            Power => "pow",
            Clamp => "clamp",
            Lerp => "lerp",
            LerpClamped => "lerpc",
            Unlerp => "ulerp",
            UnlerpClamped => "ulerpc",
            Min => "min",
            Max => "max",
            Remap => "remap",
            RemapClamped => "remapc",
            Round => "round",
            Floor => "floor",
            Ceil => "ceil",
            Sin => "sin",
            Sinh => "sinh",
            Cos => "cos",
            Cosh => "cosh",
            Tan => "tan",
            Tanh => "tanh",
            Arcsin => "asin",
            Arccos => "acos",
            Arctan => "atan",
            Arctan2 => "atan2",
            Degree => "deg",
            Radian => "rad",
            Log => "ln",
            Sign => "sign",
            Random => "rand",
            RandomInteger => "randi",
            Draw => "draw",
            Spawn => "spawn",
            BeatToTime => "beattotime",
            Halt => "hlt",
        }
    }

    /// Returns the operands of this instruction for disassembly
    pub fn operands(&self) -> Vec<InstructionOperand> {
        use Instruction::*;

        let mut operands = vec![];
        match self {
            Push(value) => operands.push(InstructionOperand::Value(*value)),
            Dup { count, offset } => {
                operands.push(InstructionOperand::Value(*count as IRValue));
                operands.push(InstructionOperand::Address(*offset));
            }
            Jump(target) | JumpZero(target) => operands.push(InstructionOperand::Address(*target)),
            JumpTable {
                targets,
                default_target,
            } => {
                operands.push(InstructionOperand::Address(*default_target));
                operands.extend(targets.iter().map(|x| InstructionOperand::Address(*x)));
            }
            _ => (),
        }
        operands
    }

    /// Returns the kind of this instruction
    pub fn kind(&self) -> InstructionKind {
        use Instruction::*;

        match self {
            Push(_) | Pop | Dup { .. } => InstructionKind::StackManipulation,
            WriteMem => InstructionKind::MemoryManipulation,
            Equal | Less | LessOr | Greater | GreaterOr | And | Or | Not => {
                InstructionKind::Comparison
            }
            Jump(_) | JumpZero(_) | JumpTable { .. } => InstructionKind::Branching,
            Add | Divide | Multiply | Subtract | Abs | Frac | Trunc | Negate | Mod | Rem
            | Power | Clamp | Lerp | LerpClamped | Unlerp | UnlerpClamped | Min | Max | Remap
            | RemapClamped | Round | Floor | Ceil | Sin | Sinh | Cos | Cosh | Tan | Tanh
            | Arcsin | Arccos | Arctan | Arctan2 | Degree | Radian | Log | Sign | Random
            | RandomInteger => InstructionKind::Math,
            DebugLog | Pause | Halt | ReadMem | BeatToTime => InstructionKind::Others,
            Draw | Spawn => InstructionKind::SideEffects,
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mnemonic = self.mnemonic();
        let operands = self.operands();

        write!(f, "{mnemonic}")?;

        if !operands.is_empty() {
            for (i, operand) in operands.iter().enumerate() {
                if i != operands.len() - 1 {
                    write!(f, " ")?;
                }
                write!(f, " {operand}")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub enum InstructionKind {
    StackManipulation,
    MemoryManipulation,
    Comparison,
    Branching,
    Math,
    SideEffects,
    Others,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum InstructionOperand {
    Value(IRValue),
    Address(usize),
}

impl Display for InstructionOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionOperand::Value(value) => write!(f, "{value}"),
            InstructionOperand::Address(addr) => write!(f, "0x{:02x}", addr),
        }
    }
}
