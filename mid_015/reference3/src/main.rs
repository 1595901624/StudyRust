use std::fmt::Arguments;
use std::io::{IoSlice, Write};

fn main() {
    // 1. 引用作为函数/方法参数
    let mut a = String::from("hello");
    print_string(&a);

    // 向 a 中追加字符串
    a.push_str(" rust!");
    print_string(&a);

    println!("println! 打印--- {}", a);

    // *******************下面的代码是错误示例 start***************
    // 注意：解引用会重新获得所有权
    // let aa = String::from("xxx");
    // operate_string(&aa);
    // *******************上面的代码是错误示例 start***************

    // 2.引用作为函数/方法返回值
    let mut b = String::from("hello world");
    let c = get_self(&mut b);
    println!("c = {}", c);
}

/// 打印一个字符串
fn print_string(s: &String) {
    println!("print_string 打印--- {}", s);
}

/// 引用作为函数/方法返回值
fn get_self(s: &String) -> &String {
    return s;
}

// fn operate_string(a: &String) {
    // 下面的代码是错误的
    // let x = *a;
// }
