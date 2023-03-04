use std::cell::{Cell, Ref, RefCell};

fn main() {
    // mut 创建一个 Student 实例
    let mut stu1 = Student1 {
        id: 1,
        name: "ZhangSan",
        age: 18,
        address: "北京".to_string(),
    };
    stu1.age = 18;
    stu1.address = "天津".to_string();
    println!("stu1 = {:?}", stu1);

    // 简单使用 Cell —— Copy类型 i32
    let cell = Cell::new(6);
    println!("cell 修改前:{}", cell.get());
    cell.set(9);
    println!("cell 修改后:{}", cell.get());

    // 简单使用 Cell —— 非Copy类型 String
    let cell = Cell::new(String::from("CELL"));
    // 下面一行代码会发生错误
    // println!("cell 修改前:{}", cell.get());
    cell.set(String::from("cell"));
    // 下面一行代码会发生错误
    // println!("cell 修改后:{}", cell.get());

    // stu2 并不需要可变
    let stu2 = Student2 {
        id: 2,
        name: "LiSi",
        age: Cell::new(20),
        address: String::from("上海"),
    };
    println!("stu2 修改年龄前: stu2 = {:?}", stu2);
    stu2.age.set(22);
    println!("stu2 修改年龄后: stu2 = {:?}", stu2);
}

/// 学生1 结构体
#[derive(Debug)]
struct Student1 {
    // 学号
    id: u32,
    // 姓名
    name: &'static str,
    // 年龄
    age: u32,
    // 地址
    address: String,
}

/// 学生2 结构体
#[derive(Debug)]
struct Student2 {
    // 学号
    id: u32,
    // 姓名
    name: &'static str,
    // 年龄
    age: Cell<u32>,
    // 地址
    address: String,
}

