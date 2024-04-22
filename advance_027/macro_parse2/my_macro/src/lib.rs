use proc_macro::{TokenStream, TokenTree};
use std::fmt::format;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

/// 自定义派生宏
#[proc_macro_derive(MyDebug)]
pub fn custom(input: TokenStream) -> TokenStream {
    // 派生宏的处理逻辑
    let ast = parse_macro_input!(input as DeriveInput);

    // 结构体名称 TokenStream
    let struct_token_stream = ast.ident.to_token_stream();

    // 名称字符串
    let struct_name = struct_token_stream.to_string();

    let expand = quote::quote! {
        // 在代码中需要使用 TokenStream
        impl #struct_token_stream {
                fn my_debug(&self) {{
                    println!("{} 自定义的派生宏!", #struct_name);
                }}
        }
    };
    expand.into()
}

#[proc_macro_attribute]
pub fn attribute_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(item as syn::ItemFn);

    item.to_token_stream().into()
}