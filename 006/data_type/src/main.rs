fn main() {
    // 1. 整型
    // 默认 i32 类型
    let a = 20;
    // 声明无符号整数 类型声明
    let b: u32 = 300;
    // 后缀声明
    let c = 4000u32;

    dbg!(a);
    dbg!(b);
    dbg!(c);

    // 整型溢出
    let mut d: i8 = 127;
    // d = 128;
    dbg!(d);

    // 2. 浮点数
    // 默认类型
    let f1 = 0.0;
    // 类型声明
    let f2: f32 = 54.0;
    // 后缀声明
    let f3 = 99.999f32;
    // 下面是错误范例 —— 整型赋值给浮点数
    // let f4: f32 = 1;

    dbg!(f1);
    dbg!(f2);
    dbg!(f3);


    // 3. 布尔值
    // 默认不声明类型
    let g = true;
    // 声明类型
    let h: bool = false;

    dbg!(g);
    dbg!(h);

    // 4. 字符类型
    // 默认不声明类型
    let i = 'i';
    // 声明类型
    let j: char = 'j';
    // emoji表情
    let k = '😃';
    // 汉字
    let l = '中';
    // 藏文
    let m = 'ག';

    dbg!(i);
    dbg!(j);
    dbg!(k);
    dbg!(l);
    dbg!(m);
}
