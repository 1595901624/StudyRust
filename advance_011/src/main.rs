fn main() {
    // panic
    // assert_eq!(3, 4);

    // 捕获panic
    let result = std::panic::catch_unwind(|| {
        // panic
        assert_eq!(3, 4);
        // 发生 panic 后的代码不会执行
        println!("hello");
    });

    println!("result: {:?}", result);
    println!("exit main");

    // panic!("test");
}

