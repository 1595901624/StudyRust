use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn my_func(input: TokenStream) -> TokenStream {
    // 解析 java 语法
    // int a = 3;
    // int b = 4;
    // System.out.println(a + b);

    let input = input.to_string();
    // 分号隔开数据
    let input_vec = input.split(';').collect::<Vec<&str>>();
    println!("{:?}", input_vec);
    // 将 int a = 3; 转为 let a = 3;
    // for code in input_vec {
    //     let code = code.trim();
    //     // 正则判断是否是 a = 3 的形式
    // }

    let o = input.clone().replace("int a = 3", "let a = 3")
        .replace("int b = 4", "let b = 4")
        .replace("System.out.println(a + b)", "println!(\"{}\", a + b)");

    // let rust: TokenStream = o.parse().unwrap();

    println!("{}", o);

    let output = format!(r#"
    fn hello() {{
        {}
    }}
    "#, o);
    output.parse().unwrap()
    // TokenStream::new()
}