use std::mem;
use std::mem::size_of_val;

#[derive(Debug)]
enum Week {
    Mon,
    Tue,
    Wed,
    Thu = 300,
    Fri,
    Sat,
    Sun,
}

#[derive(Debug)]
enum Color {
    // 定义色值，参数分别表示16进制颜色代码，R，G，B
    White(String, u8, u8, u8),
    Red,
    Black { code: String, r: u8, g: u8, b: u8 },
}

fn main() {
    // 1. C式枚举
    // 打印枚举类型
    println!("{:?}", Week::Wed);
    // 打印枚举的值
    println!("{}", Week::Wed as i32);

    println!("{}", Week::Mon as i32);

    // 由于Thu 赋值维 300, 则后面的值依次+1
    println!("{}", Week::Fri as i32);

    // 输出结果:
    // Wed
    // 2
    // 0
    // 301

    // 2.包含数据的枚举
    // ("#FFFFFF", 255, 255, 255)
    let white = Color::White(String::from("#FFFFFF"), 255, 255, 255);
    let red = Color::Red;
    let black = Color::Black { code: String::from("#000000"), r: 8, g: 6, b: 4 };
    println!("{:?}", white);
    println!("{:?}", red);
    println!("{:?}", black);

    // 输出结果:
    // White("#FFFFFF", 255, 255, 255)
    // Red
    // Black { code: "#000000", r: 0, g: 0, b: 0 }

    unsafe {
        let mem = mem::size_of::<Color>();
        let align = mem::align_of::<Color>();
        println!("内存：{}， 对齐：{}", mem, align);
    }
}

