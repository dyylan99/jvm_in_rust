use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_info::{CONSTANT_DOUBLE, CONSTANT_LONG, ConstantInfo, read_constant_info};
use crate::classfile::constant_pool::cp_class::ConstantClassInfo;
use crate::classfile::constant_pool::cp_utf8::ConstantUtf8Info;

pub(crate) mod cp_numeric;
pub mod cp_utf8;
pub mod cp_string;
pub mod cp_class;
pub mod name_and_type;
pub mod cp_member_ref;
pub mod cp_invoke_dynamic;

#[derive(Default)]
pub struct ConstantPool {
    pub infos:Vec<Option<Box<dyn ConstantInfo>>>,

    ///存储 CONSTANT_Class_info 常量映射
    pub class_info_map: HashMap<u16,ConstantClassInfo>,
    ///存储 CONSTANT_utf8_info 常量映射
    pub utf8_info_map:HashMap<u16,ConstantUtf8Info>,
}
impl ConstantPool{
    pub fn get_class_name(&self,index:u16)->String{
        match self.class_info_map.get(&index) {
            Some(info)=>info.name(),
            None=>"".to_string()
        }
    }

    pub fn get_utf8(&self,index: u16)->String{
        match self.utf8_info_map.get(&index) {
            Some(info)=>info.str(),
            None=>"".to_string()
        }
    }
}

pub fn read_constant_pool(reader:&mut ClassReader)->Rc<RefCell<ConstantPool>>{
    let cp_count = reader.read_u16();
    let cp = Rc::new(RefCell::new(ConstantPool::default()));
    // 第一个元素无效
    cp.borrow_mut().infos.push(None);
    // 常量池索引从 1 到 constant_pool_count - 1.
    let mut i= 1;
    while i<cp_count{
        let constant_info = read_constant_info(reader, i,cp.clone());
        match (&constant_info).tag() {
            CONSTANT_LONG | CONSTANT_DOUBLE => {
                cp.borrow_mut().infos.push(Some(constant_info));
                cp.borrow_mut().infos.push(None);
                i += 1;
            }
            _ => {
                cp.borrow_mut().infos.push(Some(constant_info));
            }
        }
        i+= 1;
    }
    cp
}

