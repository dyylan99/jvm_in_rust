use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::classpath::composite_entry::CompositeEntry;
use crate::classpath::dir_entry::DirEntry;
use crate::classpath::wildcard_entry::WildcardEntry;
use crate::classpath::zip_entry::ZipEntry;

#[cfg(not(windows))]
pub const PATH_LIST_SEPARATOR: char = ':';

#[cfg(windows)]
pub const PATH_LIST_SEPARATOR: char = ';';

pub trait Entry: fmt::Display{
    /**
     * @description:
        readClass（）方法的参数是class文件的相对路径，路径之间用斜
        线（/）分隔，文件名有.class后缀。比如要读取java.lang.Object类，传
        入的参数应该是java/lang/Object.class。返回值是读取到的字节数
        据、最终定位到class文件的Entry，以及错误信息。
     * @author Dylan
     * @throws
     * @time 2024/6/25 12:48
     */

    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String>;
}
/**
 * @description 获取指定路径的绝对路径
 * @author Dylan
 * @throws
 * @time 2024/6/25 12:57
 */

pub fn absolute(path:&str)->String{
    let path=Path::new(&path);
    if !path.is_absolute() {
        let current_dir = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(e) => panic!("获取当前目录失败:{}", e)
        };
        let abs_path = current_dir.join(path);
        return abs_path.to_str().unwrap().to_string()
    }
    path.to_str().unwrap().to_string()
    // println!("path:{}",path.display());
    // match path.canonicalize() {
    //     Ok(path) => path.to_str().unwrap().to_string(),
    //     Err(e) => panic!("获取绝对路径错误:{}",e)
    // }

}
#[test]
pub fn test(){
    let s:&str="E:\\jdk1.8\\src";
    let path=Path::new(&s);
    let path = path.join("java\\lang\\Object.java");
    println!("path:{}",path.display());
    let mut file =match File::open(path){
        Ok(f)=>f,
        Err(e)=>{
            println!("文件未找到:{}",e);
            return;
        }
    };
    let mut vec:Vec<u8>=Vec::new();
    file.read_to_end(&mut vec).unwrap();
    println!("vec:{:?}",vec);

}


/// 根据传入的 path 创建对应的 Entry
/// -classpath aaa1/bbb1;aaa2/bbb2 => CompositeEntry
/// -classpath aaa/*               => WildcardEntry
/// -classpath aaa.jar             => ZipEntry
/// -classpath aaa                 => DirEntry

pub fn new_entry(path: &str)->Box<dyn Entry>{
    if path.contains(PATH_LIST_SEPARATOR){
        Box::new(CompositeEntry::new(path))
    }else if path.ends_with("*"){
        Box::new(WildcardEntry::new(path))
    }else if path.ends_with(".jar") || path.ends_with(".JAR") || path.ends_with(".zip") || path.ends_with(".ZIP"){
        Box::new(ZipEntry::new(path))
    }else{
        Box::new(DirEntry::new(path))
    }
}