/// 下面的代码无法编译

/// 结构体 Worker，表示一个工人
struct OptionWorker {
    name: Option<String>,
}

impl Drop for OptionWorker {
    fn drop(&mut self) {
        if let Some(name) = self.name.take() {
            println!("释放工人 {}", name);
        }
    }
}

impl OptionWorker {
    fn into_name(&mut self) -> String {
        self.name.take().expect("name 已经被取走")
    }
}

pub fn run() {
    let mut worker = OptionWorker {
        name: Some(String::from("Alice")),
    };

    let name = worker.into_name();
    println!("工人名称为 {}", name);

    // worker 离开作用域时，会进入上面的 Drop::drop
}