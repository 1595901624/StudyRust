fn main() {
    // 1. 接收参数
    for arg in std::env::args()
    {
        println!("{}", arg);
    }

    // 2. println 和 eprintln的区别
    println!("我是 println!");
    eprintln!("我是 eprintln!");

    // 3.dbg!
    dbg!("我是一条 debug 日志");

    // 4. main函数返回值
    std::process::exit(0);
}
