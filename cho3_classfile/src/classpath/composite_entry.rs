use std::fmt::{Display, Formatter};
use crate::classpath::entry::{Entry, PATH_LIST_SEPARATOR};


/// 由多个 Entry 组成
pub struct CompositeEntry{
    entries:Vec<Box<dyn Entry>>
}

impl CompositeEntry {
    pub fn new(path_list:&str)->Self{
        let path_list=path_list.split(PATH_LIST_SEPARATOR);
        let mut entries=Vec::new();
        for path in path_list {
            entries.push(crate::classpath::entry::new_entry(path));
        }
        CompositeEntry{
            entries
        }
    }
}

impl Display for CompositeEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut vec = vec![];
        //将每个path的绝对路径拼接起来,再打印
        for entry in &self.entries {
            vec.push(format!("{}", entry))
        }
        write!(f, "{}", vec.join(&PATH_LIST_SEPARATOR.to_string()))
    }
}

impl Entry for CompositeEntry{
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
       for entry in &mut self.entries{
           match entry.read_class(class_name) {
               Ok(data)=>return Ok(data),
               Err(_)=>continue
           }
       }
        Err(format!("{}文件未找到",class_name))
    }
}