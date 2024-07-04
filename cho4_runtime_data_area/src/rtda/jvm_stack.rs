use std::cell::RefCell;
use std::rc::Rc;
use crate::rtda::frame::Frame;

//线程私有的方法栈
pub struct Stack{
    max_size: usize,
    //栈顶指针
    top: usize,
    frames: Vec<Option<Rc<RefCell<Frame>>>>,
}

impl Stack{
    pub fn new(max_size:usize)->Stack{
        Stack{
            max_size,
            top:0,
            frames: vec![None;max_size],
        }
    }

    pub fn push(&mut self,frame:Frame){
        if self.top >= self.max_size{
            panic!("java.lang.StackOverflowError")
        }
        self.frames[self.top] = Some(Rc::new(RefCell::new(frame)));
        self.top += 1;
    }
    pub fn pop(&mut self)->Option<Rc<RefCell<Frame>>>
    {
        if self.top == 0{
            panic!("jvm stack is empty,no element here")
        }
        self.top -= 1;
        let frame = self.frames[self.top].clone();
        self.frames[self.top] = None;
        frame
    }
    pub fn top(&self)->Rc<RefCell<Frame>>{
        self.frames[self.top-1].clone().unwrap()
    }
}
