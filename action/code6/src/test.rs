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
