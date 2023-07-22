extern crate proc_macro;

// 定义函数宏 hello
#[proc_macro]
pub fn hello(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // 在输入的代码包裹成一个函数体，函数名为 hello_macro_internal
    let output = format!("#[allow(unused)] fn hello_macro_internal() {{ {} }}", input);
    output.parse().expect("Failed to parse token stream")
}

