use std::env;
use std::fmt::{Display, Formatter};
use std::path::Path;
use crate::classpath::entry::{Entry, new_entry};
use crate::classpath::wildcard_entry::WildcardEntry;

pub mod entry;
mod dir_entry;
mod zip_entry;
mod composite_entry;
mod wildcard_entry;


pub struct Classpath{
    boot_classpath: Box<dyn entry::Entry>,
    ext_classpath: Box<dyn entry::Entry>,
    user_classpath: Box<dyn entry::Entry>,
}

impl Classpath{
    pub fn parse(jre_option: &str, cp_option: &str)->Self{
        let boot_classpath=Self::parse_boot_classpath(jre_option);
        let ext_classpath=Self::parse_ext_classpath(jre_option);
        let user_classpath=Self::parse_user_classpath(cp_option);
        Classpath{
            boot_classpath,
            ext_classpath,
            user_classpath
        }
    }

    /**
     * @description 加载bootstrap classpath=> jre/lib
     * @author Dylan
     * @throws
     * @time 2024/6/25 16:12
     */

    fn parse_boot_classpath(jre_option: &str)->Box<dyn Entry>{
        let jre_dir=Classpath::get_jre_dir(jre_option);

        // jre/lib/*
        let path=Path::new(&jre_dir).join("lib").join("*");
        let jre_lib_path=path.to_str().unwrap();
        Box::new(WildcardEntry::new(jre_lib_path))
    }
    fn parse_ext_classpath(jre_option: &str)->Box<dyn Entry>{
        let jre_dir=Classpath::get_jre_dir(jre_option);

        // jre/lib/ext/*
        let path=Path::new(&jre_dir).join("lib").join("ext").join("*");
        let jre_ext_path=path.to_str().unwrap();
        Box::new(WildcardEntry::new(jre_ext_path))
    }
    fn parse_user_classpath(cp_option: &str) -> Box<dyn Entry> {
        let mut cp = cp_option;
        if cp == "" {
            cp = ".";
        }
        new_entry(cp)
    }

    fn get_jre_dir(jre_option: &str) -> String {
        if jre_option != "" {
            let jre_dir = Path::new(jre_option);
            if jre_dir.exists() {
                // 使用用户输入的 -Xjre 选项作为 jre 目录
                return jre_option.to_string();
            }
        }
        let jre_dir = Path::new("./jre");
        if jre_dir.exists() {
            // 使用当前目录下的 jre 目录
            return "./jre".to_string();
        }
        // 使用 JAVA_HOME 环境变量
        match env::var("JAVA_HOME") {
            Ok(jh) => {
                if jh != "" {
                    return Path::new(&jh).join("jre")
                        .to_str().unwrap().to_string();
                }
            },
            Err(_err) => {},
        }
        panic!("{}", "Can not find jre folder!")
    }
}

impl Display for Classpath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.user_classpath)
    }
}

impl Entry for Classpath{
   fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        let class = class_name.to_string() + ".class";
        return match self.boot_classpath.read_class(&class) {
            Ok(data) => Ok(data),
            Err(_)=>{
                match self.ext_classpath.read_class(&class) {
                    Ok(data) => Ok(data),
                    Err(_)=>{
                        match self.user_classpath.read_class(&class){
                            Ok(data) => Ok(data),
                            Err(e) => Err(e)
                        }
                        }
                    }
                }
            }
    }
}