mod graph_trait;
mod rectangle;
mod circle;

use crate::graph_trait::Graph;
use crate::rectangle::Rectangle;

/// 动物 trait
trait Animal {
    // 动物的叫声
    fn make_sound(&self);
}

/// 狗
struct Dog {
    name: String,
}

/// 鱼
struct Cat {
    name: String,
}

/// 为 Dog 类型实现 Animal trait
impl Animal for Dog {
    // 打印狗的叫声
    fn make_sound(&self) {
        println!("汪汪~");
    }
}

/// 为 Cat 类型实现 Animal trait
impl Animal for Cat {
    // 打印猫的叫声
    fn make_sound(&self) {
        println!("喵喵~");
    }
}

/// 为已有类型实现 trait
impl Animal for i32 {
    fn make_sound(&self) {
        println!("i32");
    }
}


fn main() {
    // 创建dog
    let dog = Dog { name: String::from("二哈") };
    // 创建cat
    let cat = Cat { name: String::from("美短") };

    // 执行方法
    dog.make_sound();
    cat.make_sound();

    // 执行结果:
    // 汪汪~
    // 喵喵~


    // **********************扩展 START************************
    // 必须声明 use crate::graph_trait::Graph;
    let rec = Rectangle {
        width: 6.0,
        height: 2.0,
    };
    println!("rec 的面积为：{}", rec.area());
    // **********************扩展 END************************


    // 0x03.trait 作为参数
    // speak(dog);
    // speak(cat);

    let a = 5;
    speak(a);

    // 多 trait同时实现
    printMulti(dog);
    // 下面的代码错误
    // printMulti(cat);
}

/// impl trait 作为参数
fn speak(animal: impl Animal) {
    animal.make_sound()
}

/// 测试多trait
trait Test {
    fn test(&self);
}

/// 为Dog实现Test
impl Test for Dog {
    fn test(&self) {
        println!("这是一个Test方法");
    }
}

/// 打印同时实现 Test 和 Animal Trait的方法
fn printMulti(p: impl Test + Animal) {
    p.make_sound();
    p.test();
}
