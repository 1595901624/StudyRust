fn main() {
    // iter
    let vec = vec!["hello".to_string(), "rust".to_string(), "!".to_string()];
    let mut my_iter = vec.iter();

    println!("{:?}", my_iter.next());
    println!("{:?}", my_iter.next());
    println!("{:?}", my_iter.next());

    // iter_mut
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
    let mut vec = vec!["hello".to_string(), "rust".to_string(), "!".to_string()];
    for item in vec.iter() {
        println!("{:?}", item);
    }
}
