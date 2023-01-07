fn main() {
    // let name = "zhangsan";
    // const age: i32 = 30;
    //
    // let s = Student { name: name, age: &age };
    // println!("{:?}", s);

    let name = "zhangsan";
    let age: i32 = 18;

    let s = Student { name: name, age: &age };
    println!("{:?}", s);

    // 省略规则(1)
    // 如果一个函数的返回值不返回任何引用，那么永远都不需要标注参数的生命期。
    let val1 = 5;
    let val2 = 6;
    let x = fun1(&val1, &val2);
    println!("fun1 = {}", x);

    // 省略规则(2)
    // 如果一个函数的参数只出现了一个生命期且返回值是引用，那么Rust则推断返回值的生命期与函数参数的生命期相同，也不需要标注生命期。
    let array = [6, 3];
    let x = fun2(&array);
    println!("fun2 = {:?}", x);

    let a = String::from("aaa");
    let b = String::from("bbb");
    let c = String::from("ccc");
    let example = Example { data: vec![a, b, c] };
    let x = example.get_element("b");
    println!("fun3 = {:?}", x);
}

#[derive(Debug)]
struct Student<'a> {
    name: &'a str,
    age: &'a i32,
}

// 以下代码不推荐
// #[derive(Debug)]
// struct Student {
//     name: &'static str,
//     age: &'static i32,
// }

// 省略规则(1)
// 如果一个函数的返回值不返回任何引用，那么永远都不需要标注参数的生命期。
fn fun1(a: &i32, b: &i32) -> i32 {
    return a + b + 5;
}

// 省略规则(2)
// 如果一个函数的参数只出现了一个生命期且返回值是引用，那么Rust则推断返回值的生命期与函数参数的生命期相同，也不需要标注生命期。
fn fun2(a: &[i32; 2]) -> (&i32, &i32) {
    return (&a[0], &a[1]);
}

struct Example {
    data: Vec<String>,
}

impl Example {
    /// 通过前缀查找字符串
    fn get_element(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.data.len() {
            if self.data[i].starts_with(prefix) {
                return Some(&self.data[i]);
            }
        }
        None
    }
}