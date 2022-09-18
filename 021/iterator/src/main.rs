fn main() {
    println!("\n*****************1、迭代器的定义*********************");
    let vec = vec![3, 4, 5];
    let mut iter = vec.iter();

    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());
    dbg!(iter.next());

    println!("\n*****************2、迭代器的使用*********************");
    let vec_for = vec![1, 2, 3, 4, 5];
    for i in vec_for.iter() {
        print!("{} ", i);
    }

    println!("\n*****************3、迭代器与消费器*********************");
    let vec_consumer = vec![2, 4, 6, 8, 10];
    // 1. sum
    let sum_result: i32 = vec_consumer.iter().sum();
    dbg!(sum_result);

    // 2. any
    let any_result = vec_consumer.iter().any(|x| *x % 2 != 0);
    dbg!(any_result);

    //3. collect or map
    let collect_result: Vec<i32> = vec_consumer.iter().map(|x| x - 1).collect();
    dbg!(collect_result);

    println!("\n*****************4、迭代器与适配器*********************");
    let vec_adapter = vec![1, 3, 5, 7, 9];
    // 1. take
    let take_result = vec_adapter.iter().take(3);
    dbg!(take_result);
    // 2. filter
    let filter_result: Vec<i32> = vec_adapter.iter().map(|x| *x + 2).filter(|x| *x % 3 == 0).collect();
    dbg!(filter_result);

    //3. rev
    let rev_result = vec_adapter.iter().rev();
    dbg!(&rev_result);
    for i in rev_result {
        print!("{} ", i);
    }

    //4. zip
    let vec1 = vec![3, 5, 7];
    let vec2 = vec![2, 4, 6];
    let vec_zip: Vec<i32> = vec1.iter().zip(vec2.iter()).map(|x| { x.0 + x.1 }).collect();
    dbg!(vec_zip);
}
