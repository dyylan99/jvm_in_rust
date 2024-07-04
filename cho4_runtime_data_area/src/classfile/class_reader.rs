use std::convert::TryInto;


/**
 * @description字节数据读取器
 * @author Dylan
 * @throws
 * @time 2024/6/29 22:25
 */

pub struct ClassReader {
    data: Vec<u8>,
}
impl ClassReader {
    pub fn new(data: Vec<u8>) -> Self {
        ClassReader { data }
    }

    ///u1
    /// 读取一个字节
    pub fn read_u8(&mut self) -> u8 {
        let val = self.data[0];
        self.data = self.data[1..].to_vec();
        val
    }

    ///u2
    /// 读取两个字节
    pub fn read_u16(&mut self) -> u16 {
        let val = u16::from_be_bytes(self.data[0..2].try_into().unwrap());
        self.data = self.data[2..].to_vec();
        val
    }
    ///u4
    /// 读取四个字节
    pub fn read_u32(&mut self) -> u32 {
        let val = u32::from_be_bytes(self.data[0..4].try_into().unwrap());
        self.data = self.data[4..].to_vec();
        val
    }
    ///u8
    /// 读取八个字节
    pub fn read_u64(&mut self) -> u64 {
        let val = u64::from_be_bytes(self.data[0..8].try_into().unwrap());
        self.data = self.data[8..].to_vec();
        val
    }

    ///读取u2表,表的大小you第一个u2决定
    pub fn read_u16s(&mut self) -> Vec<u16> {
        let n = self.read_u16() as usize;
        let mut s = Vec::with_capacity(n);
        for _ in 0..n {
            s.push(self.read_u16());
        }
        s
    }
    ///读取指定大小的字节数组u8对应8位 1字节
    pub fn read_bytes(&mut self, n: usize) -> Vec<u8> {
        //读取的数据
        let val = self.data[0..n].to_vec();
        //剩余的数据
        self.data = self.data[n..].to_vec();
        val
    }
}
