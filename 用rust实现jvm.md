# 第一章.处理命令行输入参数

## 1.1 Cmd结构体

先定义一个结构体存储命令行输入信息,其中**cp_options**为classpath参数,class为目标类名,args则为其他参数

命令行参数解析规则: 第一个未定义的参数默认为类名,其余的参数为参数列表

```rust
pub struct Cmd{
    pub help_flag: bool,
    pub version_flag: bool,
    pub cp_options: String,
    pub class: String,
    pub args: Vec<String>,
}
```



在rust中,**getopts**库可以专门用来处理命令行输入问题,引入依赖后只需要new一个Options实力即可使用:

```rust
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
    // 未定义的参数放在 free Vec 中
    if !matches.free.is_empty() {
        //第一个未定义的参数为类名
        cmd.class = matches.free[0].clone();
        //剩余的参数为参数列表
        cmd.args = matches.free[1..].to_vec();
    }
    cmd
```

---



# 第二章.搜索加载class文件

## 2.1 类路径

常见的Java虚拟机的类路径可以分为以下三个部分:

+ 启动类路径（bootstrap classpath）
+ 扩展类路径（extension classpath）
+ 用户类路径（user classpath）

启动类路径默认对应**jre\lib**目录，Java标准库（大部分在rt.jar里）位于该路径。扩展类路径默认对应jre\lib\ext目录，使用Java扩展机制的类位于这个路径。**我们自己实现的类，以及第三方类库则位于用户类路径。**可以通过-Xbootclasspath选项修改启动类路径

用户类路径的默认值是当前目录,也可以通过-classpath和-cp命令行参数自定义路径,-classpath/-cp选项既可以指定目录，也可以指定JAR文件或者ZIP文件，如下：

```
java -cp path\to\classes ...
java -cp path\to\lib1.jar ...
java -cp path\to\lib2.zip ...
```

还可以同时指定多个目录或文件，用分隔符分开即可。分隔符因操作系统而异。在Windows系统下是分号，在类UNIX（包括Linux、Mac OS X等）系统下是冒号。例如在Windows下：

```
java -cp path\to\classes;lib\a.jar;lib\b.jar;lib\c.zip ...
```

从Java 6开始，还可以使用通配符（*）指定某个目录下的所有JAR文件，格式如下：

```
java -cp classes;lib\* ...
```

## 2.2 组合模式实现不同类路径加载

可以把类路径想象成一个大的整体，它由**启动类路径、扩展类路径和用户类路径**三个小路径构成。三个小路径又分别由更小的路

径构成。

### 2.2.1 Entry接口

先定义一个接口来表示类路径项:

```rust
pub trait Entry: fmt::Display{
    /**
     * @description:
        readClass（）方法的参数是class文件的相对路径，路径之间用斜
        线（/）分隔，文件名有.class后缀。比如要读取java.lang.Object类，传
        入的参数应该是java/lang/Object.class。返回值是读取到的字节数
        组、最终定位到class文件的Entry，以及错误信息。
     * @author Dylan
     * @throws
     * @time 2024/6/25 12:48
     */

    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String>;
}
```

```rust
/// 根据传入的 path 创建对应的 Entry
/// -classpath aaa1/bbb1;aaa2/bbb2 => CompositeEntry: 复合路径
/// -classpath aaa/*               => WildcardEntry: 包含星号的通配符路径
/// -classpath aaa.jar             => ZipEntry: 压缩包路径
/// -classpath aaa                 => DirEntry: 目录路径

pub fn new_entry(path: &str) -> Box<dyn Entry> {
    if path.contains(PATH_LIST_SEPARATOR) {
        Box::new(CompositeEntry::new(path))
    } else if path.ends_with("*") {
        Box::new(WildcardEntry::new(path))
    } else if path.ends_with(".jar")
        || path.ends_with(".JAR")
        || path.ends_with(".zip")
        || path.ends_with(".ZIP")
    {
        Box::new(ZipEntry::new(path))
    } else {
        Box::new(DirEntry::new(path))
    }
}

```

### 2.2.2 DirEntry

DirEntry，表示目录形式的类路径

```rust
//目录形式的类路径
#[derive(Debug)]
pub struct DirEntry {
    pub abs_dir: String,
}
```

DirEntry只有一个字段，用于存放目录的绝对路径,实现Entry接口:

```rust

impl Entry for DirEntry {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
         //拼接目标类的绝对路径
        let path = Path::new(&self.abs_dir).join(class_name);
        println!("path:{}", path.display());
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(format!("{}文件未找到:{}", class_name, e.to_string())),
        };
        let mut vec: Vec<u8> = Vec::new();
        //读取文件内容,并返回字节数组
        file.read_to_end(&mut vec).map_err(|err| err.to_string())?;
        Ok(vec)
    }
}
```

### 2.2.3 ZipEntry

ZipEntry表示ZIP或JAR文件形式的类路径.

```rust
// ZIP 或 JAR 文件形式的类路径
pub struct ZipEntry{
    abs_path:String,
    zip_archive:ZipArchive<File>
}
```

在rust中,可以使用zip库来读取压缩文件,可以使用`ZipArchive`来打开一个ZIP文件，遍历其内容，或者读取特定的文件

```rust
impl Entry for ZipEntry{
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        let archive=&mut self.zip_archive;
        //利用zip库的by_name方法获取zip文件中的目标类文件
        let mut file=match archive.by_name(&class_name) {
            Ok(f)=>f,
            Err(e)=>return Err(format!("{}文件未找到:{}",class_name,e.to_string()))
        };
        let mut vec:Vec<u8>=vec![];
        file.read_to_end(&mut vec).map_err(|err|err.to_string())?;
        Ok(vec)
    }
}
```

### 2.2.4 CompositeEntry

CompositeEntry由多个Entry组成:

```rust
/// 由多个 Entry 组成
pub struct CompositeEntry{
    entries:Vec<Box<dyn Entry>>
}

```

由其特殊路径形式构建CompositeEntry,分割路径,再通过Entry文件中的new_Entry按照类型创建特定的Entry:

```rust
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
```

CompositeEntry实现的Entry接口的read_class方法,由于其是Entry数组,因此从前往后遍历Entry数组,只需要加载到目标类即可返回需要的字节数组:

```rust
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
```



### 2.2.5 WildcardEntry

WildcardEntry是带通配符的路径试题,和CompositeEntry一样,其由Entry数组组成:

```rust
// 处理以 * 结尾的类路径
// 例如：/usr/local/java/*
pub struct WildcardEntry {
    entries: Vec<Box<dyn Entry>>
}
```

遍历该目录下的所有文件,筛选出以jar文件,封装成ZipEntry.

```rust
pub fn new(path: &str) -> Self {
        //移除 * 号
        let base_dir = &path[..path.len() - 1];

        //返回一个迭代器,包含目录中的所有条目
        let dir = match fs::read_dir(base_dir) {
            Ok(dir) => dir,
            Err(e) => panic!("读取目录{}失败:{}", base_dir, e.to_string()),
        };

        let convert = |entry| -> Box<dyn Entry> { Box::new(entry) };
        let mut entries = Vec::new();
        for dir_entry in dir {
            let path = dir_entry.unwrap().path();
            if path.is_dir() {
                continue;
            }
            let p = path.to_str().unwrap();
            //筛选出以 .jar 或 .JAR 结尾的文件,封装成ZipEntry,用zip读取jar压缩文件中的内容
            if p.ends_with(".jar") || p.ends_with(".JAR") {
                //创建ZipEntry
                let zip_entry = ZipEntry::new(&path.to_str().unwrap());
                entries.push(convert(zip_entry));
            }
        }
        WildcardEntry { entries }
    }
```

Entry接口实现:和CompositeEntry逻辑一样

```rust
impl Entry for WildcardEntry {
    /**
     * @description  根据类名从一个目录中读取class文件,读取到即返回
     * @author Dylan
     * @throws
     * @time 2024/6/25 18:35
     */

    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        for entry in &mut self.entries {
            match entry.read_class(&class_name) {
                Ok(data) => return Ok(data),
                Err(_) => continue,
            }
        }
        Err(format!("{}文件未找到", class_name))
    }
}

```

## 2.3 Classpath结构体

Classpath结构体有三个字段，分别存放三种类路径,Parse（）函数使用-Xjre选项解析启动类路径和扩展类路径，使用-classpath/-cp

选项解析用户类路径

```rust
pub struct Classpath{
    boot_classpath: Box<dyn entry::Entry>,
    ext_classpath: Box<dyn entry::Entry>,
    user_classpath: Box<dyn entry::Entry>,
}
```

```rust
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

```

ReadClass（）方法**依次**从启动类路径、扩展类路径和用户类路径中搜索class文件

```rust
impl Entry for Classpath {
    fn read_class(&mut self, class_name: &str) -> Result<Vec<u8>, String> {
        let class = class_name.to_string() + ".class";
        return match self.boot_classpath.read_class(&class) {
            Ok(data) => Ok(data),
            Err(_) => match self.ext_classpath.read_class(&class) {
                Ok(data) => Ok(data),
                Err(_) => match self.user_classpath.read_class(&class) {
                    Ok(data) => Ok(data),
                    Err(e) => Err(e),
                },
            },
        };
    }
}

```

