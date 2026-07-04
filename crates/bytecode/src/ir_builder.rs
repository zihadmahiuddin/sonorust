use std::collections::HashMap;

use sonorust_ir::{IRIndex, nodes::*};

use crate::ir_parser::{Expr, Statement};

#[derive(Debug)]
pub struct IRBuilder {
    pub nodes: Vec<IRNode>,
    pub symbols: HashMap<String, IRIndex>,
    pub root_index: Option<IRIndex>,
}

impl IRBuilder {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            symbols: HashMap::new(),
            root_index: None,
        }
    }

    pub fn emit(&mut self, node: IRNode) -> IRIndex {
        if self.root_index.is_some() {
            panic!("Attemptted to emit node {:#?} after returning.", node);
        }
        self.nodes.push(node);
        IRIndex::from(self.nodes.len() - 1)
    }

    pub fn resolve_symbol(&self, name: &str) -> IRIndex {
        *self
            .symbols
            .get(name)
            .expect(&format!("Undefined variable: {}", name))
    }

    pub fn define_symbol(&mut self, name: String, index: IRIndex) {
        if self.root_index.is_some() {
            panic!(
                "Attemptted to define symbol {name} -> {} after returning.",
                *index
            );
        }
        self.symbols.insert(name, index);
    }

    fn flatten_args(&mut self, args: Vec<Expr>) -> Vec<IRIndex> {
        let mut flat = Vec::new();
        for expr in args {
            match expr {
                Expr::Array(inner_exprs) => {
                    for e in inner_exprs {
                        flat.push(self.lower_expr(&e));
                    }
                }
                _ => {
                    flat.push(self.lower_expr(&expr));
                }
            }
        }
        flat
    }

    pub fn lower_statement(&mut self, stmt: Statement) {
        match stmt {
            Statement::Assign {
                target,
                opcode,
                args,
            } => {
                let args = self.flatten_args(args);

                match opcode.as_deref() {
                    Some(opcode) => {
                        let ir_node = match opcode {
                            "add" => IRNode::OpCode(OpCode::Add(Add { args })),
                            "set" => IRNode::OpCode(OpCode::Set(Set {
                                block_id: args[0],
                                index: args[1],
                                value: args[2],
                            })),
                            "get" => IRNode::OpCode(OpCode::Get(Get {
                                block_id: args[0],
                                index: args[1],
                            })),
                            "less" => IRNode::OpCode(OpCode::Less(Less {
                                lhs: args[0],
                                rhs: args[1],
                            })),
                            "while" => IRNode::OpCode(OpCode::While(While {
                                test: args[0],
                                body: args[1],
                            })),
                            "execute" => IRNode::OpCode(OpCode::Execute(Execute { args })),
                            _ => panic!("Opcode {opcode} not supported"),
                        };

                        let idx = self.emit(ir_node);
                        self.define_symbol(target, idx);
                    }
                    None => self.define_symbol(target, args[0]),
                };
            }
            Statement::Return(target) => {
                let idx = self.resolve_symbol(&target);
                self.root_index = Some(idx);
            }
        }
    }

    fn lower_expr(&mut self, expr: &Expr) -> IRIndex {
        match expr {
            Expr::Float(f) => self.emit(IRNode::Value(*f)),
            Expr::Ident(name) => self.resolve_symbol(name),
            Expr::Array(_) => {
                unreachable!("arrays can not be lowered in lower_expr")
            }
        }
    }

    pub fn finish(self) -> (IRIndex, Vec<IRNode>) {
        let Some(root_index) = self.root_index else {
            panic!("Attempted to finish before returning.");
        };

        (root_index, self.nodes)
    }
}
