// 启用实验性特性
#![feature(log_syntax)]
#![feature(trace_macros)]

// 定义一个简单的声明宏
macro_rules! hello {
    () => {
        println!("Hello!");
    };

    // 使用 `$name` 来匹配传入的标识符
    ($name:expr) => {
        // 生成一个打招呼的字符串
        println!("Hello, {}!", $name);
    };
}

macro_rules! add {
    ($value1:expr, $value2:expr) => {
        {
            let x = $value1 + $value2;
            x
        }
    };
}

// 自定义的向量宏
macro_rules! my_vec {
    ($($value:expr),*) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($value);
            )* // 这里表示重复 0 到多次
            vec
        }
    };
}

fn main() {
    // 无参
    hello!();

    // 有参
    hello!("Rust");
    hello!("World");
    hello!(123456);

    // add
    trace_macros!(true);
    let x = add!(1,2);
    trace_macros!(false);
    println!("{}", x);

    // my_vec
    trace_macros!(true);
    let vec = my_vec!(1,2,3,4,5);
    trace_macros!(false);
    println!("{:?}", vec);

    let vec = my_vec![1,2,3,4,5];
    println!("{:?}", vec);

    let vec = my_vec! {1,2,3,4,5};
    println!("{:?}", vec);

    // log_syntax
    log_syntax! {
        add!(1,2);
    }
}
