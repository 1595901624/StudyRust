fn main() {
    println!("\n*******************1、可变长度字符串String和向量Vector************************");
    let mut hello = String::with_capacity(15);
    hello.push('h');
    hello.push('e');
    hello.push('l');
    hello.push('l');
    hello.push('o');

    println!("hello 字符串的内容 -> {}", hello);

    println!("hello 堆上的指针 -> {:p}", hello.as_ptr());
    println!("hello 的容量 -> {}", hello.capacity());
    println!("hello 的字节长度 -> {}", hello.len());

    println!("hello 栈上的指针 -> {:p}", &hello);

    // let hello_string = String::from("hello");
    // let hello_str = &hello[1..];
    // let hello_literal = "world";


}
