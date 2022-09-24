#[derive(Default, Debug)]
struct Test1 {
    a: i32,
    b: f64,
    c: bool,
    d: char,
    e: (),
    f: String,
}

#[derive(Debug)]
struct Test2 {
    a: i32,
    b: f64,
    c: bool,
}

/// 默认实现 Default
impl Default for Test2 {
    fn default() -> Self {
        return Self {
            a: 10,
            b: 20.0,
            c: true,
        };
    }
}

/// 显式标明
struct MySized<T: ?Sized> {
    value: T,
}

fn main() {
    // 2. Default
    let test = Test1::default();
    println!("{:#?}", test);
    // 运行结果
    // Test1 {
    //     a: 0,
    //     b: 0,
    //     c: false,
    //     d: '\0',
    //     e: (),
    //     f: "",
    // }

    let test2 = Test2::default();
    println!("{:#?}", test2);
    // 运行结果
    // Test2 {
    //     a: 10,
    //     b: 20.0,
    //     c: true,
    // }
}
