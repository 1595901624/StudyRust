use std::ops::Not;

/// 自定义布尔类型
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MyBool {
    True,
    False,
}

impl Not for MyBool {
    type Output = MyBool;

    fn not(self) -> Self::Output {
        match self {
            MyBool::True => MyBool::False,
            MyBool::False => MyBool::True,
        }
    }
}
