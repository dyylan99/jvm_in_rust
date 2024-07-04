use std::cell::RefCell;
use std::rc::Rc;
use crate::rtda::object::Object;



#[derive(Default,Clone)]
pub struct Slot{
    pub num:i32,
    pub ref_obj:Option<Rc<RefCell<Object>>>,
}

pub struct LocalVars {
    slots: Vec<Slot>,
}


impl LocalVars{
    pub fn new(max_locals:usize)->LocalVars{
        LocalVars{
            slots: vec![Slot::default();max_locals],
        }
    }

    pub fn set_int(&mut self,index:usize,val:i32){
        self.slots[index].num = val;
    }

    pub fn get_int(&self, index: usize) -> i32 {
        self.slots[index].num
    }

    pub fn set_float(&mut self, index: usize, val: f32) {
        let bytes = f32::to_be_bytes(val);
        self.slots[index].num = i32::from_be_bytes(bytes);
    }
    pub fn get_float(&self, index: usize) -> f32 {
        let bytes = self.slots[index].num.to_be_bytes();
        f32::from_be_bytes(bytes)
    }
    pub fn set_long(&mut self, index: usize, val: i64) {
        // Long consumes two slots
        self.slots[index].num = val as i32;
        self.slots[index + 1].num = (val >> 32) as i32;
    }

    pub fn get_long(&self, index: usize) -> i64 {
        //低位
        let low = self.slots[index].num as u32;
        //高位
        let high = self.slots[index + 1].num as u32;
        //一个long占8个字节,将高位左移32位,和低位做或运算获得一个64位的long
        (high as i64) << 32 | low as i64
    }

    pub fn set_double(&mut self, index: usize, val: f64) {
        // Double consumes two slots
        let bytes = f64::to_be_bytes(val);
        let val = i64::from_be_bytes(bytes);
        self.set_long(index, val);
    }

    pub fn get_double(&self, index: usize) -> f64 {
        let bytes = i64::to_be_bytes(self.get_long(index));
        f64::from_be_bytes(bytes)
    }

    pub fn set_ref(&mut self, index: usize, _ref: Option<Rc<RefCell<Object>>>) {
        self.slots[index].ref_obj = _ref;
    }

    pub fn get_ref(&self, index: usize) -> Option<Rc<RefCell<Object>>> {
        self.slots[index].ref_obj.clone()
    }

}