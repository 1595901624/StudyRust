fn main() {
    println!("Hello, world!");

    // 调用模块中的函数
    module_demo::hello();

    outer_module::outer_function();
    outer_module::inner_module::inner_function();
}

// 模块demo
mod module_demo {
    // 常量
    const MAX_NUMBER: u32 = 100_000;

    // 函数
    pub fn hello() {
        println!("Hello, world!");
    }

    // 结构体
    struct User {
        name: String,
        age: u32,
    }
}

// 模块的嵌套
mod outer_module {
    // 外部模块的代码
    pub fn outer_function() {
        println!("这是一个外部模块的函数。");
    }


    pub mod inner_module {
        // 内部模块的代码
        pub fn inner_function() {
            println!("这是一个内部模块的函数。");
        }
    }
}
