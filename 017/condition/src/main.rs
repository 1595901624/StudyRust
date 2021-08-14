fn main() {
    println!("\n*****************1、条件判断************************");
    // 求一个数对应的星期。将一个数对7取余，如果是0，则选择的是周日，以此类推，如果是6则选择的是周6
    let week: u32 = 0;

    if week % 7 == 0 {
        println!("您选择的是周日！")
    } else if week % 7 == 1 {
        println!("您选择的是周一！")
    } else if week % 7 == 2 {
        println!("您选择的是周二！")
    } else if week % 7 == 3 {
        println!("您选择的是周三！")
    } else if week % 7 == 4 {
        println!("您选择的是周四！")
    } else if week % 7 == 5 {
        println!("您选择的是周五！")
    } else {
        println!("您选择的是周六！")
    }

    println!("\n*****************2、循环(loop)************************");
    let mut count = 0;

    loop {
        count += 1;

        // 计数器为5则跳出循环
        if count == 5 {
            break;
        }
    }

    println!("loop循环 -> count = {}", count);

    println!("\n*****************2、循环(while)************************");
    let mut count = 0;

    while count < 5 {
        count += 1;
    }

    println!("while循环 -> count = {}", count);

    println!("\n*****************2、循环(for..in..)************************");
    let mut count = 0;

    for i in 1..=5 {
        count = i
    }

    println!("for..in..循环 -> count = {}", count);

    println!("\n*****************2、循环(continue)************************");
    let mut count = 1;
    while count < 5 {
        if count % 2 == 0 {
            count += 5;
            continue;
        }
        count += 1;
    }

    println!("continue test -> count = {}", count);

    println!("\n*****************3、模式匹配************************");
    // 求一个数对应的星期。将一个数对7取余，如果是0，则选择的是周日，以此类推，如果是6则选择的是周6
    let week: u32 = 6;
    match week % 7 {
        0 => { println!("您选择的是周日！") }
        1 => { println!("您选择的是周一！") }
        2 => { println!("您选择的是周二！") }
        3 => { println!("您选择的是周三！") }
        4 => { println!("您选择的是周四！") }
        5 => { println!("您选择的是周五！") }
        6 => { println!("您选择的是周六！") }
        _ => { println!("未知！") }
    }

    println!("\n*****************4、if let************************");
    // 求一个数对应的星期。将一个数对7取余，如果是0，则选择的是周日，以此类推，如果是6则选择的是周6
    let week: u32 = 5;
    if let 5 = week % 7 {
        println!("您选择的是周五！");
    } else {
        println!("未知！");
    }
    println!("\n*****************4、while let************************");
    // match loop
    let mut vec = vec![1, 3, 5, 7, 9];
    println!("match loop:");
    loop {
        match vec.pop() {
            None => {
                break;
            }
            Some(value) => {
                print!("{} ", value);
            }
        }
    }

    // 改造为while let
    println!("\nwhile let:");
    let mut vec = vec![1, 3, 5, 7, 9];
    while let Some(value) = vec.pop() {
        print!("{} ", value);
    }
}
