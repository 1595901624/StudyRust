use std::rc::Rc;

fn main() {
    // 创建一个字符串 rust
    let a: Rc<String> = Rc::new(String::from("rust"));
    // 调用 clone 方法，使指向字符串的引用计数 +1
    let b = a.clone();
    // Rc::clone(&a) 等价于 a.clone()
    let c = Rc::clone(&a);

    // 输出3个变量指向字符串的地址
    println!("a 的地址: {:p}", a.as_ptr());
    println!("b 的地址: {:p}", b.as_ptr());
    println!("c 的地址: {:p}", c.as_ptr());

    // 查看引用计数
    println!("引用计数 {} ", Rc::strong_count(&a));
    // 下面的代码只做了解
    println!("弱引用计数 {} ", Rc::weak_count(&a));


    // 使用 String 中的方法。
    if a.contains("ru") {
        println!("true");
    }
}

