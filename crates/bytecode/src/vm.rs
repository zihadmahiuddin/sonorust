use std::collections::HashSet;

use sonorust_ir::IRValue;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sonorust_runtime::context::RuntimeContext;

use crate::instruction::Instruction;

mod control_flow;
mod debug;
mod logical;
mod math;
mod memory;
mod side_effects;
mod timing;

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
    /// Initial state, the VM has not started yet, or has been reset after reaching `VMState::Done`
    Stopped,
    /// The VM should currently be running, but not necessarily _is_ running
    /// (e.g. when `max_steps` is reached but `run` hasn't been called again yet)
    Running,
    /// The VM is currently paused, either at a breakpoint or via manually claling `VM::pause()`
    Paused,
    /// The VM has finished executing all the instructions.
    Done,
}

#[derive(Debug)]
pub enum RunResult {
    StepLimitReached,
    Paused,
    Finished,
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

    pub fn load_instructions(&mut self, bytecode: &[Instruction]) {
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
    pub fn run(&mut self, runtime_ctx: &RuntimeContext, max_steps: usize) -> RunResult {
        if self.state == VMState::Done {
            self.stop();
        }
        if self.state == VMState::Stopped {
            self.state = VMState::Running;
        }

        for _ in 0..max_steps {
            match self.step(runtime_ctx) {
                VMState::Stopped | VMState::Done => return RunResult::Finished,
                VMState::Paused => return RunResult::Paused,
                VMState::Running => continue,
            }
        }

        RunResult::StepLimitReached
    }

    pub fn step(&mut self, runtime_ctx: &RuntimeContext) -> VMState {
        if self.state != VMState::Running && self.state != VMState::Paused {
            return self.state;
        }

        if self.pc >= self.instructions.len() {
            self.state = VMState::Done;
            return self.state;
        }

        // for stepping while paused
        let originally_paused = self.state == VMState::Paused;
        if originally_paused {
            self.state = VMState::Running;
        }

        if !originally_paused && self.breakpoints.contains(&self.pc) {
            self.pause();
            return VMState::Paused;
        }

        let inst = &self.instructions[self.pc];

        use Instruction::*;
        match inst {
            Push(val) => {
                self.stack.push(*val);
                self.pc += 1;
            }
            Pop => {
                self.stack.pop().expect("Stack underflow on Pop");
                self.pc += 1;
            }
            Dup { count, offset } => {
                let len = self.stack.len();
                // offset: How many items from the top to skip
                let start = len - offset - count;
                let end = len - offset;
                self.stack.extend_from_within(start..end);
                self.pc += 1;
            }
            Halt => {
                self.state = VMState::Done;
            }
            Add => self.execute_add(),
            Divide => self.execute_divide(),
            Multiply => self.execute_multiply(),
            Subtract => self.execute_subtract(),
            Abs => self.execute_abs(),
            Frac => self.execute_frac(),
            Trunc => self.execute_trunc(),
            Negate => self.execute_negate(),
            Mod => self.execute_mod(),
            Rem => self.execute_rem(),
            Power => self.execute_power(),
            Clamp => self.execute_clamp(),
            Lerp => self.execute_lerp(),
            LerpClamped => self.execute_lerpclamped(),
            Unlerp => self.execute_unlerp(),
            UnlerpClamped => self.execute_unlerpclamped(),
            Min => self.execute_min(),
            Max => self.execute_max(),
            Remap => self.execute_remap(),
            RemapClamped => self.execute_remapclamped(),
            Round => self.execute_round(),
            Floor => self.execute_floor(),
            Ceil => self.execute_ceil(),
            Sin => self.execute_sin(),
            Sinh => self.execute_sinh(),
            Cos => self.execute_cos(),
            Cosh => self.execute_cosh(),
            Tan => self.execute_tan(),
            Tanh => self.execute_tanh(),
            Arcsin => self.execute_arcsin(),
            Arccos => self.execute_arccos(),
            Arctan => self.execute_arctan(),
            Arctan2 => self.execute_arctan2(),
            Degree => self.execute_degree(),
            Radian => self.execute_radian(),
            Log => self.execute_log(),
            Sign => self.execute_sign(),
            Random => self.execute_random(),
            RandomInteger => self.execute_randominteger(),
            Less => self.execute_less(),
            LessOr => self.execute_less_or(),
            Greater => self.execute_greater(),
            GreaterOr => self.execute_greater_or(),
            And => self.execute_and(),
            Or => self.execute_or(),
            Equal => self.execute_equal(),
            Not => self.execute_not(),
            ReadMem => self.execute_read_mem(runtime_ctx),
            WriteMem => self.execute_write_mem(runtime_ctx),
            Jump(target) => self.execute_jump(*target),
            JumpZero(target) => self.execute_jump_zero(*target),
            JumpTable { .. } => self.execute_jump_table(),
            DebugLog => self.execute_debug_log(),
            Pause => self.execute_debug_pause(),
            Draw => self.execute_draw(runtime_ctx),
            Spawn => self.execute_spawn(runtime_ctx),
            BeatToTime => self.execute_beat_to_time(runtime_ctx),
        }

        if originally_paused && self.state == VMState::Running {
            self.state = VMState::Paused;
        }

        return self.state;
    }

    pub fn toggle_breakpoint(&mut self, pc: usize) -> bool {
        if self.breakpoints.contains(&pc) {
            self.breakpoints.remove(&pc);
            false
        } else {
            self.breakpoints.insert(pc);
            true
        }
    }

    #[inline(always)]
    fn pop_value(&mut self, name: &str) -> IRValue {
        self.stack.pop().expect(&format!(
            "Stack underflow on {:?} ({name})",
            self.instructions[self.pc]
        ))
    }
}
