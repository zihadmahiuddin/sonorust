use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use sonorust_ir::{
    IRIndex, IRValue,
    nodes::{IRNode, OpCode},
};

use crate::instruction::Instruction;

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
        Self {
            nodes,
            instructions: Vec::new(),
            basic_blocks: Vec::new(),
            labels: HashMap::new(),
            break_scopes: Vec::new(),
            current_basic_block_start: None,
            control_flow_counter: 0,
        }
    }

    pub fn compile_node(&mut self, index: IRIndex) {
        match &self.nodes[*index] {
            IRNode::Value(value) => {
                self.instructions.push(Instruction::Push(*value));
            }

            IRNode::OpCode(op_code) => match op_code {
                OpCode::Add(add_node) => {
                    if add_node.args.is_empty() {
                        self.instructions.push(Instruction::Push(0.0));
                        return;
                    }

                    self.compile_node(add_node.args[0]);

                    for arg_index in add_node.args.iter().skip(1) {
                        self.compile_node(*arg_index);
                        self.instructions.push(Instruction::Add);
                    }
                }

                OpCode::Execute(exec_node) => {
                    let len = exec_node.args.len();

                    for (i, arg_index) in exec_node.args.iter().enumerate() {
                        self.compile_node(*arg_index);

                        // If this is NOT the last statement in the block,
                        // we must throw away its result so it doesn't pollute the stack.
                        if i < len - 1
                            && self
                                .instructions
                                .last()
                                .is_some_and(|opcode| opcode.produces_result())
                        {
                            self.instructions.push(Instruction::Pop);
                        }
                    }
                }

                OpCode::If(if_node) => {
                    let cf_id = self.control_flow_counter;
                    self.control_flow_counter += 1;

                    self.start_new_block(Some(format!("if_condition_{cf_id}")));
                    self.compile_node(if_node.test);

                    let jump_if_false_address = self.instructions.len();
                    self.instructions.push(Instruction::JumpZero(0));

                    self.start_new_block(Some(format!("if_true_{cf_id}")));
                    self.compile_node(if_node.consequent);

                    let jump_to_end_address = self.instructions.len();
                    self.instructions.push(Instruction::Jump(0));

                    let else_branch_start = self.instructions.len();
                    self.instructions[jump_if_false_address] =
                        Instruction::JumpZero(else_branch_start);

                    self.start_new_block(Some(format!("if_false{cf_id}")));
                    self.compile_node(if_node.alternate);

                    self.start_new_block(Some(format!("if_end_{cf_id}")));

                    let completely_done = self.instructions.len();
                    self.instructions[jump_to_end_address] = Instruction::Jump(completely_done);
                }

                OpCode::Less(less_node) => {
                    self.compile_node(less_node.lhs);
                    self.compile_node(less_node.rhs);
                    self.instructions.push(Instruction::Less);
                }

                OpCode::While(while_node) => {
                    let cf_id = self.control_flow_counter;
                    self.control_flow_counter += 1;

                    self.start_new_block(Some(format!("while_test_{cf_id}")));
                    let loop_start_address = self.instructions.len();

                    self.compile_node(while_node.test);

                    let jump_out_address = self.instructions.len();
                    self.instructions.push(Instruction::JumpZero(0));

                    self.start_new_block(Some(format!("while_body_{cf_id}")));
                    self.compile_node(while_node.body);
                    self.instructions.push(Instruction::Pop);
                    self.instructions
                        .push(Instruction::Jump(loop_start_address));

                    self.start_new_block(Some(format!("while_exit_{cf_id}")));
                    let loop_end_address = self.instructions.len();
                    self.instructions[jump_out_address] = Instruction::JumpZero(loop_end_address);
                }

                OpCode::Block(block_node) => {
                    // 1. Establish a break scope for this block
                    self.break_scopes.push(Vec::new());

                    // 2. Compile block contents
                    self.compile_node(block_node.body);

                    // 3. Resolve and backpatch all Breaks targeting this specific block
                    let end_address = self.instructions.len();
                    if let Some(patches) = self.break_scopes.pop() {
                        for patch_addr in patches {
                            self.instructions[patch_addr] = Instruction::Jump(end_address);
                        }
                    }
                }

                OpCode::Break(break_node) => {
                    // 1. Evaluate the statement value so it remains on the stack as the scope output
                    self.compile_node(break_node.value);

                    // 2. Locate the correct target scope context using the break depth count
                    let target_depth = self
                        .break_scopes
                        .len()
                        .checked_sub(*break_node.count) // TODO: IT SHOULD BE DYNAMIC
                        .expect("Break count depth out of bounds");

                    // 3. Emit a placeholder jump and save its address to be patched later
                    let patch_address = self.instructions.len();
                    self.instructions.push(Instruction::Jump(0));
                    self.break_scopes[target_depth].push(patch_address);
                }

                OpCode::SwitchInteger(switch_node) => {
                    // 1. Put the discriminant value on the stack
                    self.compile_node(switch_node.discriminant);

                    // 2. Emit a placeholder JumpTable instruction
                    let table_address = self.instructions.len();
                    self.instructions.push(Instruction::JumpTable {
                        targets: vec![],
                        default_target: 0,
                    });

                    self.break_scopes.push(Vec::new());
                    let mut targets = Vec::new();

                    // 3. Compile every branch
                    for consequent in &switch_node.consequents {
                        targets.push(self.instructions.len());
                        self.compile_node(*consequent);

                        // Break out automatically to prevent falling into the next case block
                        let exit_jump = self.instructions.len();
                        self.instructions.push(Instruction::Jump(0));
                        self.break_scopes.last_mut().unwrap().push(exit_jump);
                    }

                    let end_address = self.instructions.len();

                    // 4. Backpatch our table layout (normal switches skip to end if out of bounds)
                    self.instructions[table_address] = Instruction::JumpTable {
                        targets,
                        default_target: end_address,
                    };

                    if let Some(patches) = self.break_scopes.pop() {
                        for patch_addr in patches {
                            self.instructions[patch_addr] = Instruction::Jump(end_address);
                        }
                    }
                }

                OpCode::SwitchIntegerWithDefault(switch_node) => {
                    self.compile_node(switch_node.discriminant);

                    let table_address = self.instructions.len();
                    self.instructions.push(Instruction::JumpTable {
                        targets: vec![],
                        default_target: 0,
                    });

                    self.break_scopes.push(Vec::new());
                    let mut targets = Vec::new();

                    for consequent in &switch_node.consequents {
                        targets.push(self.instructions.len());
                        self.compile_node(*consequent);
                        let exit_jump = self.instructions.len();
                        self.instructions.push(Instruction::Jump(0));
                        self.break_scopes.last_mut().unwrap().push(exit_jump);
                    }

                    // 4. Compile the fallback default route if the index isn't found
                    let default_target = self.instructions.len();
                    self.compile_node(switch_node.default_consequent);

                    let end_address = self.instructions.len();
                    self.instructions[table_address] = Instruction::JumpTable {
                        targets,
                        default_target,
                    };

                    if let Some(patches) = self.break_scopes.pop() {
                        for patch_addr in patches {
                            self.instructions[patch_addr] = Instruction::Jump(end_address);
                        }
                    }
                }

                OpCode::Switch(switch_node) => {
                    self.compile_node(switch_node.discriminant);
                    self.break_scopes.push(Vec::new());

                    let mut case_chunks = switch_node.tests_and_consequents.chunks_exact(2);
                    let mut next_case_jump_patches = Vec::new();

                    while let Some(chunk) = case_chunks.next() {
                        let test_node = chunk[0];
                        let consequent_node = chunk[1];

                        // If there's a previous case mismatch jump, it points straight here
                        if !next_case_jump_patches.is_empty() {
                            let current_idx = self.instructions.len();
                            for patch in next_case_jump_patches.drain(..) {
                                self.instructions[patch] = Instruction::JumpZero(current_idx);
                            }
                        }

                        self.instructions.push(Instruction::Dup); // Duplicate the discriminant
                        self.compile_node(test_node); // Evaluate case matcher expression
                        self.instructions.push(Instruction::Equal); // Compare them

                        let mismatch_jump = self.instructions.len();
                        self.instructions.push(Instruction::JumpZero(0));
                        next_case_jump_patches.push(mismatch_jump);

                        self.instructions.push(Instruction::Pop); // Match found! Pop our duplicate disc
                        self.compile_node(consequent_node); // Execute case code

                        let case_end = self.instructions.len();
                        self.instructions.push(Instruction::Jump(0)); // Skip the rest of the switch
                        self.break_scopes.last_mut().unwrap().push(case_end);
                    }

                    // If everything fails and no default exists, clean up and exit
                    let current_idx = self.instructions.len();
                    for patch in next_case_jump_patches {
                        self.instructions[patch] = Instruction::JumpZero(current_idx);
                    }
                    self.instructions.push(Instruction::Pop); // Discard unmatched discriminant

                    let end_address = self.instructions.len();
                    if let Some(patches) = self.break_scopes.pop() {
                        for patch_addr in patches {
                            self.instructions[patch_addr] = Instruction::Jump(end_address);
                        }
                    }
                }

                OpCode::SwitchWithDefault(switch_node) => {
                    self.compile_node(switch_node.discriminant);
                    self.break_scopes.push(Vec::new());

                    let mut case_chunks = switch_node.tests_and_consequents.chunks_exact(2);
                    let mut next_case_jump_patches = Vec::new();

                    while let Some(chunk) = case_chunks.next() {
                        let test_node = chunk[0];
                        let consequent_node = chunk[1];

                        if !next_case_jump_patches.is_empty() {
                            let current_idx = self.instructions.len();
                            for patch in next_case_jump_patches.drain(..) {
                                self.instructions[patch] = Instruction::JumpZero(current_idx);
                            }
                        }

                        self.instructions.push(Instruction::Dup);
                        self.compile_node(test_node);
                        self.instructions.push(Instruction::Equal);

                        let mismatch_jump = self.instructions.len();
                        self.instructions.push(Instruction::JumpZero(0));
                        next_case_jump_patches.push(mismatch_jump);

                        self.instructions.push(Instruction::Pop);
                        self.compile_node(consequent_node);

                        let case_end = self.instructions.len();
                        self.instructions.push(Instruction::Jump(0));
                        self.break_scopes.last_mut().unwrap().push(case_end);
                    }

                    // Default Fallback execution zone
                    let default_idx = self.instructions.len();
                    for patch in next_case_jump_patches {
                        self.instructions[patch] = Instruction::JumpZero(default_idx);
                    }
                    self.instructions.push(Instruction::Pop); // Clean discriminant out before running default block
                    self.compile_node(switch_node.default_consequent);

                    let end_address = self.instructions.len();
                    if let Some(patches) = self.break_scopes.pop() {
                        for patch_addr in patches {
                            self.instructions[patch_addr] = Instruction::Jump(end_address);
                        }
                    }
                }

                OpCode::JumpLoop(loop_node) => {
                    // The index that triggers the final branch is always exactly
                    // the number of branches before it (1 first + N mid branches)
                    let last_index = 1 + loop_node.mid_branches.len();
                    self.break_scopes.push(Vec::new());

                    // 1. Seed the loop by pushing the index of the 0th branch (first_branch)
                    self.instructions.push(Instruction::Push(0.0));

                    // --- LOOP START ---
                    let loop_start_address = self.instructions.len();

                    // 2. Check if the current index triggers the last branch
                    self.instructions.push(Instruction::Dup); // Duplicate index for the equality check
                    self.instructions
                        .push(Instruction::Push(last_index as IRValue));
                    self.instructions.push(Instruction::Equal);

                    let jump_to_switch = self.instructions.len();
                    self.instructions.push(Instruction::JumpZero(0));

                    // --- PATH A: INDEX == LAST_BRANCH ---
                    self.instructions.push(Instruction::Pop); // Discard the duplicated index
                    self.compile_node(loop_node.last_branch); // Execute final branch (leaves return value on stack)

                    let end_jump = self.instructions.len();
                    self.instructions.push(Instruction::Jump(0)); // Escape the loop entirely

                    // --- PATH B: THE JUMP TABLE (State Machine Router) ---
                    let switch_start = self.instructions.len();
                    self.instructions[jump_to_switch] = Instruction::JumpZero(switch_start);

                    let table_address = self.instructions.len();
                    // The JumpTable consumes (pops) the index from the stack automatically
                    self.instructions.push(Instruction::JumpTable {
                        targets: vec![],
                        default_target: 0,
                    });

                    let mut targets = Vec::new();

                    // Target 0: Execute first_branch
                    targets.push(self.instructions.len());
                    self.compile_node(loop_node.first_branch);
                    self.instructions
                        .push(Instruction::Jump(loop_start_address));

                    // Targets 1 to N: Execute mid_branches
                    for mid in &loop_node.mid_branches {
                        targets.push(self.instructions.len());
                        self.compile_node(*mid);
                        self.instructions
                            .push(Instruction::Jump(loop_start_address));
                    }

                    // --- PATH C: DEFAULT / OUT OF BOUNDS ---
                    // If the branch index is not found (e.g., 999.0), the JumpTable routes here.
                    let out_of_bounds = self.instructions.len();
                    self.instructions.push(Instruction::Push(0.0)); // Spec: Return 0 if branch is not found

                    let exit_jump = self.instructions.len();
                    self.instructions.push(Instruction::Jump(0)); // Escape the loop entirely

                    // --- BACKPATCHING RESOLUTIONS ---
                    let end_address = self.instructions.len();
                    self.instructions[end_jump] = Instruction::Jump(end_address);
                    self.instructions[exit_jump] = Instruction::Jump(end_address);

                    self.instructions[table_address] = Instruction::JumpTable {
                        targets,
                        default_target: out_of_bounds,
                    };

                    // If the script explicitly called a Break node inside one of the branches
                    if let Some(patches) = self.break_scopes.pop() {
                        for patch in patches {
                            self.instructions[patch] = Instruction::Jump(end_address);
                        }
                    }
                }

                OpCode::Get(get_node) => {
                    self.compile_node(get_node.block_id);
                    self.compile_node(get_node.index);
                    self.instructions.push(Instruction::ReadMem);
                }

                OpCode::Set(set_node) => {
                    self.compile_node(set_node.block_id);
                    self.compile_node(set_node.index);
                    self.compile_node(set_node.value);
                    self.instructions.push(Instruction::WriteMem);
                }

                OpCode::DebugLog(debug_log_node) => {
                    self.compile_node(debug_log_node.value);
                    self.instructions.push(Instruction::DebugLog);
                }

                OpCode::DebugPause(_) => {
                    self.instructions.push(Instruction::DebugPause);
                }

                _ => todo!(),
            },
        }
    }

    pub fn finish(mut self) -> CompilationResult {
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
