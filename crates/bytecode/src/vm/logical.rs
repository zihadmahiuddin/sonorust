use crate::vm::VM;

impl VM {
    #[inline(always)]
    pub(crate) fn execute_equal(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack.push(if lhs == rhs { 1.0 } else { 0.0 });
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_not(&mut self) {
        let value = self.pop_value("value");
        self.stack.push(if value == 0.0 { 1.0 } else { 0.0 });
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_less(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack.push(if lhs < rhs { 1.0 } else { 0.0 });
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_less_or(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack.push(if lhs <= rhs { 1.0 } else { 0.0 });
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_greater(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack.push(if lhs > rhs { 1.0 } else { 0.0 });
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_greater_or(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack.push(if lhs >= rhs { 1.0 } else { 0.0 });
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_and(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack
            .push(if lhs != 0.0 && rhs != 0.0 { 1.0 } else { 0.0 });
        self.pc += 1;
    }

    #[inline(always)]
    pub(crate) fn execute_or(&mut self) {
        let rhs = self.pop_value("rhs");
        let lhs = self.pop_value("lhs");
        self.stack
            .push(if lhs != 0.0 || rhs != 0.0 { 1.0 } else { 0.0 });
        self.pc += 1;
    }
}
