use my_macro::{my_function_macro, script};

// 自定义语言
script! {
    int a = 6;
    int b = 4;
    System.out.println(a + b);
    System.out.println("hello script!");
}

fn main() {
    run_script();
    // my_function_macro! {
    //     println!("我在函数宏中...");
    // }
    //
    // test_function_macro();
}
