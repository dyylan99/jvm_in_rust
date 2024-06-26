use std::cell::RefCell;
use std::rc::Rc;
use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_info::{CONSTANT_CLASS, ConstantInfo};
use crate::classfile::constant_pool::ConstantPool;

///表示类或者接口的符号引用
#[derive(Clone)]
pub struct ConstantClassInfo {
    constant_pool: Rc<RefCell<ConstantPool>>,
    name_index: u16,  //name_index是常量池索引，指向CONSTANT_Utf8_info常量
}

impl ConstantInfo for ConstantClassInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.name_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        CONSTANT_CLASS
    }
}
impl ConstantClassInfo{
    pub fn new(cp: Rc<RefCell<ConstantPool>>)->Self{
        ConstantClassInfo{
            constant_pool:cp,
            name_index:0,
        }
    }

    pub fn name(&self)->String{
        self.constant_pool.borrow().get_utf8(self.name_index)
    }
}