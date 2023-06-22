use std::cmp::Ordering;
use std::ops::Mul;

#[test]
fn test_custom_type() {
    // 定义 Example 类型
    #[derive(Copy, Clone, Debug)]
    struct Example {
        value: i32,
    }

    // 实现 Mul trait
    // 定义操作符右侧类型 i32
    impl Mul<i32> for Example {
        // 定义输出类型 f64
        type Output = f64;

        fn mul(self, rhs: i32) -> f64 {
            (self.value * rhs) as f64
        }
    }

    let example = Example { value: 10 };
    let result = example * 10;
    println!("example * 10 = {}", result);
}

#[test]
fn test_ord() {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct Example {
        value: i32,
    }

    impl PartialOrd for Example {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            if self.value < other.value {
                Some(Ordering::Less)
            } else if self.value > other.value {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        }
    }

    let example1 = Example { value: 10 };
    let example2 = Example { value: 20 };

    println!("example1 > example2 ? {}", example1 > example2);
}
