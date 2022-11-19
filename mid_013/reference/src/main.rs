fn main() {
    // 1.共享引用
    // String类型
    let m = String::from("rust");
    // 引用： &m 表示 对x的共享引用
    let n = &m;
    println!("m = {}, n = {}", m, *n);

    // 2.可变引用
    let mut a = String::from("rust");
    let b = &mut a;
    b.push_str(" is so easy!");
    println!("{}", b);
    format!("");

    // 3.显式解引用
    let x = 888;
    let y = &x;
    println!(" x = {}, y = {}", x, *y);

    // 4.隐式解引用
    let mut a = String::from("rust");
    (&mut a).push_str(" is so easy!");
    println!("{}", a);


}



