use std::fmt::{Debug};
use std::ops::{Add, AddAssign, Deref, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Sub, SubAssign};

/// 复数
/// a + bi (i^2 = -1)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Complex<T> {
    // 实部
    a: T,
    // 虚部
    b: T,
}

impl<T> Complex<T> {
    pub fn new(a: T, b: T) -> Complex<T> {
        Self {
            a,
            b,
        }
    }
}

// impl<T> Display for Complex<T> where T: Display + PartialOrd {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}+{}i", self.a, self.b)
//     }
// }

/// 加法
impl<T> Add for Complex<T> where T: Add<Output=T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.a + rhs.a, self.b + rhs.b)
    }
}

/// 减法
impl<T> Sub for Complex<T> where T: Sub<Output=T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Complex::new(self.a - rhs.a, self.b - rhs.b)
    }
}

/// 乘法
impl<T> Mul for Complex<T> where T: Mul<Output=T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex::new(self.a * rhs.a, self.b * rhs.b)
    }
}

/// 除法
impl<T> Div for Complex<T> where T: Div<Output=T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Complex::new(self.a / rhs.a, self.b / rhs.b)
    }
}

/// +=
impl<T> AddAssign for Complex<T> where T: AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.a += rhs.a;
        self.b += rhs.b;
    }
}

/// -=
impl<T> SubAssign for Complex<T> where T: SubAssign {
    fn sub_assign(&mut self, rhs: Self) {
        self.a -= rhs.a;
        self.b -= rhs.b;
    }
}

/// *=
impl<T> MulAssign for Complex<T> where T: MulAssign {
    fn mul_assign(&mut self, rhs: Self) {
        self.a *= rhs.a;
        self.b *= rhs.b;
    }
}

/// /=
impl<T> DivAssign for Complex<T> where T: DivAssign {
    fn div_assign(&mut self, rhs: Self) {
        self.a /= rhs.a;
        self.b /= rhs.b;
    }
}

/// -
impl<T> Neg for Complex<T> where T: Neg<Output=T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Complex::new(-self.a, -self.b)
    }
}

/// 为 Complex 实现 [] 操作符，[0] 访问实部，[1] 访问虚部
/// []
impl<T> Index<usize> for Complex<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.a,
            1 => &self.b,
            _ => panic!("index out of range"),
        }
    }
}

impl<T> IndexMut<usize> for Complex<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.a,
            1 => &mut self.b,
            _ => panic!("index out of range"),
        }
    }
}

impl Deref {
    
}

