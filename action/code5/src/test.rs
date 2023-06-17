use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[test]
fn test_rc() {
    // 创建一个字符串 rust
    let a: Rc<String> = Rc::new(String::from("rust"));
    // 调用 clone 方法，使指向字符串的引用计数 +1
    let b = a.clone();
    // Rc::clone(&a) 等价于 a.clone()
    let c = Rc::clone(&a);

    // 输出3个变量指向字符串的地址，以下三个地址输出是相同的
    println!("{:p}", a.as_ptr());
    println!("{:p}", b.as_ptr());
    println!("{:p}", c.as_ptr());

    // 查看引用计数
    println!("引用计数 {} ", Rc::strong_count(&a));
    // 下面的代码只做了解
    println!("弱引用计数 {} ", Rc::weak_count(&a));
}

#[test]
fn test_cell() {
    #[derive(Debug)]
    struct Student {
        // 学号
        id: u32,
        // 姓名
        name: &'static str,
        // 年龄
        age: Cell<u32>,
        // 地址
        address: String,
    }

    let stu = Student {
        id: 1,
        name: "XiaoLan",
        age: Cell::new(18),
        address: String::from("上海"),
    };
    println!("stu 修改年龄前: age = {}", stu.age.get());
    // 内部修改值
    stu.age.set(22);
    println!("stu 修改年龄后: age = {}", stu.age.get());
}

#[test]
fn test_refcell() {
    #[derive(Debug)]
    struct Student {
        // 学号
        id: u32,
        // 姓名
        name: &'static str,
        // 年龄
        age: Cell<u32>,
        // 地址
        address: RefCell<String>,
    }

    let stu = Student {
        id: 1,
        name: "XiaoLan",
        age: Cell::new(18),
        address: RefCell::new(String::from("上海")),
    };
    println!("stu 修改地址前: address = {}", stu.address.borrow());
    // 内部修改值
    stu.address.borrow_mut().push_str("浦东新区");

    println!("stu 修改地址后: address = {}", stu.address.borrow());
}

