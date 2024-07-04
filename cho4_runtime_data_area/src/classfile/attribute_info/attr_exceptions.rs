use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;

///Exceptions是变长属性，记录方法抛出的异常表
#[derive(Default)]
pub struct ExceptionsAttribute {
    exception_index_table: Vec<u16>,
}

impl AttributeInfo for ExceptionsAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.exception_index_table = reader.read_u16s();
    }
}