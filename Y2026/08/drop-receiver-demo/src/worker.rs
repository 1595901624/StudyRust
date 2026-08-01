/// 下面的代码无法编译

/// 结构体 Worker，表示一个工人
struct Worker {
    name: String,
}

impl Drop for Worker {
    fn drop(&mut self) {
        println!("释放工人 {}", self.name);
    }
}

impl Worker {
    // 将 name 字段的所有权转移出去，返回一个 String
    // 下面的方法会导致编译错误
    // fn into_name(&mut self) -> String {
    // self.name
    // }

    fn into_name(&mut self) -> String {
        // std::mem::take 会把 self.name 取出来，同时给它留下一个空的 String
        // 或者你可以自己调用 std::mem::replace(&mut self.name, String::new()) 来实现同样的效果
        std::mem::take(&mut self.name)
    }
}
