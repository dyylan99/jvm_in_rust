use crate::rtda::local_vars::LocalVars;
use crate::rtda::operand_stack::OperandStack;

//栈帧
pub struct Frame{
    //局部变量表
    local_vars:LocalVars,
    //操作数栈
    operand_stack: OperandStack,
}
impl Frame {
    pub fn new(max_locals: usize, max_size: usize) -> Self {
        Frame {
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_size),
        }
    }

    pub fn local_vars_mut(&mut self) -> &mut LocalVars {
        &mut self.local_vars
    }

    pub fn operand_stack_mut(&mut self) -> &mut OperandStack {
        &mut self.operand_stack
    }
}