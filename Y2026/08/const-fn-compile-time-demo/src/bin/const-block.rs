const fn build_mask(bits: u32) -> u32 {
    (1_u32 << bits) - 1
}

fn main() {
    // `const {}` 明确要求这一段在编译期完成求值
    let mask = const { build_mask(8) };
    println!("{mask:#010b}");
}
