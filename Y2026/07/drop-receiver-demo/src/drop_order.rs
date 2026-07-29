struct Field(&'static str);

impl Drop for Field {
    fn drop(&mut self) {
        println!("析构字段 {}", self.0);
    }
}

struct Container {
    first: Field,
    second: Field,
}

impl Drop for Container {
    fn drop(&mut self) {
        println!(
            "先执行 Container::drop，此时仍可访问 {} 和 {}",
            self.first.0, self.second.0
        );
    }
}

pub fn run() {
    let _container = Container {
        first: Field("first"),
        second: Field("second"),
    };

    // 结构体先执行自己的 Drop::drop。
    // 方法返回后，字段按照声明顺序析构：先 first，再 second。
}
