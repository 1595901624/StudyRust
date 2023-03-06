fn main() {
    // // 某游戏账号结构体
    // // 1. 定义结构体
    // #[derive(Debug)]
    // struct Account {
    //     // 账号id i32
    //     id: u32,
    //     // 账号状态 是否是正常状态 true:正常 false:异常
    //     status: bool,
    //     // 账号类型 'n'是普通用户 's'是高级用户
    //     acc_type: char,
    // }
    //
    // // 2. 创建一个账号
    // let acc1 = Account {
    //     id: 1,
    //     status: true,
    //     acc_type: 'n',
    // };
    //
    // let id = 2;
    // let status = false;
    // let acc2 = Account {
    //     id,
    //     status,
    //     acc_type: 'n',
    // };
    //
    // // 打印结构体 需要在结构体上注解 #[derive(Debug)]
    // dbg!(acc1);
    // dbg!(acc2);
    // // println!("acc1的账号信息: {:?}", acc1);
    //
    // test();

    let name = String::from("com.123123");
    if name.contains(".") {
        let s = name.replace(".", "/");
        println!("{}", s);
    }
}


///
/// 两个乒乓球队进行比赛，各出三人。甲队为a,b,c三人，乙队为x,y,z三人。已抽签决定比赛名单。
/// 有人向队员打听比赛的名单。a说他不和x比，c说他不和x,z比，请编程序找出三队赛手的名单。
fn test() {

    let team1 = ['a', 'b', 'c'];
    let team2 = ['x', 'y', 'z'];


    for i in team1.iter() {
        for j in team2.iter() {}
    }
}
