/// 函数宏测试
#[test]
pub fn test_function_macro() {
    use my_macro::hello;

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

/// 属性宏测试
#[test]
fn test_attribute_macro() {
    use my_macro::hello_attribute;

    #[hello_attribute]
    fn test() {
    }
    test();
}


// 派生宏测试
#[test]
fn test_derive_macro() {
    use my_macro::LQPrint;

    pub trait LQPrint {
        fn lq_print(&self);
    }

    #[derive(LQPrint)]
    struct Student;

    let student = Student;
    student.lq_print();
}