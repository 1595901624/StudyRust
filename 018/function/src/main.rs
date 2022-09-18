fn main() {
    println!("\n******************1、函数定义********************");
    let a = 5;
    let b = 3;
    let c = add(a, b);
    println!("a + b = {}", c);

    println!("\n******************2、无返回值函数********************");
    let d = empty_no_return();
    dbg!(d);

    println!("\n******************3、高阶函数 函数指针********************");
    // 函数赋值，声明类型
    let say_hello_ptr: fn() = say_hello;
    say_hello_ptr();

    // 函数赋值，类型推断
    let other_say_hello_ptr= say_hello;
    other_say_hello_ptr();

    // 原函数调用
    say_hello()

}

fn say_hello() {
    println!("hello!")
}

/// 计算两个数的加法
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

/// 空函数，无返回值
fn empty_no_return() {}

