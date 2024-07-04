use std::env;
use getopts::{Options, ParsingStyle};

#[derive(Debug)]
pub struct Cmd{
    pub help_flag: bool,
    pub version_flag: bool,
    pub cp_options: String,
    pub class: String,
    pub args: Vec<String>,
    pub x_jre_option: String
}
impl Cmd{
    pub fn print_usage(&self){
        let args:Vec<String>=env::args().collect();
        println!("Usage: {} [-options] class [args...]",args[0]);
    }
}

pub fn parse_cmd() -> Cmd {
    let mut cmd = Cmd {
        help_flag: false,
        version_flag: false,
        cp_options: String::new(),
        class: String::new(),
        args: Vec::new(),
        x_jre_option: String::new()
    };

    //获取命令行参数
    let args: Vec<String> = env::args().collect();
    //第一个参数为程序名
    let program=args[0].clone();
    let mut opts = Options::new();
    // ParsingStyle::StopAtFirstFree: 解析时剩余参数不作为标记参数一部分
    // long_only = true: 允许使用 -xxx
    let opts=opts.parsing_style(ParsingStyle::StopAtFirstFree).long_only(true);
    //optflag 不带参数
    opts.optflag("h", "help", "print the help message");
    opts.optflag("v", "version", "print version and exit");
    //optopt 带参数
    opts.optopt("", "classpath", "specify classpath", "classpath");
    opts.optopt("", "cp", "Specify the classpath", "classpath");
    opts.optopt("", "Xjre", "Specify the jre path", "jre");
    //从第二个参数开始解析所有的参数,返回一个Matches结构体,将符合的参数放到opts,未标记的参数放到free中
    let matches=match opts.parse(&args[1..]) {
        Ok(m) =>  m ,
        Err(f) => {
           print_usage(&program, opts);
            panic!("{}",f.to_string())
        }
    };
    //如果解析到了help参数,则将help_flag设置为true
    if matches.opt_present("help") {
        cmd.help_flag = true;
    }
    //如果解析到了version参数,则将version_flag设置为true
    if matches.opt_present("version") {
        cmd.version_flag = true;
    }
    //如果解析到了classpath或者cp参数,则将cp_options设置为参数值
    match matches.opt_str("classpath"){
        Some(classpath)=>{
            cmd.cp_options=classpath;
        },
        None=>{
            match matches.opt_str("cp"){
                Some(cp)=>cmd.cp_options=cp,
                None=>{}
            }
        }
    }
    //如果解析到了Xjre参数,则将x_jre_option设置为参数值
    match matches.opt_str("Xjre"){
        Some(jre)=>{
            cmd.x_jre_option=jre;
        },
        None=>{}
    }
    // 未定义的参数放在 free Vec 中
    if !matches.free.is_empty() {
        //第一个未定义的参数为类名
        cmd.class = matches.free[0].clone();
        //剩余的参数为参数列表
        cmd.args = matches.free[1..].to_vec();
    }
    cmd
}
pub fn print_usage(program: &str, opts: &mut Options) {
    let brief = format!("Usage: {} [-options] class [args...]", program);
    println!("{}", opts.usage(&brief));
}