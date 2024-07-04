use std::cell::RefCell;
use std::rc::Rc;
use crate::rtda::object::Object;

//一个更节省内存的方法是直接使用一个字节缓冲区（Vec<u8>）来存储操作数栈，
// 然后根据需要将这些字节解释为不同的数据类型。
#[derive(Default,Clone)]
pub struct OperandStackU8 {
    buffer: Vec<u8>,
    index: usize,
}
#[test]
fn test_operand_stack() {
    let mut operand_stack = OperandStackU8::new(100);
    operand_stack.push_int(100);
    operand_stack.push_int(-100);
    operand_stack.push_long(2997924580);
    operand_stack.push_long(-2997924580);
    operand_stack.push_float(3.1415926);
    operand_stack.push_double(2.71828182845);
    operand_stack.push_ref(None);

    println!("{:?}", operand_stack.pop_ref());
    println!("{}", operand_stack.pop_double());
    println!("{}", operand_stack.pop_float());
    println!("{}", operand_stack.pop_long());
    println!("{}", operand_stack.pop_long());
    println!("{}", operand_stack.pop_int());
    println!("{}", operand_stack.pop_int());
}

impl OperandStackU8{
    pub fn new(max_stack: usize) -> Self {
        OperandStackU8 {
            buffer: vec![0u8; max_stack],
            index:0
        }
    }
    /**
     * @description 下面所有方法都没有对栈的大小进行检查，调用者需要自己保证栈的大小
     * @author Dylan
     * @throws
     * @time 2024/7/4 18:06
     */

    pub fn push_int(&mut self, val: i32) {
        let bytes = val.to_be_bytes();
        self.buffer[self.index..self.index+4].copy_from_slice(&bytes);
        self.index += 4;
    }
    pub fn pop_int(&mut self)->i32{
        let mut bytes = [0u8;4];
        bytes.copy_from_slice(&self.buffer[self.index-4..self.index]);
        self.index-=4;
        i32::from_be_bytes(bytes)
    }
    pub fn push_float(&mut self,val:f32){
        let bytes = val.to_be_bytes();
        self.buffer[self.index..self.index+4].copy_from_slice(&bytes);
        self.index += 4;
    }
    pub fn pop_float(&mut self)->f32{
        let mut bytes = [0u8;4];
        bytes.copy_from_slice(&self.buffer[self.index-4..self.index]);
        self.index-=4;
        f32::from_be_bytes(bytes)
    }
    pub fn push_long(&mut self,val:i64){
        let bytes=val.to_be_bytes();
        self.buffer[self.index..self.index+8].copy_from_slice(&bytes);
        self.index+=8
    }
    pub fn pop_long(&mut self)->i64{
        let mut bytes = [0u8;8];
        bytes.copy_from_slice(&self.buffer[self.index-8..self.index]);
        self.index-=8;
        i64::from_be_bytes(bytes)
    }
    pub fn push_double(&mut self,val:f64){
        let bytes = val.to_be_bytes();
        self.buffer[self.index..self.index+8].copy_from_slice(&bytes);
        self.index+=8;
    }
    pub fn pop_double(&mut self)->f64{
        let mut bytes = [0u8;8];
        bytes.copy_from_slice(&self.buffer[self.index-8..self.index]);
        self.index-=8;
        f64::from_be_bytes(bytes)
    }
    pub fn push_ref(&mut self,val:Option<Rc<RefCell<Object>>>){
        let ref_val = match val {
            Some(r) =>Rc::into_raw(r) as usize,
            None => 0,
        };
        let bytes = ref_val.to_be_bytes();
        self.buffer[self.index..self.index+8].copy_from_slice(&bytes);
        self.index+=8;
    }
    pub fn pop_ref(&mut self)->Option<Rc<RefCell<Object>>>{
        let mut bytes = [0u8;8];
        bytes.copy_from_slice(&self.buffer[self.index-8..self.index]);
        let ref_val = usize::from_be_bytes(bytes);
        self.index-=8;
        if ref_val == 0 {
            None
        } else {
            Some(unsafe {Rc::from_raw(ref_val as *const RefCell<Object>)})
        }
    }
}
