use proc_macro::{TokenStream, TokenTree};


#[proc_macro_attribute]
pub fn test_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    // `args` 用于接收属性参数的输入流，
    println!("attr: {:#?}", attr);
    // `item` 用于接收被处理的输入流。
    println!("item: {:#?}", item);

    // 收集属性参数和输入流中的元素
    let attr_vec: Vec<TokenTree> = attr.clone().into_iter().collect();
    let item_vec: Vec<TokenTree> = item.clone().into_iter().collect();
    println!("attr_vec: {:#?}", attr_vec);
    println!("item_vec: {:#?}", item_vec);

    // 暂时不做任何处理，直接返回原始输入。
    item
}

#[proc_macro_attribute]
pub fn my_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析属性宏
    let mut args = attr.into_iter();
    let value = args.next().unwrap();

    println!("value = {:#?}", value.span().source_text());

    // 替换掉我们自己的代码
    // 输出一条语句，输出的内容是属性定义的值
    let modified_item = format!(
        "fn main() {{ print!(\"Hello, \"); println!({}); }}",
        value
    );
    println!("modified_item: {:#?}", modified_item);
    // 返回我们自己的代码
    modified_item.parse().unwrap()
}
