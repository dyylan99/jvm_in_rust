use crate::classfile::CLassFile;
use crate::classpath::{entry::Entry, Classpath};
use crate::cmd::{parse_cmd, Cmd};

mod classfile;
mod classpath;
mod cmd;

fn main() {
    let cmd = parse_cmd();

    if cmd.version_flag {
        println!("version 0.0.1");
    } else if cmd.help_flag || cmd.class == "" {
        cmd.print_usage();
    } else {
        start_jvm(cmd);
    }
}
fn start_jvm(cmd: Cmd) {
    let mut cp = Classpath::parse(&cmd.x_jre_option, &cmd.cp_options);
    println!("{:?}", &cmd);
    println!("===============================");
    println!(
        "classpath: {} class:{} args: {:?}",
        cmd.cp_options, cmd.class, cmd.args
    );
    println!("===============================");

    let class_name = cmd.class.replace(".", "/");
    // ep: java/lang/Object.class
    let class_data = match cp.read_class(&class_name) {
        Ok(data) => data,
        Err(_) => {
            panic!("Could not find or load main class {}", cmd.class);
        }
    };
    let class_file=load_class(class_name,cp);

    print_class_info(class_file);
    // println!("class data: {:?}", class_data);
}
fn load_class(class_name:String,mut class_path:Classpath)->CLassFile{
    //先通过输入的jre和class类名加载class文件数据,格式为字节数组
    let class_data=match class_path.read_class(&class_name) {
        Ok(data) => data,
        Err(e)=>panic!("{}",e)
    };
    let class_file=match CLassFile::parse(class_data){
        Ok(file)=>file,
        Err(e)=>panic!("{}",e)
    };
    class_file
}
fn print_class_info(class_file:CLassFile){
    println!("version: {}.{}",class_file.major_version(),class_file.minor_version());
    println!("constants count: {}",class_file.constant_pool().borrow().infos.len());
    println!("access flags: 0x{:X}",class_file.access_flags());
    println!("this class: {}",class_file.class_name());
    println!("super class: {}",class_file.super_class_name());
    println!("interfaces: {:?}",class_file.interface_names());
    println!("fields count: {}",class_file.fields().len());
    for field in class_file.fields().iter(){
        println!("字段类型描述符:{} ,字段名称:{}",field.descriptor(),field.name());
    }
    println!("methods count: {}",class_file.methods().len());
    for method in class_file.methods().iter(){
        println!("  {}",method.name());
    }
}
