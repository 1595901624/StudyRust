fn main() {
    println!("\n*******************1、type定义别名*********************");
    type Int = i32;
    type Float = f32;
    type Double = f64;
    type Char = char;

    let a: Int = 3;
    let b: Float = 4.5;
    let c: Double = 134.6753453424234231;
    let d: Char = '我';

    dbg!(a);
    dbg!(b);
    dbg!(c);
    dbg!(d);

    println!("\n*******************2、函数作为参数*********************");
    let a = 5;
    let b = 3;
    let add_result = operation_alias(add, a, b);
    let mul_result = operation(mul, a, b);

    dbg!(add_result);
    dbg!(mul_result);

    println!("\n*******************3、函数作为返回值*********************");
    let a = 4;
    let b = 6;
    let add_result = get_operation_alias("add")(a, b);
    let mul_result = get_operation("mul")(a, b);

    dbg!(add_result);
    dbg!(mul_result);
}

/// 函数声明别名
type Calc = fn(i32, i32) -> i32;

/// 操作运算(别名)
fn operation_alias(calc: Calc, a: i32, b: i32) -> i32 {
    return calc(a, b);
}

/// 操作运算
fn operation(calc: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    return calc(a, b);
}

/// 根据字符串获取函数(别名)
fn get_operation_alias(s: &str) -> Calc {
    if s == "add" {
        add
    } else {
        mul
    }
}

/// 根据字符串获取函数
fn get_operation(s: &str) -> fn(i32, i32) -> i32 {
    if s == "add" {
        add
    } else {
        mul
    }
}

/// 加法
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

/// 乘法
fn mul(a: i32, b: i32) -> i32 {
    return a * b;
}
