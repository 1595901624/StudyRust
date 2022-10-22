fn main() {
    // 1. 转移

    // 1.1 基本数据类型
    let a = 3;
    let b = a;

    println!("a = {}, b = {}", a, b);

    // 1.2 String类型
    let m = String::from("rust");
    // 让渡所有权
    let n = m;

    // m 变为无法访问，下面的代码将会报错
    // println!("m = {}, n = {}", m, n);

    println!("n = {}", n);

    // 变量遮蔽
    let m = 5;
    println!("m = {}", m);

    let a = vec![String::from("hello"), String::from("study"), String::from("rust")];
    let b = a;
    // 下面的语句会报错
    // let c = a;
}
