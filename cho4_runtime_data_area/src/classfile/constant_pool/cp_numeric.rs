use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_info::{CONSTANT_FLOAT, CONSTANT_INTEGER, CONSTANT_LONG, ConstantInfo};

#[derive(Default)]
pub struct ConstantIntegerInfo {
    val: i32,
}

impl ConstantInfo for ConstantIntegerInfo{
    fn read_info(&mut self, reader: &mut ClassReader) {
        //java的int类型大小为4字节
        self.val=reader.read_u32() as i32;
    }

    fn tag(&self) -> u8 {
        CONSTANT_INTEGER
    }
}
impl ConstantIntegerInfo {
    pub fn value(&self) -> i32 {
        self.val
    }
}
#[derive(Default)]
pub struct ConstantFloatInfo {
    val: f32,
}
impl ConstantInfo for ConstantFloatInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.val = f32::from_bits(reader.read_u32());
    }

    fn tag(&self) -> u8 {
        CONSTANT_FLOAT
    }
}
impl ConstantFloatInfo {
    pub fn value(&self) -> f32 {
        self.val
    }
}
#[derive(Default)]
pub struct ConstantLongInfo {
    val: i64,
}

impl ConstantInfo for  ConstantLongInfo  {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.val= f64::from_bits(reader.read_u64()) as i64;
    }

    fn tag(&self) -> u8 {
        CONSTANT_LONG
    }
}
#[derive(Default)]
pub struct ConstantDoubleInfo {
    val: f64,
}
impl ConstantInfo for ConstantDoubleInfo {
    fn read_info(&mut self, reader: &mut ClassReader) {
        self.val = f64::from_bits(reader.read_u64());
    }

    fn tag(&self) -> u8 {
        super::CONSTANT_DOUBLE
    }
}
impl ConstantDoubleInfo {
    pub fn value(&self) -> f64 {
        self.val
    }
}

