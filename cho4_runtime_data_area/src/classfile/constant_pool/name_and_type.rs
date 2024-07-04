use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_info::{CONSTANT_NAME_AND_TYPE, ConstantInfo};

///字段或方法的名称和描述符,name_index和descriptor_index都是常量池索
/// 引，指向CONSTANT_Utf8_info常量
#[derive(Default)]
pub struct ConstantNameAndTypeInfo {
    name_index: u16,
    descriptor_index: u16,
}

impl ConstantInfo for ConstantNameAndTypeInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.name_index=reader.read_u16();
        self.descriptor_index=reader.read_u16();
    }

    fn tag(&self) -> u8 {
        CONSTANT_NAME_AND_TYPE
    }
}