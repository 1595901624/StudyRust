mod test;

use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

fn main() {

    // 1. 取反/负号运算符重载
    let a = Complex { a: 1, b: 2 };
    // 等同于 let b = a.neg();
    let b = -a;
    println!("{:?}", b);

    // 2. 取非
    let my_bool = test::MyBool::True;
    // 等同于 let my_bool_not = my_bool.not();
    let my_bool_not = !my_bool;
    println!("{:?}", !my_bool_not);

    // 3. 加法运算符重载
    let c1 = Complex::new(1, 3);
    let c2 = Complex::new(2, 4);
    // 等同于 let sum = c1.add(c2);
    let sum = c1 + c2;
    println!("c1 + c2 = {:?}", sum);

    // 4. 减法运算符重载
    let c1 = Complex::new(1, 3);
    let c2 = Complex::new(2, 4);
    // 等同于 let sub = c1.sub(c2);
    let sub = c1 - c2;
    println!("c1 - c2 = {:?}", sub);

    // 5. 乘法运算符重载
    let c1 = Complex::new(1, 3);
    let c2 = Complex::new(2, 4);
    // 等同于 let mul = c1.mul(c2);
    let mul = c1 * c2;
    println!("c1 * c2 = {:?}", mul);

    // 6. 除法运算符重载
    let c1 = Complex::new(1, 3);
    let c2 = Complex::new(2, 4);
    // 等同于 let div = c1.div(c2);
    let div = c1 / c2;
    println!("c1 / c2 = {:?}", div);

    // 7. 索引运算符重载
    let mut c = Complex::new(1, 3);
    // 等同于 let a = c.index(0);
    let a = c[0];
    let b = c[1];
    println!("a = {}, b = {}", a, b);
    // 等同于 c.index_mut(0) = 2;
    c[0] = 2;
    c[1] = 4;
    println!("c = {:?}", c);
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

impl Complex {
    fn new(a: i32, b: i32) -> Complex {
        Complex {
            a,
            b,
        }
    }
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

/// 加法运算符重载
impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

/// 减法运算符重载
impl Sub for Complex {
    type Output = Complex;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
        }
    }
}

/// 乘法运算符重载
impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        // (ac－bd)+(bc+ad)i
        Complex {
            a: self.a * rhs.a - self.b * rhs.b,
            b: self.b * rhs.a + self.a * rhs.b,
        }
    }
}

/// 除法运算符重载
impl Div for Complex {
    type Output = Complex;

    fn div(self, rhs: Self) -> Self::Output {
        // (ac+bd)/(c^2 + d^2) + (bc-ad)/(c^2 + d^2) * i
        Complex {
            a: (self.a * rhs.a + self.b * rhs.b) / (rhs.a * rhs.a + rhs.b * rhs.b),
            b: (self.b * rhs.a - self.a * rhs.b) / (rhs.a * rhs.a + rhs.b * rhs.b),
        }
    }
}

impl Index<usize> for Complex {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.a,
            1 => &self.b,
            _ => panic!("index out of range"),
        }
    }
}

impl IndexMut<usize> for Complex {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.a,
            1 => &mut self.b,
            _ => panic!("index out of range"),
        }
    }
}



