fn main() {
    println!("\n*******************1、字符串字面量*************************");
    // 未声明类型
    let hello1 = "hello study rust!";
    // 声明类型
    let hello2: &str = "hello study rust!";

    println!("hello1 = {}", hello1);
    println!("hello2 = {}", hello2);

    let hello3 = "I bought a book called \"Rust\"";
    println!("hello3 = {}", hello3);

    // 换行
    let hello4 = "My name is Betty, 18 years old. I like play piano very much and was awarded
    of a numbers of prizes for that.";
    println!("hello4 = {}", hello4);

    // 忽略换行符
    let hello5 = "My name is Betty, 18 years old. I like play piano very much and was awarded \
    of a numbers of prizes for that.";
    println!("hello5 = {}", hello5);

    // 原始字符串
    // 测试转义
    let raw_str = r"D:\study_rust\013\string";
    println!("raw_str = {}", raw_str);

    // 测试引号
    let raw_str_ref = r##"测试引号"英文引号",英文引号会原样输出！！"##;
    println!("raw_str_ref = {}", raw_str_ref);

    println!("\n*******************2、字节字符串字面量*************************");
    // 字节字符串
    let byte_str = b"a byte string!";
    println!("byte_str = {:?}", byte_str);

    // 原始字节字符串
    let raw_byte_str = br#"it is a "raw byte string"."#;
    println!("raw_str_ref = {:?}", raw_byte_str);

    println!("\n*******************3、String创建*************************");
    // 创建空字符串
    let empty_string = String::new();
    dbg!(empty_string);

    // 根据字符串字面量创建字符串
    let rust_str = "rust";
    let rust_string = String::from(rust_str);
    dbg!(&rust_string);

    println!("rust_str 字面量指向的地址 {:?}", rust_str.as_ptr());
    println!("rust_string 指向的地址 {:?}", &rust_string.as_ptr());

    // to_string
    let s1 = "rust_to_string";
    let s2 = s1.to_string();
    dbg!(s1);
    dbg!(s2);


}
