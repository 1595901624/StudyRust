use std::f64::consts::PI;

/// 声明一个图形 shape trait
trait Shape {
    /// 计算面积
    fn area(&self) -> f64;

    /// 计算周长
    fn perimeter(&self) -> f64;
}

/// 矩形结构体
struct Rectangle {
    width: f64,
    height: f64,
}

/// 圆形结构体
struct Circle {
    radius: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Self {
        return Rectangle { width, height };
    }
}

impl Circle {
    fn new(radius: f64) -> Self {
        return Circle { radius };
    }
}

/// 为 Rectangle 实现 Shape
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        return self.height * self.width;
    }

    fn perimeter(&self) -> f64 {
        return (self.width + self.height) * 2.0;
    }
}

/// 为 Circle 实现 Shape
impl Shape for Circle {
    fn area(&self) -> f64 {
        return PI * self.radius * self.radius;
    }

    fn perimeter(&self) -> f64 {
        return 2.0 * PI * self.radius;
    }
}

fn main() {
    // 1. trait object
    let circle = Circle::new(10.0);
    let rectangle = Rectangle::new(3.0, 4.0);

    let shape: &dyn Shape = &rectangle;
    let rec_ref: &Rectangle = &rectangle;

    // println!("[circle] area = {}, perimeter = {}", circle.area(), circle.perimeter());
    // println!("[rectangle] area = {}, perimeter = {}", rectangle.area(), rectangle.perimeter());
    println!("[shape] area = {}, perimeter = {}", shape.area(), shape.perimeter());

    println!("[rec_ref] area = {}, perimeter = {}", rec_ref.area(), rec_ref.perimeter());

    // 2.impl trait 和 trait object 的区别

    // 以下代码是错误的
    // let shape: impl Shape = circle;
    let circle_impl = create_circle_impl_trait();
    println!("[circle_impl] area = {}, perimeter = {}", circle_impl.area(), circle_impl.perimeter());
    print_shape(circle_impl);
}

fn create_circle_impl_trait() -> impl Shape {
    let circle = Circle::new(5.0);
    return circle;
}

fn print_shape(shape: impl Shape) {
    println!("[print_shape] area = {}, perimeter = {}", shape.area(), shape.perimeter());
}