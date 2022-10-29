fn main() {
    // ****************************一. 向函数传递值*************************************
    // 1. 创建 name，name所有权在 main 函数作用域内
    let name = String::from("ZhangSan");
    // 2.
    // 2.1 将 name 以参数形式传递到 print_name 函数中
    // 2.2 name 离开 main 函数作用域，所有权转移到函数内部
    print_name(name);

    // 5.接下来 将无法再使用 name 变量
    // name 变为无效，下面的代码报错
    // println!("{}", name);

    // ****************************二. 向函数传递值*************************************
    // 3. get_name 返回值的所有权转移至 name
    let name = get_name();
    // 4. 打印name
    println!("{}", name);

    // ****************************三. 控制流程*************************************
    // 【分支】
    let x = 3 % 2;
    let name = String::from("小明");
    if x == 0 {
        print_dance(name);
    } else {
        print_swim(name);
    }
    // 无法再次使用 name
    // print_dance(name);

    // 【循环】
    let name = String::from("rust");
    for _ in 1..4 {
        // 下面的代码 是错误的
        // print_dance(name);
    }

    // 【匹配】
    let x = 3 % 2;
    let name = String::from("小明");
    match x {
        0 => {
            print_dance(name);
        }
        1 => {
            print_swim(name);
        }
        _ => {
            println!("{}", name);
        }
    };
    // 下面的代码 是错误的
    // println!("{}", name);
}

fn print_dance(name: String) {
    println!("{} 喜欢跳舞", name);
}

fn print_swim(name: String) {
    println!("{} 喜欢游泳", name);
}

fn print_name(name: String) {
    // 3. name 进入 print_name 作用域, 外部将无法访问 name 变量
    println!("My name is {}", name);
} // 4. 函数结束，作用域失效，内存释放，name 变量也被释放


fn get_name() -> String {
    // 1. 函数内创建 name 变量
    let name = String::from("LiSi");
    return String::from("My name is ") + name.as_str();
} // 2. 作用域结束，内存释放，name释放，将返回值的所有权转移至调用者
