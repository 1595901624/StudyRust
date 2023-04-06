fn main() {
    // for 遍历
    let vec = vec![1, 2, 3];
    for i in vec {
        println!("{}", i);
    }

    // 等价简写为下面的代码
    let vec = vec![1, 2, 3];
    let mut iterator = vec.into_iter();
    while let Some(i) = iterator.next() {
        println!("{}", i);
    }

    let example = Example(8);
    for item in example.into_iter() {
        println!("item = {}", item);
    }
    let mut example = Example(8);
    println!("未熔断 {:?}", example.next());
    println!("未熔断 {:?}", example.next());
    println!("未熔断 {:?}", example.next());
    println!("未熔断 {:?}", example.next());
    println!("未熔断 {:?}", example.next());

    println!("---------------------");

    let mut example = Example(8).fuse();
    println!("熔断后 {:?}", example.next());
    println!("熔断后 {:?}", example.next());
    println!("熔断后 {:?}", example.next());
    println!("熔断后 {:?}", example.next());
    println!("熔断后 {:?}", example.next());

}

struct Example(i32);

// 为Example实现一个迭代器
impl Iterator for Example {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // 如果它的值 % 10 != 0 则返回 Some，否则返回None
        return if self.0 % 10 != 0 {
            self.0 += 1;
            Some(self.0)
        } else {
            self.0 += 1;
            None
        };
    }
}
