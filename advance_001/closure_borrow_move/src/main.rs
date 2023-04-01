fn main() {
    // 1. 闭包与借用
    let hello = "hello rust".to_string();
    let c = || {
        println!("{}", hello);
    };
    c();
    // println!("{}", hello);

    // 1.1 Fn
    let x = 10;
    let c1 = || {
        println!("{}", x);
    };
    c1();
    c1();
    println!("{}", x);
    // 1.2 FnMut
    let mut y = 10;
    let mut c2 = || {
        y += 10;
    };
    c2();
    c2();
    println!("{}", y);

    // 1.3 FnOnce
    let z = "rust".to_string();
    let c3 = move || {
        println!("{}", z);
    };
    c3();
    // 无法再次调用
    // c4();
    // 无法再次使用 z
    // println!("{}", z);

    // 2. 检验闭包
    test_closure();
}


fn test_closure() {
    let x = 5;
    let c1 = || {
        println!("{}", x);
    };

    let mut y = 6;
    let mut c2 = || {
        y += 1;
        println!("{}", y);
    };

    let z = "rust".to_string();
    let c3 = move || {
        println!("{}", z);
    };

    is_Fn(&c1);
    is_FnMut(&c1);
    is_FnOnce(&c1);

    is_FnMut(&c2);
    is_FnOnce(&c2);

    is_FnOnce(&c3);
}

fn is_Fn<F>(_: &F) where F: Fn() -> () {}

fn is_FnMut<F>(_: &F) where F: FnMut() -> () {}

fn is_FnOnce<F>(_: &F) where F: FnOnce() -> () {}
