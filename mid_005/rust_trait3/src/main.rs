use std::cmp::Ordering;
use std::cmp::Ordering::Less;
use std::ptr::eq;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(PartialOrd)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 三角形
struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

// 自己实现 PartialEq
// 判断三角形相似
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        let x = self.a / other.a;
        let y = self.b / other.b;
        let z = self.c / other.c;
        return x == y && x == z && y == z;
    }

    fn ne(&self, other: &Self) -> bool {
        return !eq(self, other);
    }
}

// 货物
#[derive(PartialEq)]
struct Good {
    price: f64,
}

impl PartialOrd for Good {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // 要求价格低的反而大
        return if self.price < other.price {
            Some(Ordering::Greater)
        } else if self.price > other.price {
            Some(Less)
        } else {
            Some(Ordering::Equal)
        };
    }
}

fn main() {
    // 1、Debug
    let rec1 = Rectangle {
        width: 3,
        height: 5,
    };
    println!("{:?}", rec1);

    let rec2 = Rectangle {
        width: 6,
        height: 4,
    };
    dbg!(rec2);

    // 2.Eq PartialEq

    // 浮点数
    let fa = f64::NAN;
    let fb = f64::NAN;
    println!("fa == fb ? {}", fa == fb);

    let rec1 = Rectangle {
        width: 3,
        height: 5,
    };

    let rec2 = Rectangle {
        width: 3,
        height: 5,
    };

    println!("rec1 == rec2 ? {}", rec1 == rec2);

    // 相似三角形
    let tri1 = Triangle { a: 3.0, b: 4.0, c: 5.0 };
    let tri2 = Triangle { a: 6.0, b: 8.0, c: 10.0 };
    println!("tri1 和 tri2 相似 ? {}", tri1 == tri2);

    // 3. Ord 和 PartialOrd
    let good1 = Good {
        price: 1.0
    };

    let good2 = Good {
        price: 2.0
    };
    println!("good1 > good2 ? {}", good1 > good2);
}
