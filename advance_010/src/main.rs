use std::ops::Neg;

fn main() {

    // 1. 取反/负号运算符重载
    let a = Complex { a: 1, b: 2 };
    let b = -a;
    println!("{:?}", b)
}


/// 复数
/// 简化复数的定义，实部和虚部都是整数
#[derive(Debug)]
struct Complex {
    // 实部
    a: i32,
    // 虚部
    b: i32,
}

/// 实现取反/负号运算符重载
impl Neg for Complex {
    // 输出结果的类型
    type Output = Complex;

    // 对复数的实部和虚部都取反
    fn neg(self) -> Self::Output {
        Complex {
            a: -self.a,
            b: -self.b,
        }
    }
}