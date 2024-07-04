use std::cell::RefCell;
use std::rc::Rc;
use crate::classfile::attribute_info::{AttributeInfo, read_attributes};
use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::ConstantPool;

///Code是变长属性，只存在于method_info结构中
#[derive(Default)]
pub struct CodeAttribute{
    constant_pool: Rc<RefCell<ConstantPool>>,
    ///max_stack给出操作数栈的最大深度
    max_stack: u16,
    ///max_locals给出局部变量表大小
    max_locals: u16,
    code: Vec<u8>,
    exception_table: Vec<ExceptionTableEntry>,
    attributes: Vec<Box<dyn AttributeInfo>>,
}
pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl AttributeInfo for CodeAttribute{
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.max_stack=reader.read_u16();
        self.max_locals=reader.read_u16();
        let code_length=reader.read_u32() as usize;
        self.code=reader.read_bytes(code_length);
        self.exception_table=read_exception_table(reader);
        self.attributes=read_attributes(reader,self.constant_pool.clone());
    }
}
impl CodeAttribute{
    pub fn new(cp: Rc<RefCell<ConstantPool>>)->Self{
       let mut ca=CodeAttribute::default();
        ca.constant_pool=cp;
        ca
    }
}
fn read_exception_table(reader: &mut ClassReader) -> Vec<ExceptionTableEntry> {
    let exception_table_length = reader.read_u16();
    let mut exception_table = vec![];
    for _ in 0..exception_table_length {
        exception_table.push(ExceptionTableEntry {
            start_pc: reader.read_u16(),
            end_pc: reader.read_u16(),
            handler_pc: reader.read_u16(),
            catch_type: reader.read_u16(),
        });
    }
    exception_table
}