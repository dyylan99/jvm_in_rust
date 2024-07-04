use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;

pub struct InnerClassesAttribute {
    classes: Vec<InnerClassInfo>,
}

pub struct InnerClassInfo {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}

impl AttributeInfo for InnerClassesAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let number_of_classes = reader.read_u16();
        let mut classes = vec![];
        for _i in 0..number_of_classes {
            classes.push(InnerClassInfo {
                inner_class_info_index: reader.read_u16(),
                outer_class_info_index: reader.read_u16(),
                inner_name_index: reader.read_u16(),
                inner_class_access_flags: reader.read_u16(),
            });
        }
        self.classes = classes;
    }
}