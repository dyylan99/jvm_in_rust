use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;
use crate::classpath::entry::{absolute, Entry};

// ZIP 或 JAR 文件形式的类路径
pub struct ZipEntry{
    abs_path:String,
    zip_archive:ZipArchive<File>
}

impl ZipEntry{
    pub fn new(path: &str)->Self{
        let abs_path=absolute(path);
        let path=Path::new(&abs_path);

        let zip_file=match File::open(path) {
            Ok(file)=>file,
            Err(e)=>panic!("打开文件{}失败:{}",&path.display(),e.to_string())
        };
        ZipEntry{
            abs_path,
            zip_archive:ZipArchive::new(zip_file).unwrap()
        }
    }
}

impl Display for ZipEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.abs_path)
    }
}

impl Entry for ZipEntry{
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        let archive=&mut self.zip_archive;
        //利用zip库的by_name方法获取zip文件中的文件
        let mut file=match archive.by_name(&class_name) {
            Ok(f)=>f,
            Err(e)=>return Err(format!("{}文件未找到:{}",class_name,e.to_string()))
        };
        let mut vec:Vec<u8>=vec![];
        file.read_to_end(&mut vec).map_err(|err|err.to_string())?;
        Ok(vec)
    }
}