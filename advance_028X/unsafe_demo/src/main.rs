fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = &x as *const i32;

    unsafe {
        // 打印 y
        println!("y = {:p}", y);
        // 打印 y.offset(1)
        println!("y.offset(1) = {:p}", y.offset(1));

        // 解引用 y 的值
        println!("*y = {}", *y);
        // 解引用 y.offset(1) 的值
        println!("*y.offset(1) = {}", *y.offset(1));

        println!("");
    }
}
