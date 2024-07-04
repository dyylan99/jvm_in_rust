use crate::classfile::CLassFile;
use crate::classpath::{entry::Entry, Classpath};
use crate::cmd::{parse_cmd, Cmd};
use crate::rtda::frame::Frame;
use crate::rtda::local_vars::LocalVars;
use crate::rtda::operand_stack::OperandStack;

mod classfile;
mod classpath;
mod cmd;
mod rtda;

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
    // let cp = Classpath::parse(&cmd.x_jre_option, &cmd.cp_options);
    // println!("{:?}", &cmd);
    // println!("===============================");
    // println!(
    //     "classpath: {} class:{} args: {:?}",
    //     cmd.cp_options, cmd.class, cmd.args
    // );
    // println!("===============================");
    //
    // let class_name = cmd.class.replace(".", "/");
    //
    // let class_file = load_class(class_name, cp);
    //
    // print_class_info(class_file);
    let mut frame=Frame::new(100,100);
    test_local_vars(frame.local_vars_mut());
    test_operand_stack(frame.operand_stack_mut())
}
fn test_local_vars(local_vars: &mut LocalVars) {
    local_vars.set_int(0, 100);
    local_vars.set_int(1, -100);
    local_vars.set_long(2, 2997924580);
    local_vars.set_long(4, -2997924580);
    local_vars.set_float(6, 3.1415926);
    local_vars.set_double(7, 2.71828182845);
    local_vars.set_ref(9, None);

    println!("{}", local_vars.get_int(0));
    println!("{}", local_vars.get_int(1));
    println!("{}", local_vars.get_long(2));
    println!("{}", local_vars.get_long(4));
    println!("{}", local_vars.get_float(6));
    println!("{}", local_vars.get_double(7));
    println!("{:?}", local_vars.get_ref(9));
}
fn test_operand_stack(operand_stack: &mut OperandStack) {
    operand_stack.push_int(100);
    operand_stack.push_int(-100);
    operand_stack.push_long(2997924580);
    operand_stack.push_long(-2997924580);
    operand_stack.push_float(3.1415926);
    operand_stack.push_double(2.71828182845);
    operand_stack.push_ref(None);

    println!("{:?}", operand_stack.pop_ref());
    println!("{}", operand_stack.pop_double());
    println!("{}", operand_stack.pop_float());
    println!("{}", operand_stack.pop_long());
    println!("{}", operand_stack.pop_long());
    println!("{}", operand_stack.pop_int());
    println!("{}", operand_stack.pop_int());
}
fn load_class(class_name: String, mut class_path: Classpath) -> CLassFile {
    //先通过输入的jre和class类名加载class文件数据,格式为字节数组
    let class_data = match class_path.read_class(&class_name) {
        Ok(data) => data,
        Err(e) => panic!("{}", e),
    };
    let class_file = match CLassFile::parse(class_data) {
        Ok(file) => file,
        Err(e) => panic!("{}", e),
    };
    class_file
}

fn print_class_info(class_file: CLassFile) {
    println!(
        "version: {}.{}",
        class_file.major_version(),
        class_file.minor_version()
    );
    println!(
        "constants count: {}",
        class_file.constant_pool().borrow().infos.len()
    );
    println!("access flags: 0x{:X}", class_file.access_flags());
    println!("this class: {}", class_file.class_name());
    println!("super class: {}", class_file.super_class_name());
    println!("interfaces: {:?}", class_file.interface_names());
    println!("fields count: {}", class_file.fields().len());
    for field in class_file.fields().iter() {
        println!(
            "字段类型描述符:{} ,字段名称:{}",
            field.descriptor(),
            field.name()
        );
    }
    println!("methods count: {}", class_file.methods().len());
    for method in class_file.methods().iter() {
        println!("  {}", method.name());
    }
}
