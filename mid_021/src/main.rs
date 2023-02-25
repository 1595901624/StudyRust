fn main() {
    println!("[main start]");
    let name = String::from("zhangsan");
    let age = 16;
    let mut s = Student {
        name,
        age,
    };
    // 禁止执行下面一行代码
    // s.drop();

    // 释放 s 后 下面代码会报错
    // std::mem::drop(s);
    println!("student : name = {}, age = {}", s.name, s.age);

    // drop 发生的场景 之 【重新赋值】
    // s = Student {
    //     name: String::from("lisi"),
    //     age: 25,
    // };
    // println!("重新赋值 => student : name = {}, age = {}", s.name, s.age);

    // drop 发生的场景 之 【表达式的值被 `;` 表达式丢弃。】
    // s;

    // drop 发生的场景 之 【值被传递给函数或者方法中】
    print_student(s);

    println!("[main end]");
}

fn print_student(s: Student) {
    println!("[print_student] name = {}, age = {}", s.name, s.age);
}

/// 学生结构体
struct Student {
    name: String,
    age: u16,
}

/// 为 Student 实现 Drop trait
impl Drop for Student {
    fn drop(&mut self) {
        println!("[Student Drop] name = {}, age = {}", self.name, self.age);
    }
}

struct Dog {
    // 每条狗都有一个唯一 id
    id: i32,
}

// 不允许同时显式的实现 Copy 和 Drop trait。
impl Drop for Dog {
    fn drop(&mut self) {}
}