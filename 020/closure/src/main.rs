fn main() {
    println!("\n*****************1、闭包示例******************");
    let add = |a, b| a + b;
    // let add: fn(i32, i32) -> i32 = |a, b| a + b;

    let result = add(1, 2);
    // let result = add(1.2, 3.4);
    dbg!(result);

    println!("\n*****************2、闭包类型******************");
    let add_1 = |a: i32, b: i32| -> i32 { a + b };
    let add_2 = |a, b| { a + b };
    let add_3 = |a, b| a + b;

    dbg!(add_function(0, 1));
    dbg!(add_1(1, 2));
    dbg!(add_2(2, 3));
    dbg!(add_3(3, 4));

    println!("\n*****************3、捕获变量******************");
    let k = 8;
    let add_var = |a| a + k;

    let result = add_var(10);
    dbg!(result);
}

fn add_function(a: i32, b: i32) -> i32 {
    a + b
}
