use std::cell::RefCell;
use std::rc::Rc;
use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_pool::ConstantPool;
use crate::classfile::constant_pool::cp_class::ConstantClassInfo;
use crate::classfile::constant_pool::cp_invoke_dynamic::{ConstantInvokeDynamicInfo, ConstantMethodHandleInfo, ConstantMethodTypeInfo};
use crate::classfile::constant_pool::cp_member_ref::{ConstantFieldRefInfo, ConstantInterfaceMethodRefInfo, ConstantMethodRefInfo};
use crate::classfile::constant_pool::cp_numeric::{ConstantDoubleInfo, ConstantFloatInfo, ConstantIntegerInfo, ConstantLongInfo};
use crate::classfile::constant_pool::cp_string::ConstantStringInfo;
use crate::classfile::constant_pool::cp_utf8::ConstantUtf8Info;
use crate::classfile::constant_pool::name_and_type::ConstantNameAndTypeInfo;

pub const CONSTANT_UTF8: u8                    = 1;
pub const CONSTANT_INTEGER: u8                 = 3;
pub const CONSTANT_FLOAT: u8                   = 4;
pub const CONSTANT_LONG: u8                    = 5;
pub const CONSTANT_DOUBLE: u8                  = 6;
pub const CONSTANT_CLASS: u8                   = 7;
pub const CONSTANT_STRING: u8                  = 8;
pub const CONSTANT_FIELD_REF: u8               = 9;
pub const CONSTANT_METHOD_REF: u8              = 10;
pub const CONSTANT_INTERFACE_METHOD_REF: u8    = 11;
pub const CONSTANT_NAME_AND_TYPE: u8           = 12;
pub const CONSTANT_METHOD_HANDLE: u8           = 15;
pub const CONSTANT_METHOD_TYPE: u8             = 16;
pub const CONSTANT_INVOKE_DYNAMIC: u8          = 18;

/**
 * @description 常量信息接口,所有常量信息都实现该接口,包括常量池中的类名、字段名、方法名等等
                常量数据的第一字节是tag，用来区分常量类型。
 * @author Dylan
 * @throws
 * @time 2024/6/28 22:09
 */

pub trait ConstantInfo{
    fn read_info(&mut self,reader: &mut ClassReader);

    ///获取标志
    fn tag(&self)->u8;
}
pub fn read_constant_info(reader: &mut ClassReader,i:u16,cp:Rc<RefCell<ConstantPool>>)->Box<dyn ConstantInfo>{
    let tag=reader.read_u8();
    let mut c=new_constant_info(reader, tag, i, cp);
    match (&c).tag() {
        // CONSTANT_Utf8_info、CONSTANT_Class_info 在创建之后立即调用 read_info
        CONSTANT_UTF8 | CONSTANT_CLASS => {},
        _ => {
            c.read_info(reader)
        }
    }
    c
}
/**
 * @description 根据tag创建对应的常量信息实例
                1. 创建常量信息实例
                2. 立即调用 read_info 方法读取常量信息
                3. 将常量信息实例存入常量池
                4. 返回常量信息实例
 * @author Dylan
 * @throws
 * @time 2024/6/28 22:12
 */

fn new_constant_info(reader: &mut ClassReader, tag: u8, i: u16,cp:Rc<RefCell<ConstantPool>>)->Box<dyn ConstantInfo>{
    match tag {
        CONSTANT_CLASS =>{
            let mut b = Box::new(ConstantClassInfo::new(cp.clone()));
            //立即调用
            b.read_info(reader);
            cp.borrow_mut().class_info_map.insert(i, *b.clone());
            b
        },
        CONSTANT_FIELD_REF => Box::new(ConstantFieldRefInfo::new(cp)),
        CONSTANT_METHOD_REF => Box::new(ConstantMethodRefInfo::new(cp)),
        CONSTANT_INTERFACE_METHOD_REF => Box::new(ConstantInterfaceMethodRefInfo::new(cp)),
        CONSTANT_STRING => Box::new(ConstantStringInfo::new(cp)),
        CONSTANT_INTEGER => Box::new(ConstantIntegerInfo::default()),
        CONSTANT_FLOAT => Box::new(ConstantFloatInfo::default()),
        CONSTANT_LONG => Box::new(ConstantLongInfo::default()),
        CONSTANT_DOUBLE => Box::new(ConstantDoubleInfo::default()),
        CONSTANT_NAME_AND_TYPE => Box::new(ConstantNameAndTypeInfo::default()),
        CONSTANT_UTF8 => {
            let mut b = Box::new(ConstantUtf8Info::default());
            // 立即调用
            b.read_info(reader);
            cp.borrow_mut().utf8_info_map.insert(i, *b.clone());
            b
        },
        CONSTANT_METHOD_HANDLE => Box::new(ConstantMethodHandleInfo::default()),
        CONSTANT_METHOD_TYPE => Box::new(ConstantMethodTypeInfo::default()),
        CONSTANT_INVOKE_DYNAMIC => Box::new(ConstantInvokeDynamicInfo::default()),
        _ => panic!("{}", "java.lang.ClassFormatError: constant pool tag!")
    }
}
