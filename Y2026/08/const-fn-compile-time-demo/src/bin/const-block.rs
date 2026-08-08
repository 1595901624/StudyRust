const fn calculate(x: u64) -> u64 {
    x.wrapping_mul(31).wrapping_add(7)
}

fn main() {
    // `const {}` 明确要求这一段在编译期完成求值
    let input = const { calculate(10) };

    println!("{}", input);
}
