fn main() {
    // 1. 模式匹配中的下划线
    let a = 6;
    match a % 2 {
        1 => {
            println!("a % 2 == 1");
        }
        _ => {
            println!("a % 2 == 0");
        }
    }
}
