use std::ops::{Mul, Neg, SubAssign};
use crate::complex::Complex;

mod complex;
mod test;

fn main() {}

#[test]
fn calc1() {
    let c = Complex::new(1.0, 3.0);
    // -c 等价于 c.neg()
    println!("c = {:?}", -c);
    println!("c = {:?}", c.neg());
}

/// 四则运算
#[test]
fn calc2() {
    let c1 = Complex::new(1.0, 3.0);
    let c2 = Complex::new(2.0, 4.0);

    // + - * /
    let sum = c1 + c2;
    let sub = c1 - c2;
    let mul = c1 * c2;
    let div = c1 / c2;

    println!("c1 + c2 = {:?}", sum);
    println!("c1 - c2 = {:?}", sub);

    // c1 * c2 等价与 c1.mul(c2)
    println!("c1 * c2 = {:?}", mul);
    println!("c1 * c2 = {:?}", c1.mul(c2));

    println!("c1 / c2 = {:?}", div);
}

/// 自增运算
#[test]
fn calc3() {
    let c1 = Complex::new(1.0, 3.0);
    let c2 = Complex::new(2.0, 4.0);

    // +=
    let mut c = c1;
    c += c2;
    println!("add_assign c = {:?}", c);

    // -=
    let mut c = c1;
    // 等价于 c.sub_assign(c2)
    c -= c2;
    println!("sub_assign c = {:?}", c);

    // *=
    let mut c = c1;
    c *= c2;
    println!("mul_assign c = {:?}", c);

    // /=
    let mut c = c1;
    c /= c2;
    println!("div_assign c = {:?}", c);
}

#[test]
fn test_eq() {
    let c1 = Complex::new(1.0, 3.0);
    let c2 = Complex::new(2.0, 4.0);
    let c3 = Complex::new(1.0, 3.0);

    assert_eq!(c1, c3);
    assert_ne!(c1, c2);
}

#[test]
fn test() {
    let c = Complex::new(1, 3);
    println!("a = {:?}", !c);
}