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
    /// Duplicates the top of the stack
    Dup,

    /// Pops rhs and lhs, pushes 1.0 if lhs == rhs, 0.0 otherwise
    Equal,
    /// Pops rhs and lhs, pushes 1.0 if lhs < rhs, 0.0 otherwise
    Less,

    /// Pops rhs and lhs, performs addition, pushes the result
    Add,

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
    /// Log the value on top of the stack
    DebugPause,
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
    pub fn used_stack_indices(&self) -> UsedStackIndices {
        use Instruction::*;

        let mut map = HashMap::new();

        match self {
            Push(_) | Jump(_) | JumpTable { .. } | DebugPause => (),
            JumpZero(_) => {
                map.insert(0, "is_zero");
            }
            Pop | Dup | DebugLog => {
                map.insert(0, "value");
            }
            Equal | Less => {
                map.reserve(2);
                map.insert(0, "lhs");
                map.insert(1, "rhs");
            }
            Add => {
                map.reserve(2);
                map.insert(0, "a");
                map.insert(1, "b");
            }
            ReadMem => {
                map.reserve(2);
                map.insert(0, "block_id");
                map.insert(1, "index");
            }
            WriteMem => {
                map.reserve(3);
                map.insert(0, "block_id");
                map.insert(1, "index");
                map.insert(2, "value");
            }
        };
        UsedStackIndices(map.into_iter().map(|(k, v)| (k, v.to_owned())).collect())
    }

    /// Returns the mnemonic of this operand as a string for disassembly
    pub fn mnemonic(&self) -> &'static str {
        use Instruction::*;

        match self {
            Push(_) => "push",
            Pop => "pop",
            Dup => "dup",
            Equal => "eq",
            Less => "lt",
            Add => "add",
            Jump(_) => "jmp",
            JumpZero(_) => "jz",
            JumpTable { .. } => "jtbl",
            ReadMem => "rmem",
            WriteMem => "wmem",
            DebugLog => "dbgl",
            DebugPause => "dbgp",
        }
    }

    /// Returns the operands of this instruction for disassembly
    pub fn operands(&self) -> Vec<InstructionOperand> {
        use Instruction::*;

        let mut operands = vec![];
        match self {
            Pop | Dup | Equal | Less | Add | ReadMem | WriteMem | DebugLog | DebugPause => (),
            Push(value) => operands.push(InstructionOperand::Value(*value)),
            Jump(target) | JumpZero(target) => operands.push(InstructionOperand::Address(*target)),
            JumpTable {
                targets,
                default_target,
            } => {
                operands.push(InstructionOperand::Address(*default_target));
                operands.extend(targets.iter().map(|x| InstructionOperand::Address(*x)));
            }
        }
        operands
    }

    /// Returns the kind of this instruction
    pub fn kind(&self) -> InstructionKind {
        use Instruction::*;

        match self {
            Push(_) | Pop | Dup => InstructionKind::StackManipulation,
            ReadMem | WriteMem => InstructionKind::MemoryManipulation,
            Equal | Less => InstructionKind::Comparison,
            Jump(_) | JumpZero(_) | JumpTable { .. } => InstructionKind::Branching,
            Add => InstructionKind::Math,
            DebugLog | DebugPause => InstructionKind::Others,
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
