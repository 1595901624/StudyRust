/// 声明一个图形 shape trait
trait Shape {
    /// 计算面积
    fn area(&self) -> u32;

    /// 计算周长
    fn perimeter(&self) -> u32;
}

/// 矩形结构体
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        return Rectangle { width, height };
    }
}

/// 为 Rectangle 实现 Shape
impl Shape for Rectangle {
    fn area(&self) -> u32 {
        return self.height * self.width;
    }

    fn perimeter(&self) -> u32 {
        return (self.width + self.height) * 2;
    }
}

fn main() {
    let rectangle = Rectangle::new(3, 4);

    let shape: &dyn Shape = &rectangle;

    println!("shape => {}", shape.area());

    println!("rectangle address :{:p}", &rectangle);
    println!("shape address :{:p}", &shape);

    println!("end");
}
