use std::str::{from_utf8, from_utf8_unchecked};

fn main() {
    let rust = vec![72, 101, 108, 108, 111, 44, 32, 82, 117, 115, 116, 33];
    println!("安全版本 rust: {}", from_utf8(&rust).unwrap());

    unsafe {
        println!("不安全版本 rust: {}", from_utf8_unchecked(&rust));
    }

    // 255 是一个错误的编码
    let err_rust = vec![72, 101, 108, 108, 111, 44, 32, 82, 117, 115, 116, 33, 255];

    println!("安全版本 err_rust: {:#?}", from_utf8(&err_rust));

    unsafe {
        println!("不安全版本 err_rust: {}", from_utf8_unchecked(&err_rust));
    }
}

