fn main() {
    // 1. println!
    let mut m = String::from("rust");
    // 打印 m 和 n(没有解引用)
    println!("m = {}", m);
    // dbg!将会失去所有权
    // dbg!(m);
    // m 追加字符串(没有失去所有权)
    m.push_str(" is easy.");
    // 打印 m
    println!("{}", m);

    // 2. 引用的引用
    let a: i32 = 4;
    let b: &i32 = &a;
    let c: &&i32 = &b;
    let d: &&&i32 = &c;
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);

    struct Rectangle {
        w: u32,
        h: u32,
    }
    let x: Rectangle = Rectangle { w: 3, h: 4 };
    let y: &Rectangle = &x;
    let z: &&Rectangle = &y;

    println!("x: w = {}, h = {}", x.w, x.h);
    println!("y: w = {}, h = {}", y.w, y.h);
    println!("z: w = {}, h = {}", z.w, z.h);

    // 3. 引用的比较
    let m = 1;
    let n = 2;
    let m1 = &m;
    let n1 = &n;

    // 比较
    if m1 < n1 {
        println!("m1 < n1");
    }
    // 下面的代码错误
    // if m1 < n { }

    // 4. 对表达式的引用
    // 4.1.普通表达式
    let x = &(8 + 78);
    println!("x = {}", x);

    // 4.2 闭包
    let f = |h| h + 1;
    let y = &f(1);
    println!("y = {}", y);

    // 4.3 函数
    fn func_test(a: i32) -> i32 {
        return a + 2;
    }
    let z = &func_test(3);
    println!("y = {}", z);
}
