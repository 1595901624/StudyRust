/*************************************************************************************
                        Rust 中级教程  公众号: Rust学习日记
**************************************************************************************/


fn main() {
    // 1、 泛型与向量
    let mut vec: Vec<&str> = vec![];
    vec.push("Hello");
    vec.push("Rust");

    println!("{:?}", vec);

    // 下面的语句会产生错误
    // vec.push(1);

    // 2、泛型与函数
    // 普通
    let a = print(3, 5);
    println!("a = {}", a);


    // 泛型
    let b = print_generic::<f32>(6.7, 4.5);
    println!("b = {}", b);

    let c = print_generic::<&str>("hello", "rust");
    println!("c = {}", c);

    // 3、泛型枚举
    let op = Some(5);
    println!("{:?}", op);
    // 输出结果
    // Some(5)

    // 除法
    let x1 = divider(5.0, 2.0);
    let x2 =  divider(5.0, 0.0);
    println!("{:?}", x1);
    println!("{:?}", x2);
}

fn print(a: i32, b: i32) -> i32 {
    return a + b;
}

fn print_generic<T>(a: T, b: T) -> T {
    return a;
}

/// 计算除法
fn divider(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}