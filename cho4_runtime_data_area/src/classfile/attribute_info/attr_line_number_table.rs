use crate::classfile::attribute_info::AttributeInfo;
use crate::classfile::class_reader::ClassReader;

#[derive(Default)]
pub struct LineNumberTableAttribute{
    line_number_table: Vec<LineNumberTableEntry>,
}

pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

impl AttributeInfo for LineNumberTableAttribute{
    fn read_info(&mut self, reader: &mut ClassReader) {
        let line_number_table_length = reader.read_u16();
        self.line_number_table = Vec::with_capacity(line_number_table_length as usize);
        for _ in 0..line_number_table_length {
            let start_pc = reader.read_u16();
            let line_number = reader.read_u16();
            self.line_number_table.push(LineNumberTableEntry{start_pc, line_number});
        }
    }
}