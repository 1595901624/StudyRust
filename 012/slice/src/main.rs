fn main() {
    println!("\n*******************1、切片的定义************************");
    let vec = vec![1, 3, 5, 7, 9];
    let array = [0, 2, 4, 6, 8];

    let vec_slice: &[i32] = &vec;
    let array_slice: &[i32] = &array;

    dbg!(vec_slice);
    dbg!(array_slice);

    println!("\n*******************2、切片在内存中的定义************************");
    // let vec_slice = &vec;
    // let array_slice = &array;

    println!("\n*******************3、在切片中使用范围************************");
    let vec = vec![1, 3, 5, 7, 9];
    let array = [0, 2, 4, 6, 8];

    let vec1 = &vec[1..3];
    let vec2 = &vec[..2];
    let vec3 = &vec[3..];
    let vec4 = &vec[..];

    println!("vec1 => vec中下标1到下标3的 元素 {:#?}", vec1);
    println!("vec2 => vec中下标0到下标2的 元素 {:#?}", vec2);
    println!("vec3 => vec中下标3到结束的 元素 {:#?}", vec3);
    println!("vec4 => vec中下标0到结束的 元素 {:#?}", vec4);
    // 相同
    assert_eq!(&vec[..], &vec);

    let array1 = &array[1..3];
    let array2 = &array[..2];
    let array3 = &array[3..];
    let array4 = &array[..];

    println!("array1 => array中下标1到下标3的 元素 {:#?}", array1);
    println!("array2 => array中下标0到下标2的 元素 {:#?}", array2);
    println!("array3 => array中下标3到结束的 元素 {:#?}", array3);
    println!("array4 => array中下标3到结束的 元素 {:#?}", array4);
    // 相同
    assert_eq!(&array[..], &array);

    println!("\n*******************4、访问切片中的元素************************");
    let vec = [1, 3, 5, 7, 9];
    let vec_s = &vec[1..4];
    dbg!(vec_s[2]);

    println!("\n*******************5、修改切片中的元素************************");
    // 必须mut修饰
    let mut vec = [1, 3, 5, 7, 9];
    // 声明为可修改的切片
    let vec_m = &mut vec[1..4];
    vec_m[2] = 10;

    dbg!(vec_m);
    dbg!(vec);

    println!("\n*******************6、切片的常用方法************************");
    let vec = [1, 3, 5, 7, 9];
    let vec_s = &vec[0..0];

    println!("切片 vec_s 的长度是{} ", vec_s.len());
    println!("切片 vec_s 是空吗？{} ", vec_s.is_empty());
}

