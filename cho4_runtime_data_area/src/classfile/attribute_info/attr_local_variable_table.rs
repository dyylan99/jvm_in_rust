use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;

#[derive(Default)]
pub struct LocalVariableTableAttribute {
    local_variable_table: Vec<LocalVariableTableEntry>,
}

pub struct LocalVariableTableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

impl AttributeInfo for LocalVariableTableAttribute {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let local_variable_table_length = reader.read_u16();
        let mut local_variable_table = vec![];
        for _i in 0..local_variable_table_length {
            local_variable_table.push(LocalVariableTableEntry {
                start_pc: reader.read_u16(),
                length: reader.read_u16(),
                name_index: reader.read_u16(),
                descriptor_index: reader.read_u16(),
                index: reader.read_u16(),
            });
        }
        self.local_variable_table = local_variable_table;
    }
}