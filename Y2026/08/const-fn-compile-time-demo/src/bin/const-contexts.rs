const fn buffer_size(blocks: usize) -> usize {
    blocks * 1024
}

// `const` 和 `static` 的初始化表达式都是常量上下文
const DEFAULT_SIZE: usize = buffer_size(4);
static LIMIT: usize = buffer_size(8);

fn main() {
    // 数组长度和 `const {}` 块也要求在编译期求值
    let buffer = [0_u8; buffer_size(2)];
    let local_size = const { buffer_size(16) };

    println!("{} {}", buffer.len(), local_size);
}
