use proc_macro::{TokenStream, TokenTree};
use std::fmt::format;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

/// 自定义派生宏
#[proc_macro_derive(MyDebug)]
pub fn custom(input: TokenStream) -> TokenStream {
    // 派生宏的处理逻辑
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = ast.ident.to_token_stream();

    // 下节内容
    let expand = quote::quote! {
        impl #struct_name {
                fn my_debug(&self) {{
                    println!("自定义的派生宏!");
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