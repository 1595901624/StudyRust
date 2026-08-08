const fn square(x: u32) -> u32 {
    x * x
}

// `const` 的初始化表达式属于常量上下文
const AREA: u32 = square(12); // 一定在编译期求值

fn main() {
    let side = 12;
    // 普通函数中的调用不要求在编译期完成
    let area = square(side);
    println!("{AREA}, {area}");
}
