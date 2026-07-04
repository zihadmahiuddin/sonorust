use cranelift::codegen::ir::BlockArg;
use cranelift::prelude::*;

use crate::codegen::CodegenContext;
use sonorust_ir::{IRIndex, nodes::*};

impl<'s, 'b> CodegenContext<'s, 'b> {
    fn build_comparison_ir(&mut self, cond: FloatCC, lhs: IRIndex, rhs: IRIndex) -> Value {
        let lhs = self.build_node_ir(lhs);
        let rhs = self.build_node_ir(rhs);
        let compare_result = self.builder.ins().fcmp(cond, lhs, rhs);
        self.builder
            .ins()
            .fcvt_from_sint(crate::IR_VALUE_CRANELIFT_TYPE, compare_result)
    }

    pub(crate) fn build_equal_ir(&mut self, node: &Equal) -> Value {
        self.build_comparison_ir(FloatCC::Equal, node.lhs, node.rhs)
    }

    pub(crate) fn build_not_equal_ir(&mut self, node: &NotEqual) -> Value {
        self.build_comparison_ir(FloatCC::NotEqual, node.lhs, node.rhs)
    }

    pub(crate) fn build_greater_ir(&mut self, node: &Greater) -> Value {
        self.build_comparison_ir(FloatCC::GreaterThan, node.lhs, node.rhs)
    }

    pub(crate) fn build_greater_or_ir(&mut self, node: &GreaterOr) -> Value {
        self.build_comparison_ir(FloatCC::GreaterThanOrEqual, node.lhs, node.rhs)
    }

    pub(crate) fn build_less_ir(&mut self, node: &Less) -> Value {
        self.build_comparison_ir(FloatCC::LessThan, node.lhs, node.rhs)
    }

    pub(crate) fn build_less_or_ir(&mut self, node: &LessOr) -> Value {
        self.build_comparison_ir(FloatCC::LessThanOrEqual, node.lhs, node.rhs)
    }

    pub(crate) fn build_not_ir(&mut self, node: &Not) -> Value {
        let input = self.build_node_ir(node.value);
        let zero = crate::ir_value_cranelift_const(self.builder.ins(), 0.0);
        let cmp = self.builder.ins().fcmp(FloatCC::Equal, input, zero);
        self.builder
            .ins()
            .fcvt_from_sint(crate::IR_VALUE_CRANELIFT_TYPE, cmp)
    }

    pub(crate) fn build_and_ir(&mut self, node: &And) -> Value {
        assert!(!node.args.is_empty(), "And requires at least one argument");

        let result_block = self.builder.create_block();
        let result_param = self
            .builder
            .append_block_param(result_block, crate::IR_VALUE_CRANELIFT_TYPE);

        let zero = crate::ir_value_cranelift_const(self.builder.ins(), 0.0);

        let mut done = false;
        for (i, arg) in node.args.iter().enumerate() {
            let val = self.build_node_ir(*arg);

            let cond = self.builder.ins().fcmp(FloatCC::Equal, val, zero);

            if i == node.args.len() - 1 {
                self.builder
                    .ins()
                    .jump(result_block, &[BlockArg::Value(val)]);
                done = true;
                break;
            }

            let next_block = self.builder.create_block();

            self.builder.ins().brif(
                cond,
                result_block,
                &[BlockArg::Value(zero)],
                next_block,
                &[],
            );
            self.builder.seal_block(next_block);
            self.builder.switch_to_block(next_block);
        }

        if !done {
            self.builder
                .ins()
                .jump(result_block, &[BlockArg::Value(zero)]);
        }

        self.builder.switch_to_block(result_block);
        self.builder.seal_block(result_block);
        result_param
    }

    pub(crate) fn build_or_ir(&mut self, node: &Or) -> Value {
        assert!(!node.args.is_empty(), "Or requires at least one argument");

        let result_block = self.builder.create_block();
        let result_param = self
            .builder
            .append_block_param(result_block, crate::IR_VALUE_CRANELIFT_TYPE);

        let zero = crate::ir_value_cranelift_const(self.builder.ins(), 0.0);

        let mut done = false;
        for (i, arg) in node.args.iter().enumerate() {
            let val = self.build_node_ir(*arg);

            let cond = self.builder.ins().fcmp(FloatCC::NotEqual, val, zero);

            if i == node.args.len() - 1 {
                self.builder
                    .ins()
                    .jump(result_block, &[BlockArg::Value(val)]);
                done = true;
                break;
            }

            let next_block = self.builder.create_block();

            self.builder
                .ins()
                .brif(cond, result_block, &[BlockArg::Value(val)], next_block, &[]);
            self.builder.seal_block(next_block);
            self.builder.switch_to_block(next_block);
        }

        if !done {
            self.builder
                .ins()
                .jump(result_block, &[BlockArg::Value(zero)]);
        }

        self.builder.switch_to_block(result_block);
        self.builder.seal_block(result_block);
        result_param
    }
}
