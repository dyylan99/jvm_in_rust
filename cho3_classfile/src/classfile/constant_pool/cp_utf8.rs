use std::char::decode_utf16;
use crate::classfile::class_reader::ClassReader;
use crate::classfile::constant_info::{CONSTANT_UTF8, ConstantInfo};

#[derive(Default, Clone)]
pub struct ConstantUtf8Info {
    str: String,
}
impl ConstantInfo for ConstantUtf8Info {
    fn read_info(&mut self, reader: &mut ClassReader) {
        let length = reader.read_u16();
        let bytes = reader.read_bytes(length as usize);
        self.str = self.decode_m_utf8(bytes)
    }
    fn tag(&self) -> u8 {
        CONSTANT_UTF8
    }
}
impl ConstantUtf8Info{
    pub fn str(&self) -> String {
        self.str.to_string()
    }

    /**
     * @description
             字符串在class文件中是以MUTF-8（Modified UTF-8）方式编码的

     * @author Dylan
     * @throws
     * @time 2024/6/28 22:39
     */

    fn decode_m_utf8(&self,bytes: Vec<u8>) -> String {
       let utf_len=bytes.len();
        let mut char_arr = vec![0_u16; utf_len];
        let mut c;
        let mut char2;
        let mut char3;
        let mut count = 0_i32;
        let mut char_arr_count = 0_usize;
        loop {
            if (count as usize) >= utf_len {
                break;
            }
            c = bytes[count as usize] as u16;
            if c > 127 {
                break;
            }
            count += 1;
            char_arr[char_arr_count] = c;
            char_arr_count += 1;
        }
        loop {
            if (count as usize) >= utf_len {
                break;
            }
            c = bytes[count as usize] as u16;
            match c >> 4 {
                0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                    // 0xxxxxxx
                    count += 1;
                    char_arr[char_arr_count] = c;
                    char_arr_count += 1;
                },
                12 | 13 => {
                    // 110x xxxx   10xx xxxx
                    count += 2;
                    if (count as usize) > utf_len {
                        panic!("malformed input: partial character at end");
                    }
                    char2 = bytes[(count - 1) as usize] as u16;
                    if char2 & 0xC0 != 0x80 {
                        panic!("malformed input around byte {}", count)
                    }
                    char_arr[char_arr_count] = c & 0x1F << 6 | char2 & 0x3F;
                    char_arr_count += 1;
                },
                14 => {
                    // 1110 xxxx  10xx xxxx  10xx xxxx
                    count += 3;
                    if (count as usize) > utf_len {
                        panic!("malformed input: partial character at end");
                    }
                    char2 = bytes[count as usize - 2] as u16;
                    char3 = bytes[count as usize - 1] as u16;
                    if char2 & 0xC0 != 0x80 || char3 & 0xC0 != 0x80 {
                        panic!("malformed input around byte {}", count - 1)
                    }
                    char_arr[char_arr_count] = c & 0x0F << 12 | char2 & 0x3F << 6 | char3 & 0x3F << 0;
                    char_arr_count += 1;
                },
                // 10xx xxxx,  1111 xxxx
                _ => panic!("malformed input around byte {}", count)
            }
        }
        // The number of chars produced may be less than utf_len
        let char_arr = &char_arr[..char_arr_count];
        decode_utf16(char_arr.iter().cloned()).map(|r| r.unwrap()).collect::<String>()

    }
}