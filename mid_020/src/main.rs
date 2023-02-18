use std::ops::Deref;

fn main() {

    // 在堆上分配内存，然后将 20 放入。
    let a: Box<i32> = Box::new(20);

    // 解引用
    let b = a.deref();
    let c = *a;
    println!("使用 deref() 解引用: {}", b);
    println!("使用 * 解引用: {}", c);
}
