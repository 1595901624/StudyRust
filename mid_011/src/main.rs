fn main() {

    // ********************1、Clone***********************

    let student1 = Student { name: String::from("xiaoming") };
    let student2 = student1.clone();

    println!("student1 = {:?}", student1);
    println!("student2 = {:?}", student2);

    // 创建 s1
    let s1 = String::from("Zhang San");
    // 深复制一个 s1 传递给 print_name 函数
    print_name(s1.clone());
    // 打印原来的 s1
    println!("{}", s1);

    // ********************2、Copy***********************
    let rec1 = Rectangle { width: 10, height: 5 };
    // 注意：同样需要使用 clone 方法
    let rec2 = rec1.clone();
    println!("rec1 的宽 : {}， 高 : {}", rec1.width, rec1.height);
    println!("rec2 的宽 : {}， 高 : {}", rec2.width, rec2.height);
}

fn print_name(name: String) {
    println!("My name is {}", name);
}

#[derive(Debug, Clone)]
struct Student {
    name: String,
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 下面是错误示范代码
// #[derive(Debug, Copy, Clone)]
// struct Test {
//     a: u32,
//     b: u32,
//     c: String,
// }