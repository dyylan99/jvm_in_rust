use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;

#[derive(Default)]
pub struct ConstantValueAttribute {
    constant_value_index: u16,
}

impl AttributeInfo for ConstantValueAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.constant_value_index = reader.read_u16();
    }
}