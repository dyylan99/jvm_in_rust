use crate::classpath::{Classpath,entry::Entry};
use crate::cmd::{Cmd, parse_cmd};

mod cmd;
mod classpath;


fn main() {
    let cmd=parse_cmd();

    if cmd.version_flag{
        println!("version 0.0.1");
    }else if cmd.help_flag || cmd.class=="" {
        cmd.print_usage();
    }else {
        start_jvm(cmd);
    }
}
fn start_jvm(cmd: Cmd) {
    let mut cp=Classpath::parse(&cmd.x_jre_option,&cmd.cp_options);
    println!("{:?}",&cmd);
    println!("===============================");
    println!("classpath: {} class:{} args: {:?}", cmd.cp_options, cmd.class, cmd.args);
    println!("===============================");

    let class_name=cmd.class.replace(".","/");
    // ep: java/lang/Object.class
    let class_data= match cp.read_class(&class_name){
        Ok(data)=>data,
        Err(_)=>{
            panic!("Could not find or load main class {}",cmd.class);
        }
    };
    println!("class data: {:?}", class_data);
}

