use std::cell::RefCell;
use std::rc::Rc;
use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_info::{CONSTANT_FIELD_REF, CONSTANT_INTERFACE_METHOD_REF, CONSTANT_METHOD_REF, ConstantInfo};
use crate::classfile::constant_pool::ConstantPool;

/**
 * @description class_index和name_and_type_index都是常量池索引，分别指向
    CONSTANT_Class_info和CONSTANT_NameAndType_info常量
 * @author Dylan
 * @throws
 * @time 2024/6/28 23:03
 */
pub struct ConstantMemberRefInfo{
    class_index: u16,
    name_and_type_index: u16,
    constant_pool: Rc<RefCell<ConstantPool>>,
}
impl ConstantMemberRefInfo{
    fn new (cp: Rc<RefCell<ConstantPool>>)->Self{
        ConstantMemberRefInfo{
            class_index:0,
            name_and_type_index:0,
            constant_pool:cp,
        }
    }
    fn read_info(&mut self,reader: &mut ClassReader){
        self.class_index = reader.read_u16();
        self.name_and_type_index = reader.read_u16();
    }
}

///字段符号引用
pub struct ConstantFieldRefInfo {
    member_ref_info: ConstantMemberRefInfo
}
impl ConstantInfo for ConstantFieldRefInfo{
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.member_ref_info.read_info(reader);
    }

    fn tag(&self) -> u8 {
        CONSTANT_FIELD_REF
    }
}

impl ConstantFieldRefInfo{
    pub fn new(cp: Rc<RefCell<ConstantPool>>)->Self{
        ConstantFieldRefInfo{
            member_ref_info: ConstantMemberRefInfo::new(cp)
        }
    }
}

///普通非接口方法符号引用
pub struct ConstantMethodRefInfo {
    member_info: ConstantMemberRefInfo,
}
impl ConstantInfo for ConstantMethodRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.member_info.read_info(reader);
    }

    fn tag(&self) -> u8 {
        CONSTANT_METHOD_REF
    }
}
impl ConstantMethodRefInfo {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantMethodRefInfo {
            member_info: ConstantMemberRefInfo::new(cp),
        }
    }
}
///接口方法符号引用
pub struct ConstantInterfaceMethodRefInfo {
    member_info: ConstantMemberRefInfo,
}
impl ConstantInfo for ConstantInterfaceMethodRefInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.member_info.read_info(reader);
    }

    fn tag(&self) -> u8 {
        CONSTANT_INTERFACE_METHOD_REF
    }
}
impl ConstantInterfaceMethodRefInfo {
    pub fn new(cp: Rc<RefCell<ConstantPool>>) -> Self {
        ConstantInterfaceMethodRefInfo {
            member_info: ConstantMemberRefInfo::new(cp),
        }
    }
}