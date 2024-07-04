use std::cell::RefCell;
use std::rc::Rc;
use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::ConstantPool;

#[derive(Default)]
pub struct SignatureAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    signature_index: u16,
}

impl AttributeInfo for SignatureAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.signature_index = reader.read_u16();
    }
}

impl SignatureAttribute {
    pub fn new (cp: Rc<RefCell<ConstantPool>>) -> Self {
        let mut sa = SignatureAttribute::default();
        sa.constant_pool = cp;
        sa
    }
}