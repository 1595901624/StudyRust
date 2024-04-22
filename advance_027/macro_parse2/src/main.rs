use my_macro::MyDebug;
use proc_macro2::{Ident, Span};

fn main() {
    println!("Hello, world!");

    let student = Student;
    student.my_debug();
}

#[derive(MyDebug)]
struct Student;


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

#[test]
fn quote_test4() {
    let func_name = Ident::new("test_function", Span::call_site());
    let expand = quote::quote! {
       fn #func_name() {
           println!("这是一个动态命名的函数。");
       }
    };
    println!("expand: {}", expand);
}
