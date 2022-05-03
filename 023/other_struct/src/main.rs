fn main() {
    //1. 类元组结构体
    // 声明类元组结构体
    struct Point(i32, i32);
    // 创建类元组结构体
    let mut point = Point(1, 1);
    // 修改值
    point.0 = 10;
    // 访问值
    println!("Point{{x = {}, y = {}}}", point.0, point.1);

    //2. 类基元结构体
    // 声明
    struct UnitStruct;
    // 创建
    let us = UnitStruct;


    // 2. 结构体
    struct Salary {
        // 表示月薪
        monthly: Vec<u32>,
        // 表示奖金
        bonus: u32,
    }
    // 我的薪资每个月，10,000元RMB，共12个月
    // 另外我的年终奖是 66,666元
    let mut my_salary = Salary {
        monthly: vec![10_000; 12],
        bonus: 66_666,
    };

    // 总结
    #[derive(Debug)]
    struct ASCII(Vec<u8>);

    let ascii_demo = ASCII(vec![0,0,0,0,0,0,0,1]);
    dbg!(ascii_demo);

}
