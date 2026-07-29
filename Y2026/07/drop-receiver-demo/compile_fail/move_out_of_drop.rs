// 这个文件故意无法通过编译，用于观察 E0509。
// 运行：rustc compile_fail/move_out_of_drop.rs

struct Worker {
    name: String,
}

impl Drop for Worker {
    fn drop(&mut self) {
        println!("drop worker {}", self.name);
    }
}

impl Worker {
    fn into_name(self) -> String {
        // Worker 实现了 Drop，析构方法和 drop glue 后面还需要看到完整的 self。
        // 因此，安全 Rust 不允许直接移出这个非 Copy 字段。
        self.name
    }
}

fn main() {
    let worker = Worker {
        name: String::from("worker-1"),
    };
    println!("{}", worker.into_name());
}

