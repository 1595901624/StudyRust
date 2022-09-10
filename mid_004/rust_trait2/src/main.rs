// 定义一个泛型trait
trait MyPrint<T> {
    // 输出传递的参数
    fn print(&self, x: T) -> T;
}

// 测试结构体
struct Test;

// 为Test实现MyPrint
impl MyPrint<i32> for Test {
    // 返回值
    fn print(&self, x: i32) -> i32 {
        return x;
    }
}


fn main() {
    let test = Test;
    // 直接输出结果
    println!("{}", test.print(3));

    // ********************以下示例用于复杂的开发场景**************************
    // 1. 一个泛型，多个约束
    println!("**************1. 一个泛型，多个约束**************");
    let trait_all = TestStructAll;
    trait_demo(trait_all);

    // 2. 多个泛型，多个约束
    println!("\n**************2. 多个泛型，多个约束**************");
    let trait_one = TestStructOne;
    let trait_two_and_other = TestStructTwoAndOther;
    multi_fun(trait_one, trait_two_and_other);

    // 3. where 关键字优化
    println!("\n**************3. where 关键字优化**************");
    let trait_one_where = TestStructOne;
    let trait_two_and_other_where = TestStructTwoAndOther;
    multi_fun_where(trait_one_where, trait_two_and_other_where);
    // ********************以上示例用于复杂的开发场景**************************
}

// Supertraits
trait Animal {
    fn speak(&self);
}

trait Dog: Animal {
    // 狗还会跳
    fn jump(&self);
}

struct SmallDog;

// 为 SmallDog 实现 Dog 的同时，也必须实现 Animal
impl Animal for SmallDog {
    fn speak(&self) {

    }
}

impl Dog for SmallDog {
    fn jump(&self) {
    }
}

// ********************以下示例用于复杂的开发场景**************************
// 声明多个 trait，trait 约束
trait TraitOne {
    fn print_trait_one(&self);
}

trait TraitTwo {
    fn print_trait_two(&self);
}

trait TraitOther {
    fn print_trait_other(&self);
}

// 测试结构体
struct TestStructOne;

struct TestStructTwoAndOther;

struct TestStructAll;

impl TraitOne for TestStructOne {
    fn print_trait_one(&self) {
        println!("trait one");
    }
}

impl TraitTwo for TestStructTwoAndOther {
    fn print_trait_two(&self) {
        println!("trait two");
    }
}

impl TraitOther for TestStructTwoAndOther {
    fn print_trait_other(&self) {
        println!("trait other");
    }
}

// *******************以下为 TestStructAll 同时实现了3个TraitOne、TraitTwo、TraitOther*******************
impl TraitOne for TestStructAll {
    fn print_trait_one(&self) {
        println!("trait one");
    }
}

impl TraitTwo for TestStructAll {
    fn print_trait_two(&self) {
        println!("trait two");
    }
}

impl TraitOther for TestStructAll {
    fn print_trait_other(&self) {
        println!("trait other");
    }
}
// *******************以上为 TestStructAll 同时实现了3个TraitOne、TraitTwo、TraitOther*******************


// 调用trait的方法
// 一个泛型，多个约束
fn trait_demo<T: TraitOne + TraitTwo + TraitOther>(param: T) {
    param.print_trait_one();
    param.print_trait_two();
    param.print_trait_other();
}

// 多个泛型，多个约束
fn multi_fun<T: TraitOne, E: TraitTwo + TraitOther>(param1: T, param2: E) {
    // param1 有一个约束
    param1.print_trait_one();
    // param2 有两个约束
    param2.print_trait_two();
    param2.print_trait_other();
}

// where 优化
fn multi_fun_where<T, E>(param1: T, param2: E) where T: TraitOne, E: TraitTwo + TraitOther {
    // param1 有一个约束
    param1.print_trait_one();
    // param2 有两个约束
    param2.print_trait_two();
    param2.print_trait_other();
}

// ********************以上示例用于复杂的开发场景**************************