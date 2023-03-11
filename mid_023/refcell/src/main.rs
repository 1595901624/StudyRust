use std::any::Any;
use std::cell::{Cell, Ref, RefCell};
use std::rc::Rc;

fn main() {
    let name = String::from("ZhangSan");
    let name_refcell = RefCell::new(name);

    // ***************************borrow START******************************
    // borrow **下面代码执行时，请注释掉下方 borrow_mut 代码段**
    // let borrow1 = name_refcell.borrow();
    // let borrow2 = name_refcell.borrow();
    //
    // println!("borrow => {}", borrow1);
    // println!("borrow => {}", borrow2);
    // ***************************borrow E N D******************************


    // ***************************borrow_mut START******************************
    // borrow_mut **下面代码执行时，请上方注释掉 borrow 代码段**
    println!("修改前: {:?}", name_refcell);
    // 引用的生命期属于当前main作用域
    // let mut borrow_mut1 = name_refcell.borrow_mut();
    // borrow_mut1.push_str("Feng");
    // println!("修改后: {:?}", name_refcell);

    // name_refcell.borrow_mut(); 该行代码会发生错误
    // name_refcell.borrow(); 该行代码会发生错误

    // 解决办法：1 在不同作用域修改值
    {
        let mut borrow_mut1 = name_refcell.borrow_mut();
        borrow_mut1.push_str("Feng");
    }
    println!("修改后: {:?}", name_refcell);

    // 解决办法：2 在不同作用域修改值
    name_refcell.borrow_mut().push_str("Feng");
    println!("修改后: {:?}", name_refcell);

    // 解决办法：3 手动回收引用
    let mut borrow_mut1 = name_refcell.borrow_mut();
    borrow_mut1.push_str("Feng");
    std::mem::drop(borrow_mut1);
    println!("修改后: {:?}", name_refcell);

    // 解决办法：4 借助共享所有权 Rc
    let rc = Rc::new(name_refcell);
    rc.borrow_mut().push_str("Feng");
    println!("修改后: {:?}", rc);

    // ***************************borrow_mut E N D******************************

    // 上节的问题答案
    let id = 1;
    let name = "ZhangSan";
    let age = Cell::new(18);
    let address = RefCell::new("北京".to_string());
    let stu = Student { id, name, age, address };
    println!("stu 修改前: {:?}", stu);
    stu.age.set(20);
    stu.address.borrow_mut().clear();
    stu.address.borrow_mut().push_str("上海");

    println!("stu 修改后: {:?}", stu);
}

/// 学生 结构体
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
