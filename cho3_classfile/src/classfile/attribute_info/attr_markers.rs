use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;

#[derive(Default)]
pub struct DeprecatedAttribute {

}
impl AttributeInfo for DeprecatedAttribute{
    fn read_info(&mut self, reader: &mut ClassReader) {

    }
}

#[derive(Default)]
pub struct SyntheticAttribute {}

impl AttributeInfo for SyntheticAttribute {
    fn read_info(&mut self, _reader: &mut ClassReader) {

    }
}
pub struct MarkerAttribute {

}
impl AttributeInfo for MarkerAttribute {
    fn read_info(&mut self, _reader: &mut ClassReader) {

    }
}