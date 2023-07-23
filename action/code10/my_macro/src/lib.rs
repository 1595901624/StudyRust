extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// 定义函数宏 hello
#[proc_macro]
pub fn hello(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // 在输入的代码包裹成一个函数体，函数名为 hello_macro_internal
    let output = format!("#[allow(unused)] fn hello_macro_internal() {{ {} }}", input);
    output.parse().expect("Failed to parse token stream")
}

#[proc_macro_attribute]
pub fn hello_attribute(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("这是一个属性宏");
    item
}

/// 派生宏
#[proc_macro_derive(LQPrint)]
pub fn hello_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl LQPrint for #name {
            fn lq_print(&self) {
                println!("hello lanqiao derive macro");
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}