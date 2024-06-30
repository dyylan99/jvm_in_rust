use std::cell::RefCell;
use std::rc::Rc;
use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::ConstantPool;

#[derive(Default)]
pub struct EnclosingMethodAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    class_index: u16,
    method_index: u16,
}

impl AttributeInfo for EnclosingMethodAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.class_index = reader.read_u16();
        self.method_index = reader.read_u16();
    }
}

impl EnclosingMethodAttribute {
    pub fn new (cp: Rc<RefCell<ConstantPool>>) -> Self {
        let mut ema = EnclosingMethodAttribute::default();
        ema.constant_pool = cp;
        ema
    }
}