const fn percentage(total: u32, part: u32) -> u32 {
    assert!(total != 0, "total 不能为 0");
    part * 100 / total
}

fn main() {
    // 参数在运行时才得到，panic 也会留到运行时发生
    let total = std::hint::black_box(0);
    let value = percentage(total, 3);
    println!("{value}");
}
