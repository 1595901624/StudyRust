fn main() {
    // 1. 创建结构体
    // 某游戏账号结构体
    struct Account {
        // 账号id i32
        id: u32,
        // 账号状态 是否是正常状态 true:正常 false:异常
        status: bool,
        // 账号类型 'n'是普通用户 's'是高级用户
        acc_type: char,
    }

    // 2. 实例化结构体
    // 2.1 不可变实例
    let account = Account {
        id: 1,
        status: false,
        acc_type: 's',
    };

    // 2.2 可变实例
    let mut account = Account {
        id: 1,
        status: false,
        acc_type: 's',
    };

    // 2.3 外部变量名与结构体属性相同
    let id = 1;
    let mut account = Account {
        id, // 这里的 id 等同于 id: id,
        status: false,
        acc_type: 's',
    };

    let mut account2 = Account {
        id: 3,
        ..account // 其余字段使用account实例对应的字段
    };

    let mut account3 = Account {
        id: 4,
        ..account // 其余字段使用account实例对应的字段
    };

    //3. 修改和访问
    //3.1 访问
    println!("某游戏账号的 id 是 {}, 当前的用户状态: {}, 用户类型为 {}", account.id, account.status, account.acc_type);

    //3.2 修改
    account.id = 99;
    account.status = true;
    account.acc_type = 'n';
    println!("[修改后]某游戏账号的 id 是 {}, 当前的用户状态: {}, 用户类型为 {}", account.id, account.status, account.acc_type);
}
