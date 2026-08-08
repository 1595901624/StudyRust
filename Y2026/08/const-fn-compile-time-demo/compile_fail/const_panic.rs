const fn percentage(total: u32, part: u32) -> u32 {
    assert!(total != 0, "total 不能为 0");
    part * 100 / total
}

// 改成零后，编译器会在常量求值期间触发 panic
const FAILURE: u32 = percentage(0, 3);

fn main() {
    println!("{FAILURE}");
}
