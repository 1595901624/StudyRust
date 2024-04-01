use proc_macro::{TokenStream, TokenTree};
use std::fmt::format;


/// 自定义派生宏
#[proc_macro_derive(MyDebug)]
pub fn custom(input: TokenStream) -> TokenStream {
    // let input: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);

    // 派生宏的处理逻辑
    let output = input.to_string();
    // 分割代码
    let str_list = output.split::<&str>(" ").collect::<Vec<&str>>();
    if str_list.len() < 1 {
        return "".parse().unwrap();
    }
    // 获取结构体的名称
    let struct_name = str_list.get(1).unwrap();

    println!("struct_name {}", struct_name);

    let s = format!(r#"impl {} {{
    fn my_debug(&self) {{
        println!("{} 的自定义的派生宏!");
    }}
}}"#, struct_name, struct_name);
    s.parse().unwrap()
}

#[proc_macro]
pub fn my_function_macro(input: TokenStream) -> TokenStream {
    let output = input.to_string();
    // 定义一个函数 test_function_macro, 在函数里可以写 rust 语言。外部直接调用 test_function_macro 即可运行函数内的内容
    let output = format!("#[allow(unused)] fn test_function_macro() {{ {} }}", input);
    output.parse().unwrap()
}

// #[proc_macro]
// pub fn script(input: TokenStream) -> TokenStream {
//     // 解析 java 语法
//     // int a = 3;
//     // int b = 4;
//     // System.out.println(a + b);
//
//     // int语句
//     let int_syntax_regex = Regex::new(r#"\s*int\s+(\w+)\s*=\s*(\d+)\s*;"#).unwrap();
//     // 输出语句
//     let println_syntax_regex = Regex::new(r#"\s*System.out.println\((.*)\)\s*;"#).unwrap();
//
//     let input = input.to_string();
//
//     // 将 Java 代码转为 rust 代码后保存在codes变量中
//     let mut codes = String::new();
//
//     // 解析 int 语句
//     // let captures: Vec<Captures> = int_syntax_regex.captures_iter(&input).collect();
//     // println!("{:?}", captures);
//     let int_syntax_vec: Vec<(&str, [&str; 2])> = int_syntax_regex.captures_iter(&input).map(|cap| {
//         cap.extract()
//     }).collect();
//     int_syntax_vec.into_iter().for_each(|(_, [key, value])| {
//         // println!("key = {}, value = {}", key, value);
//         codes += &format!("let {} = {};\n", key, value);
//     });
//
//     let println_syntax_vec: Vec<(&str, [&str; 1])> = println_syntax_regex.captures_iter(&input).map(|cap| {
//         cap.extract()
//     }).collect();
//     println!("{:?}", println_syntax_vec);
//     println_syntax_vec.into_iter().for_each(|(_, [value])| {
//         // println!("key = {}, value = {}", key, value);
//         codes += &format!("println!(\"{{}}\", {});\n", value);
//     });
//
//     // println!("{:?}", codes);
//
//     let output = format!(r#"
//     fn run_script() {{
//         {}
//     }}
//     "#, codes);
//     output.parse().unwrap()
// }