use my_macro::hello;


/// 函数宏测试
#[test]
pub fn test_function_macro() {
    hello! {
        println!("hello my macro!!!!");
        let a = 5;
        let b = 6;
        let c = a + b;
        println!("{} + {} = {}", a, b, c);
    }

    // 调用 hello_macro_internal
    hello_macro_internal();
}