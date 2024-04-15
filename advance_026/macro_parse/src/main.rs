use my_macro::{attribute_macro, MyDebug};
use syn::__private::ToTokens;
use syn::Expr;
use syn::spanned::Spanned;

fn main() {
    println!("Hello, world!");
}


#[attribute_macro]
fn test_attr() {

}

/// syn 解析 Rust 代码
#[test]
fn syn_test() -> Result<(), Box<dyn std::error::Error>> {
    // 表达式 a = 3
    let fn_str = r#"let a = 3"#;
    // 解析表达式
    let expr = syn::parse_str::<Expr>(fn_str)?;
    // 输出 TokenStream（proc_macro2::TokenStream）
    println!("{:#?}", expr.to_token_stream());
    Ok(())
}