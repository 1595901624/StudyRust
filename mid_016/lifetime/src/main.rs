fn main() {
    // 以下代码是错误的
    // let a;
    // {
    //     let b = 1;
    //     a = &b;
    // }
    // println!("{}", *a);
    // 以上代码是错误的

    let b = 1;

    {
        let a = &b;
        println!("{}", *a);
    }


    // 函数签名中的生命期标注
    let name: &str = "zhangsan";
    test_life(name);

    let x = String::from("xxx");
    let y = "yyyy";

    let z = longest(x.as_str(), y);

    println!("{}", z);

    // 引用作为函数/方法返回值
    // concat_str(x, y);
    get_any_str();
}

fn get_any_str() -> &'static str {
    return "static";
}

// 以下代码是错误的
/// 拼接两个字符串
// fn concat_str<'a>(x: &'a str, y: &'a str) -> &'a str {
//     let s = format!("{}{}",x, y);
//     return s.as_str();
// }
// 以上代码是错误的

// 以下代码是错误的
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// 以上代码是错误的


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn test(name: &str) -> &str {
    println!("{}", name);
    return name;
}

fn test_life<'_a>(name: &'_a str) -> &'_a str {
    println!("{}", name);
    return name;
}



