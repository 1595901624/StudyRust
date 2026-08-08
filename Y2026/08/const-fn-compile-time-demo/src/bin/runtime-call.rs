const fn calculate(x: u64) -> u64 {
    x.wrapping_mul(31).wrapping_add(7)
}

fn runtime_calculate(input: u64) -> u64 {
    calculate(input)
}

fn main() {
    // `black_box` 让优化器无法在编译期知道参数的值
    // let input = std::hint::black_box(10);
    let input = 10;
    println!("{}", runtime_calculate(input));
}
