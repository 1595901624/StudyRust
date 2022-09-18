fn main() {
    // 1. 变量声明
    // 每次声明变量时，注意变量命名规范
    // 声明整数
    let a = 5;

    // 布尔值
    let b1 = true;

    // 字符串
    let _c = "zhangsan";

    // 浮点数
    let d_1 = 123.3;

    dbg!(a);
    dbg!(b1);
    dbg!(_c);
    dbg!(d_1);

    // 2. 可变变量
    let x = 5;
    // 再次赋值会报错
    // x = 15;

    // 使用mut关键字声明变量
    let mut y = 5;
    // 编译器不会报错
    y = 15;

    // 3. 变量遮蔽
    let m = 1;
    let m = 2.3;
    let m = "张三";
    let mut m = 4;
    let m = 5;
    dbg!(m);

    // 4. 声明常量
    // 命名规则：变量名全部大写
    // 多单词组合使用下划线分割，如：MIN_VALUE
    // 必须指定变量的数据类型
    // const PI = 3.141592653; [X] 错误写法
    const PI: f64 = 3.141592653;
    dbg!(PI);

    // 5. 静态变量
    static IP: &str = "111.111.111.111";
    static mut NAME : &str = "zhangsan";

    dbg!(IP);

    unsafe {
        dbg!(NAME);
    }
}
