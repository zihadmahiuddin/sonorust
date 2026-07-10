use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use sonorust_ir::{IRIndex, nodes::IRNode};

use crate::{compiler::compilable::Compilable, instruction::Instruction};

mod audio;
mod compilable;
mod control_flow;
mod debug;
mod logical;
mod math;
mod memory;
mod side_effects;
mod timing;

#[derive(Debug)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
pub struct Compiler<'a> {
    nodes: &'a [IRNode],
    instructions: Vec<Instruction>,
    basic_blocks: Vec<BasicBlock>,
    labels: HashMap<usize, String>,
    break_scopes: Vec<Vec<usize>>, // Tracks instruction indices for backpatching breaks
    current_basic_block_start: Option<usize>,
    control_flow_counter: usize,
}

impl<'a> Compiler<'a> {
    pub fn new(nodes: &'a [IRNode]) -> Self {
        let mut compiler = Self {
            nodes,
            instructions: Vec::new(),
            basic_blocks: Vec::new(),
            labels: HashMap::new(),
            break_scopes: Vec::new(),
            current_basic_block_start: None,
            control_flow_counter: 0,
        };
        compiler.start_new_block(Some("start".to_string()));
        compiler
    }

    pub fn compile_node(&mut self, index: IRIndex) {
        self.nodes[*index].compile(self);
    }

    pub fn add_inst(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }

    pub fn finish(mut self) -> CompilationResult {
        self.add_inst(Instruction::Halt);
        self.build_cfg_edges();
        return CompilationResult {
            instructions: self.instructions,
            basic_blocks: self.basic_blocks,
            labels: self.labels,
        };
    }

    fn build_cfg_edges(&mut self) {
        self.start_new_block(None);

        let get_block_id_by_pc = |blocks: &[BasicBlock], pc: usize| -> Option<usize> {
            blocks.iter().find(|b| b.start_pc == pc).map(|b| b.id)
        };

        for i in 0..self.basic_blocks.len() {
            let block = &self.basic_blocks[i];
            let end_pc = block.end_pc;

            if end_pc == 0 || end_pc > self.instructions.len() {
                continue;
            }

            let last_inst = &self.instructions[end_pc - 1];

            let edge = match last_inst {
                Instruction::Jump(target) => {
                    if let Some(target_id) = get_block_id_by_pc(&self.basic_blocks, *target) {
                        BlockEdge::Unconditional {
                            target_block_id: target_id,
                        }
                    } else {
                        BlockEdge::Exit
                    }
                }
                Instruction::JumpZero(target) => {
                    let false_block = get_block_id_by_pc(&self.basic_blocks, *target);
                    let true_block = get_block_id_by_pc(&self.basic_blocks, end_pc);

                    if let (Some(f), Some(t)) = (false_block, true_block) {
                        BlockEdge::Conditional {
                            true_block_id: t,
                            false_block_id: f,
                        }
                    } else {
                        BlockEdge::Exit
                    }
                }
                _ => {
                    if i + 1 < self.basic_blocks.len() {
                        BlockEdge::Fallthrough {
                            target_block_id: self.basic_blocks[i + 1].id,
                        }
                    } else {
                        BlockEdge::Exit
                    }
                }
            };

            self.basic_blocks[i].edge = Some(edge);
        }
    }

    fn start_new_block(&mut self, label: Option<String>) {
        let current_pc = self.instructions.len();

        if let Some(start) = self.current_basic_block_start {
            if start < current_pc {
                self.basic_blocks.push(BasicBlock {
                    id: self.basic_blocks.len(),
                    start_pc: start,
                    end_pc: current_pc,
                    edge: None,
                });
            }
        }

        self.current_basic_block_start = Some(current_pc);

        if let Some(l) = label {
            self.labels.insert(current_pc, l);
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
pub enum BlockEdge {
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    Unconditional {
        target_block_id: usize,
    },
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    Conditional {
        true_block_id: usize,
        false_block_id: usize,
    },
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    Fallthrough {
        target_block_id: usize,
    },
    Exit,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
pub struct BasicBlock {
    pub id: usize,
    pub start_pc: usize,
    pub end_pc: usize,
    pub edge: Option<BlockEdge>,
}

#[derive(Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
pub struct CompilationResult {
    pub instructions: Vec<Instruction>,
    pub basic_blocks: Vec<BasicBlock>,
    pub labels: HashMap<usize, String>,
}
