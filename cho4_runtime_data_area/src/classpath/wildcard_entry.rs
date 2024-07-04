use std::fmt::{Display, Formatter};
use std::fs;
use crate::classpath::entry::{Entry, PATH_LIST_SEPARATOR};
use crate::classpath::zip_entry::ZipEntry;


// 处理以 * 结尾的类路径
// 例如：/usr/local/java/*
pub struct WildcardEntry {
    entries: Vec<Box<dyn Entry>>
}

impl WildcardEntry {
    pub fn new(path: &str)->Self{
        //移除 * 号
        let base_dir = &path[..path.len()-1];

        //当前目录下的所有文件以条目的形式存储,可以遍历
        let dir=match fs::read_dir(base_dir) {
            Ok(dir)=>dir,
            Err(e)=>panic!("读取目录{}失败:{}",base_dir,e.to_string())
        };

        let convert=|entry|->Box<dyn Entry>{
            Box::new(entry)
        };
        let mut entries=Vec::new();
        for dir_entry in dir{
            let path=dir_entry.unwrap().path();
            if path.is_dir(){
                continue;
            }
            let p=path.to_str().unwrap();
            //筛选出以 .jar 或 .JAR 结尾的文件,封装成ZipEntry,用zip读取jar压缩文件中的内容
            if p.ends_with(".jar") || p.ends_with(".JAR") {
                //创建ZipEntry
                let zip_entry=ZipEntry::new(&path.to_str().unwrap());
                entries.push(convert(zip_entry));
            }
        }
        WildcardEntry{
            entries
        }
    }
}

impl Display for WildcardEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut vec = vec![];
        for entry in &self.entries {
            vec.push(format!("{}", entry))
        }
        write!(f, "{}", vec.join(&PATH_LIST_SEPARATOR.to_string()))
    }
}

impl Entry for WildcardEntry{
    /**
     * @description  根据类名从一个目录中读取class文件,读取到即返回
     * @author Dylan
     * @throws
     * @time 2024/6/25 18:35
     */

    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        for entry in &mut self.entries{
            match entry.read_class(&class_name) {
                Ok(data)=>return Ok(data),
                Err(_)=>continue
            }
        }
        Err(format!("{}文件未找到",class_name))
    }
}