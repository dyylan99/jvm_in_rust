use std::cell::RefCell;
use std::rc::Rc;
use crate::rtda::frame::Frame;
use crate::rtda::jvm_stack::Stack;

pub struct Thread{
    //寄存器
    pc: i64,
    stack: Stack,
}

impl Thread{
    pub fn new_thread() -> Thread {
        Thread {
            pc: 0,
            stack: Stack::new(1024),
        }
    }

    pub fn pc(&self)->i64{
        self.pc
    }
    pub fn set_pc(&mut self, pc: i64) {
        self.pc = pc;
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.push(frame);
    }
    pub fn pop_frame(&mut self) -> Option<Rc<RefCell<Frame>>> {
        self.stack.pop()
    }
    pub fn current_frame(&self) -> Rc<RefCell<Frame>> {
        self.stack.top()
    }
}