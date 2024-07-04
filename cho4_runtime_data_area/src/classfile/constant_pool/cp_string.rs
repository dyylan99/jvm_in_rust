use std::cell::RefCell;
use std::rc::Rc;
use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_info::{CONSTANT_STRING, ConstantInfo};
use crate::classfile::constant_pool::ConstantPool;

///，CONSTANT_String_info本身并不存放字符串数据，
/// 只存了常量池索引，这个索引指向一个CONSTANT_Utf8_info常量。
pub struct ConstantStringInfo {
    constant_pool: Rc<RefCell<ConstantPool>>,
    string_index: u16
}
impl ConstantInfo for ConstantStringInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.string_index = reader.read_u16();
    }

    fn tag(&self) -> u8 {
        CONSTANT_STRING
    }
}
impl ConstantStringInfo {
    pub fn new(cp:  Rc<RefCell<ConstantPool>>) -> Self {
        ConstantStringInfo {
            constant_pool: cp,
            string_index: 0,
        }
    }
    pub fn string(&self) ->String{
        //根据索引从常量池的utf8结构中查找字符串
        self.constant_pool.borrow().get_utf8(self.string_index)
    }
}