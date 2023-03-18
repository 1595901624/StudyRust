fn main() {
    // 1. 占位符
    // 1.1 match 模式匹配中的下划线
    let a = 6;
    match a % 2 {
        1 => {
            println!("a % 2 == 1");
        }
        _ => {
            println!("a % 2 == 0");
        }
    }
    // 1.2 解构赋值
    let (m, _, n) = (4, 5, 6);
    println!("m = {}, n = {}", m, n);

    // 2. let 中的下划线
    let name = String::from("rust");
    // 下面一行代码直接忽略掉 replace 之后的值
    let _ = name.replace("s", "x");
    println!("name = {} ", name);

    // 忽略 #[must_use]警告
    // let mut input = String::new();
    // let _ = std::io::stdin().read_line(&mut input);

    // 3. drop
    // 3.1  在 let 绑定后，匹配为 `_` 的值不会发生所有权转移
    let rust = String::from("hello");
    let _ = rust; // 不发生所有权转移，不会被drop
    // rust; // 所有权转移，会被drop
    println!("rust = {}", rust);

    // 3.2 在 `match` 、 `if let` 、 `while let` 和 `for` 块中，匹配为 `_` 的值不会发生所有权转移
    let q = (String::from("x"), String::from("y"), String::from("z"));
    match q {
        (x, y, _) => {
            // q.0 和 q.1 的所有权转移
            println!("m = {}, n = {}", x, y);
        }
    }
    // println!("q = {:?}", q); q中存在被move的值，无法打印
    // println!("q.0 = {:?}", q.0); q.0 被move，无法打印
    // println!("q.1 = {:?}", q.1); q.0 被move，无法打印
    // 打印q.2
    println!("q.2 = {:?}", q.2);

    // 3.3 在函数、方法、闭包中，匹配为 `_` 的值作用域结束后
    // 函数
    let u = String::from("hello");
    let v = String::from("rust");
    function(u, v);
    // 下面一行代码报错
    // println!("v = {}", v);

    // 闭包
    let u = String::from("hello");
    let v = String::from("rust");
    let closure = |u, _| u; // u、v 被 drop掉
    closure(u, v);
    // 下面一行代码报错
    // println!("v = {}", v);

    // 4. 省略类型声明
    let vec: Vec<_> = vec![1, 2, 3];
    println!("{:?}", vec);

    // 6. 未命名常量
    const _: i32 = 1;
}

fn function(u: String, _: String) {
    println!("u = {}", u);
} // u、v 被 drop掉

// 5. 匿名生命期
struct Dog<'a> {
    name: &'a str,
}

impl Dog<'_> {
    fn new(name: &str) -> Dog {
        return Dog { name };
    }

    fn get_name(&self) -> &str {
        return self.name;
    }
}

// 不使用匿名生命期
// impl<'a> Dog<'a> {
//     fn new(name: &str) -> Dog {
//         return Dog { name };
//     }
//
//     fn get_name(&self) -> &str {
//         return self.name;
//     }
// }


