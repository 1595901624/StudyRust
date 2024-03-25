use my_macro::MyDebug;

fn main() {
    println!("Hello, world!");
    let s = Student;
    s.my_debug();

    let p = People {name: "zhangsan".to_string(), age: 18};
    p.my_debug();
}


#[derive(MyDebug)]
struct Student;

#[derive(MyDebug)]
struct People {
    name: String,
    age: i32,
}


// impl Student {
//     fn my_debug(&self) {
//         println!("自定义的派生宏!");
//     }
// }