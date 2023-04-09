fn main() {
    // 1. iter
    let vec = vec!["hello".to_string(), "rust".to_string(), "!".to_string()];
    let mut my_iter = vec.iter();

    println!("{:?}", my_iter.next());
    println!("{:?}", my_iter.next());
    println!("{:?}", my_iter.next());

    // 2. iter_mut
    let mut vec = vec!["hello".to_string(), "rust".to_string(), "!".to_string()];
    let mut my_iter_mut = vec.iter_mut();

    // 修改第一项的值
    if let Some(first) = my_iter_mut.next() {
        first.push_str("!");
    }
    println!("{:?}", my_iter_mut.next());
    println!("{:?}", my_iter_mut.next());

    println!("{:?}", vec[0]);

    // for + iterator
    let vec = vec!["hello".to_string(), "rust".to_string(), "!".to_string()];
    for item in vec.iter() {
        println!("{:?}", item);
    }

    // 3. into_iterator
    let vec = vec!["hello".to_string(), "rust".to_string(), "!".to_string()];
    let mut iter = vec.into_iter();
    // 集合里的第1个元素所有权转移至 first
    let first = iter.next();
    // 集合里的第2个元素所有权转移至 second
    let second = iter.next();
    // 集合里的第3个元素所有权转移至 second
    let third = iter.next();

    println!("{:?}", first);
    println!("{:?}", second);
    println!("{:?}", third);

    // 至此 vec 已变成空壳
    // vec 无法再使用，所有权已经转移
    // 下面的代码将会编译失败
    // println!("{:?}", vec);

    println!("----------------------");
    // 4. 集合引用的 into_iter
    // 4.1 共享引用
    let vec: Vec<String> = vec!["hello".to_string(), "rust".to_string(), "!".to_string()];
    let mut iter = (&vec).into_iter();
    println!("{:?}", iter.next());
    // 下面的代码并不会编译失败
    println!("{:?}", vec);
    // 4.2 可修改引用
    let mut vec: Vec<String> = vec!["hello".to_string(), "rust".to_string(), "!".to_string()];
    let mut iter = (&mut vec).into_iter();
    iter.next().unwrap().push_str("hello");
    // 下面的代码并不会编译失败
    println!("{:?}", vec);


}
