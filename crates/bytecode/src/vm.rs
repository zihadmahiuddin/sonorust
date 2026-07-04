use std::collections::HashSet;

use sonorust_ir::IRValue;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sonorust_runtime::context::RuntimeContext;

use crate::instruction::Instruction;

pub struct VM {
    pub stack: Vec<IRValue>,
    pub pc: usize,
    pub breakpoints: HashSet<usize>,
    pub instructions: Vec<Instruction>,
    pub state: VMState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub enum VMState {
    Stopped,
    Running,
    Paused,
    Done,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            pc: 0,
            breakpoints: HashSet::new(),
            instructions: vec![],
            state: VMState::Stopped,
        }
    }

    pub fn load_bytecode(&mut self, bytecode: &[Instruction]) {
        self.state = VMState::Stopped;
        self.stack.clear();
        self.instructions.clear();
        self.instructions.extend(bytecode.iter().cloned());
    }

    pub fn pause(&mut self) {
        if self.state == VMState::Running {
            self.state = VMState::Paused;
        }
    }

    pub fn resume(&mut self) {
        let was_stopped = self.state == VMState::Stopped;

        if self.state == VMState::Stopped || self.state == VMState::Paused {
            self.state = VMState::Running;
        }

        // handle breakpoint at initial pc
        if was_stopped && self.breakpoints.contains(&self.pc) {
            self.state = VMState::Paused;
        }
    }

    pub fn stop(&mut self) {
        self.pc = 0;
        self.state = VMState::Stopped;
        self.stack.clear();
    }

    /// Executes instructions up to `max_steps`, but yields EARLY
    /// if the VM hits a pause, breakpoint, or halt instruction.
    pub fn run(&mut self, runtime_ctx: &mut RuntimeContext, max_steps: usize) -> VMState {
        if self.state != VMState::Running {
            return self.state;
        }

        for _ in 0..max_steps {
            let new_state = self.step(runtime_ctx);

            // If the step changed our state (e.g., hit a Pause instruction),
            // break the loop and return control to the caller
            if new_state != VMState::Running {
                break;
            }
        }

        self.state
    }

    pub fn step(&mut self, runtime_ctx: &mut RuntimeContext) -> VMState {
        if self.state != VMState::Running && self.state != VMState::Paused {
            return self.state;
        }

        if self.pc >= self.instructions.len() {
            self.state = VMState::Done;
            return self.state;
        }

        let originally_paused = self.state == VMState::Paused;
        if originally_paused {
            self.state = VMState::Running;
        }

        let inst = &self.instructions[self.pc];

        match inst {
            Instruction::Push(val) => {
                self.stack.push(*val);
                self.pc += 1;
            }
            Instruction::Pop => {
                self.stack.pop().expect("Stack underflow on Pop");
                self.pc += 1;
            }
            Instruction::Dup => {
                let val = *self.stack.last().expect("Stack underflow on Dup operation");
                self.stack.push(val);
                self.pc += 1;
            }
            Instruction::Add => {
                let b = self.stack.pop().expect("Stack underflow on Add (rhs)");
                let a = self.stack.pop().expect("Stack underflow on Add (lhs)");
                self.stack.push(a + b);
                self.pc += 1;
            }
            Instruction::Equal => {
                let b = self.stack.pop().expect("Stack underflow on Equal (rhs)");
                let a = self.stack.pop().expect("Stack underflow on Equal (lhs)");
                self.stack.push(if a == b { 1.0 } else { 0.0 });
                self.pc += 1;
            }
            Instruction::Less => {
                let b = self.stack.pop().expect("Stack underflow on Less (rhs)");
                let a = self.stack.pop().expect("Stack underflow on Less (lhs)");
                let result = if a < b { 1.0 } else { 0.0 };
                self.stack.push(result);
                self.pc += 1;
            }
            Instruction::ReadMem => {
                // TODO: maybe instead of rounding it's better to do strict checks... that's for future me to worry about
                let index = self
                    .stack
                    .pop()
                    .expect("Stack underflow on Get (index)")
                    .round() as usize;
                let block_id = self
                    .stack
                    .pop()
                    .expect("Stack underflow on Get (block_id)")
                    .round() as u64;
                let value = runtime_ctx
                    .memory
                    .read(&runtime_ctx, block_id, index)
                    .unwrap_or_default();
                self.stack.push(value);
                self.pc += 1;
            }
            Instruction::WriteMem => {
                let value = self.stack.pop().expect("Stack underflow on Set (value)");
                let index = self
                    .stack
                    .pop()
                    .expect("Stack underflow on Set (index)")
                    .round() as usize;
                let block_id = self
                    .stack
                    .pop()
                    .expect("Stack underflow on Set (block_id)")
                    .round() as u64;
                let value = runtime_ctx
                    .memory
                    .write(&runtime_ctx, block_id, index, value)
                    .unwrap_or_default();
                self.stack.push(value);
                self.pc += 1;
            }
            Instruction::Jump(target) => {
                self.pc = *target;
            }
            Instruction::JumpZero(target) => {
                let cond = self.stack.pop().expect("Stack underflow on JumpIfFalse");
                if cond == 0.0 {
                    self.pc = *target;
                } else {
                    self.pc += 1;
                }
            }
            Instruction::JumpTable {
                targets,
                default_target,
            } => {
                let index_val = self
                    .stack
                    .pop()
                    .expect("Stack underflow reading JumpTable key");
                let index = index_val as usize;

                if index < targets.len() {
                    self.pc = targets[index];
                } else {
                    self.pc = *default_target;
                }
            }
            Instruction::DebugLog => {
                let val = self.stack.pop().expect("Stack underflow on DebugLog");
                eprintln!("DebugLog: {val}");
                self.stack.push(0.0);
                self.pc += 1;
            }
            Instruction::DebugPause => {
                eprintln!("TODO: DebugPause");
                self.stack.push(0.0);
                self.pc += 1;
                self.pause();
            }
        }

        if self.state == VMState::Running && self.breakpoints.contains(&self.pc) {
            self.pause();
            return VMState::Paused;
        }

        if originally_paused && self.state == VMState::Running {
            self.state = VMState::Paused;
        }

        return self.state;
    }

    pub fn toggle_breakpoint(&mut self, index: usize) {
        if self.breakpoints.contains(&index) {
            self.breakpoints.remove(&index);
        } else {
            self.breakpoints.insert(index);
        }
    }
}
