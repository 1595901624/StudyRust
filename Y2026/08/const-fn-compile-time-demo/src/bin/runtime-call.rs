const fn calculate(x: u64) -> u64 {
    x.wrapping_mul(31).wrapping_add(7)
}

fn runtime_calculate(input: u64) -> u64 {
    calculate(input)
}

fn main() {
    // `black_box` 让输入在这里不容易被优化器提前看穿
    let input = std::hint::black_box(10);
    println!("{}", runtime_calculate(input));
}
