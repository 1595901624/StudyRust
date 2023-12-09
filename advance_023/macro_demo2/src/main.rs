use my_macro::{my_main, test_macro};

#[my_main("Rust")]
fn main() {
    test();
}

#[test_macro(name = "zhangsan", age = 18)]
fn test() {
    println!("Hello, world!");
}