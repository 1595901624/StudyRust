const fn percentage(total: u32, part: u32) -> u32 {
    assert!(total != 0, "total 不能为 0");
    part * 100 / total
}

// 常量上下文中的这次调用会在编译期执行
const SUCCESS: u32 = percentage(10, 3);

fn main() {
    println!("{SUCCESS}");
}
