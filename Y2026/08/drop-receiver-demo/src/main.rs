mod drop_order;
mod manual_drop;
mod option_take;
mod option_worker;
mod two_phase;
mod worker;

fn main() {
    println!("=== 1. Drop::drop 与字段析构的两个阶段 ===");
    two_phase::run();

    println!("\n=== 2. 使用 Option::take 安全取出字段 ===");
    option_take::run();

    println!("\n=== 3. OptionWorker ===");
    option_worker::run();

    println!("\n=== 4. 结构体字段的析构顺序 ===");
    drop_order::run();
}
