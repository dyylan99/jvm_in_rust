use std::cell::RefCell;
use std::rc::Rc;
use crate::rtda::object::Object;

//一个更节省内存的方法是直接使用一个字节缓冲区（Vec<u8>）来存储所有的局部变量，
// 然后根据需要将这些字节解释为不同的数据类型。
#[derive(Default,Clone)]
pub struct LocalVarsU8 {
    slots: Vec<u8>,
}
#[test]
fn test_local_vars() {
    let mut local_vars = LocalVarsU8 { slots: vec![0; 100] };
    local_vars.set_int(0, 100);
    local_vars.set_int(1, -100);
    local_vars.set_long(2, 2997924580);
    local_vars.set_long(4, -2997924580);
    local_vars.set_float(6, 3.1415926);
    local_vars.set_double(7, 2.71828182845);
    local_vars.set_ref(9, Option::from(Rc::new(RefCell::new(Object::default()))));

    println!("{}", local_vars.get_int(0));
    println!("{}", local_vars.get_int(1));
    println!("{}", local_vars.get_long(2));
    println!("{}", local_vars.get_long(4));
    println!("{}", local_vars.get_float(6));
    println!("{}", local_vars.get_double(7));
    println!("{:?}", local_vars.get_ref(9));
}


impl LocalVarsU8{
    pub fn set_int(&mut self, index: usize, val: i32) {
        let bytes = val.to_be_bytes();
        self.slots[index..index + 4].copy_from_slice(&bytes);
    }
    pub fn get_int(&self,index:usize)->i32{
        let mut bytes = [0u8;4];
        bytes.copy_from_slice(&self.slots[index..index+4]);
        i32::from_be_bytes(bytes)
    }
    pub fn set_float(&mut self,index:usize,val:f32){
        let bytes = val.to_be_bytes();
        self.slots[index..index+4].copy_from_slice(&bytes);
    }
    pub fn get_float(&self,index:usize)->f32{
        let mut bytes = [0u8;4];
        bytes.copy_from_slice(&self.slots[index..index+4]);
        f32::from_be_bytes(bytes)
    }
    pub fn set_long(&mut self,index:usize,val:i64){
        let bytes=val.to_be_bytes();
        self.slots[index..index+8].copy_from_slice(&bytes);
    }
    pub fn get_long(&self,index:usize)->i64{
        let mut bytes = [0u8;8];
        bytes.copy_from_slice(&self.slots[index..index+8]);
        i64::from_be_bytes(bytes)
    }
    pub fn set_double(&mut self,index:usize,val:f64){
        let bytes = val.to_be_bytes();
        self.slots[index..index+8].copy_from_slice(&bytes);
    }
    pub fn get_double(&self,index:usize)->f64{
        let mut bytes = [0u8;8];
        bytes.copy_from_slice(&self.slots[index..index+8]);
        f64::from_be_bytes(bytes)
    }
    pub fn set_ref(&mut self,index:usize,val:Option<Rc<RefCell<Object>>>){
        let ref_val = match val {
            Some(r) =>Rc::into_raw(r) as usize,
            None => 0,
        };
        let bytes = ref_val.to_be_bytes();
        self.slots[index..index+8].copy_from_slice(&bytes);
    }
    pub fn get_ref(&mut self,index:usize)->Option<Rc<RefCell<Object>>>{
        let mut bytes = [0u8;8];
        bytes.copy_from_slice(&self.slots[index..index+8]);
        let ref_val = usize::from_be_bytes(bytes);
        if ref_val == 0 {
            None
        } else {
            Some(unsafe {Rc::from_raw(ref_val as *const RefCell<Object>)})
        }
    }
}
