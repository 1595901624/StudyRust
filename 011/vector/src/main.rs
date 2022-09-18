// 动态数组打印时都添加了 “&” 符号，这里与Rust的所有权有关，这里了解即可，后面会详细介绍
fn main() {
    println!("\n**************1、创建空的动态数组********************");
    let vec_empty: Vec<i32> = Vec::new();
    dbg!(&vec_empty);

    println!("\n**************2、初始化时为动态数组指定容量********************");
    let vec_capacity: Vec<i32> = Vec::with_capacity(5);
    dbg!(&vec_capacity);

    println!("\n**************3、使用“宏”创建动态数组********************");
    let vec_marco: Vec<i32> = vec![];
    dbg!(&vec_marco);
    let vec_marco = vec![1, 2, 3, 4, 5];
    dbg!(&vec_marco);
    // 长度为5，元素初始化为0
    let vec_marco = vec![0; 5];
    dbg!(&vec_marco);

    println!("\n**************4、追加元素********************");
    let mut vec_push = vec![0; 5];
    vec_push.push(1);
    dbg!(&vec_push);

    println!("\n**************5、修改元素********************");
    vec_push[3] = 1;
    dbg!(&vec_push);

    println!("\n**************6、删除元素(pop)********************");
    let mut vec_pop = vec![1];
    let pop = vec_pop.pop();
    dbg!(pop);
    let pop = vec_pop.pop();
    dbg!(pop);

    println!("\n**************7、删除元素(remove)********************");
    let mut vec_remove = vec!['w', 'o', 'r', 'l', 'd'];
    let remove_element = vec_remove.remove(3);
    dbg!(remove_element);
    // 索引越界，会发生错误
    // let remove_element = vec_remove.remove(5);

    println!("\n**************8、访问元素(n[i])********************");
    let vec_find = vec![1, 2, 3];
    dbg!(vec_find[0]);
    dbg!(vec_find[1]);
    dbg!(vec_find[2]);
    // 下面代码产生越界
    // dbg!(vec_find[3]);

    println!("\n**************9、访问元素(get)********************");
    let vec_get = vec![1, 2, 3];
    dbg!(vec_get.get(0));
    dbg!(vec_get.get(1));
    dbg!(vec_get.get(2));
    // 下面代码不会产生错误，正常执行
    dbg!(vec_get.get(3));


    println!("\n**************10、动态数组容量的变化********************");
    let mut vec_capacity: Vec<i32> = Vec::with_capacity(5);
    // 动态增长
    println!("vec_capacity 填充元素前的长度为 {}", vec_capacity.len());
    println!("vec_capacity 填充元素前的容量为 {}", vec_capacity.capacity());
    // 关于循环这里了解即可，这里的意思是，填充0,1,2,3,4五个元素

    vec_capacity.push(0);
    vec_capacity.push(1);
    vec_capacity.push(2);
    vec_capacity.push(3);
    vec_capacity.push(4);

    println!("vec_capacity 填充5个元素后的长度为 {}", vec_capacity.len());
    println!("vec_capacity 填充5个元素后的容量为 {}", vec_capacity.capacity());

    // 填充第6个元素
    vec_capacity.push(5);

    println!("vec_capacity 填充6个元素后的长度为 {}", vec_capacity.len());
    println!("vec_capacity 填充6个元素后的容量为 {}", vec_capacity.capacity());

    println!("\n**************11、默认动态数组的容量********************");
    let mut vec_default: Vec<i32> = Vec::new();
    println!("vec_default 的长度为 {}", vec_default.len());
    println!("vec_default 的容量为 {}", vec_default.capacity());
    vec_default.push(1);
    println!("vec_default 添加一个元素的长度为 {}", vec_default.len());
    println!("vec_default 添加一个元素的容量为 {}", vec_default.capacity());

}
