fn main() {
    println!("Hello, world!");
}


/// 插值
#[test]
fn quote_test() {
    let name = "Rust";
    let expand = quote::quote! {
        println!("Hello, {}!", #name);
    };
    println!("expand: {}", expand);
}

/// 重复
#[test]
fn quote_test2() {
    let list = vec!["a", "b", "c"];
    let expand = quote::quote! {
       let (#(#list),*) = (1, 2, 3);
   };
    println!("expand: {}", expand);
}

/// 嵌套
#[test]
fn quote_test3() {
    let name = "Rust";
    let expand = quote::quote! {
        quote::quote! {
            println!("Hello, {}!", #name);
        }
    };
    println!("expand: {}", expand);
}
