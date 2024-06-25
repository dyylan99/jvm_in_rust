use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::classpath::entry::{absolute, Entry};

//目录形式的类路径
#[derive(Debug)]
pub struct DirEntry {
    pub abs_dir: String,
}

impl DirEntry{
    pub fn new(path: &str)->Self{
        DirEntry{
            abs_dir: absolute(path)
        }
    }
}


impl Display for DirEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Absolute Directory Path: {}", self.abs_dir)

    }
}

impl Entry for DirEntry {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        let path=Path::new(&self.abs_dir).join(class_name);
        println!("path:{}",path.display());
        let mut file=match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(format!("{}文件未找到:{}",class_name,e.to_string()))
        };
        let mut vec:Vec<u8>=Vec::new();
        file.read_to_end(&mut vec).map_err(|err|err.to_string())?;
        Ok(vec)
    }
}