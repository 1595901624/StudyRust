fn main() {
    println!("***************1、编码*****************");
    let a = "a";
    let b = "©";
    let c = "汉";
    let d = "😃";

    println!("a 占 {} 个字节", std::mem::size_of_val(a));
    println!("b 占 {} 个字节", std::mem::size_of_val(b));
    println!("c 占 {} 个字节", std::mem::size_of_val(c));
    println!("d 占 {} 个字节", std::mem::size_of_val(d));

    println!("\n***************1、编码(打印二进制)*****************");
    for x in a.bytes() {
        print!("{:08b}_", x);
    }
    println!();
    for x in b.bytes() {
        print!("{:08b}_", x);
    }
    println!();
    for x in c.bytes() {
        print!("{:08b}_", x);
    }
    println!();
    for x in d.bytes() {
        print!("{:08b}_", x);
    }

    println!("\n***************1、编码(打印Unicode)*****************");
    println!("{:X}", 'a' as i32);
    println!("{:X}", '©' as i32);
    println!("{:X}", '汉' as i32);
    println!("{:X}", '😃' as i32);

    println!("\n***************2、字符串的长度*****************");
    let string_length = "我正在学习Rust~";
    println!("\"{}\"的字节长度 : {}", string_length, string_length.len());
    println!("\"{}\"的字符长度 : {}", string_length, string_length.chars().count());

    println!("\n***************3、迭代器访问字符串*****************");
    let string_nth = "Rust编程基础";

    // 访问第5个字符
    dbg!(string_nth.chars().nth(5));
    // 访问第5个字节
    dbg!(string_nth.bytes().nth(5));
}
