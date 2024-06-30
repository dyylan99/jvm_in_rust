use std::cell::RefCell;
use std::rc::Rc;
use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::ConstantPool;

///SourceFile是可选定长属性，只会出现在ClassFile结构中，用于
/// 指出源文件名。其结构定义如下
#[derive(Default)]
pub struct SourceFileAttribute {
    constant_pool: Rc<RefCell<ConstantPool>>,
    source_file_index:u16,
}

impl AttributeInfo for SourceFileAttribute{
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.source_file_index=reader.read_u16();
    }
}

impl SourceFileAttribute{
    pub fn new(cp: Rc<RefCell<ConstantPool>>)->Self{
      let mut source_file_attribute = SourceFileAttribute::default();
        source_file_attribute.constant_pool=cp;
        source_file_attribute
    }
}