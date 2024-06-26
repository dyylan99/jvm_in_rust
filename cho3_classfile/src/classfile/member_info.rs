use std::{cell::RefCell, rc::Rc};
use crate::classfile::attribute_info::{AttributeInfo, read_attributes};
use crate::classfile::constant_pool::ConstantPool;

use super::class_reader::ClassReader;

pub struct MemberInfo {
    /// 常量池
    constant_pool: Rc<RefCell<ConstantPool>>,
    /// 链接标识
    access_flags: u16,
    /// 成员访问标志
    name_index: u16,
    /// 描述符索引
    descriptor_index: u16,
    attributes: Vec<Box<dyn AttributeInfo>>,
}
impl MemberInfo {
    pub fn read(reader: &mut ClassReader, cp: Rc<RefCell<ConstantPool>>) -> Vec<Self> {
        let member_count = reader.read_u16();
        let mut members = vec![];
        for _i in 0..member_count {
            members.push(MemberInfo::read_member(reader, cp.clone()));
        }
        members
    }
    pub fn read_member(reader: &mut ClassReader, cp: Rc<RefCell<ConstantPool>>) -> Self {
        MemberInfo {
            constant_pool: cp.clone(),
            access_flags: reader.read_u16(),
            name_index: reader.read_u16(),
            descriptor_index: reader.read_u16(),
            attributes: read_attributes(reader, cp),
        }
    }
    pub fn access_flags(&self) -> u16 {
        self.access_flags
    }

    pub fn name(&self) -> String {
        self.constant_pool.borrow().get_utf8(self.name_index)
    }

    pub fn descriptor(&self) -> String {
        self.constant_pool.borrow().get_utf8(self.descriptor_index)
    }
}


