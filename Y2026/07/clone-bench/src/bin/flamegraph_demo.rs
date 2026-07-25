use std::env;
use std::hint::black_box;

// 100_000 个 u64 一共占 800_000 字节，也就是 800 KB。
const DATA_LEN: usize = 100_000;
// 默认复制 50_000 次，让采样 profiler 有足够长的运行时间观察调用栈。
const DEFAULT_ROUNDS: usize = 50_000;

fn main() {
    // 可以通过命令行修改复制次数，例如：cargo run --release --bin flamegraph_demo -- 10000
    let rounds = env::args()
        .nth(1)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(DEFAULT_ROUNDS);
    let numbers = vec![7_u64; DATA_LEN];

    let result = clone_and_read(&numbers, rounds);
    println!("完成 {rounds} 次深复制，校验结果：{result}");
}

/// 保留这一层函数，方便在火焰图中找到业务调用入口。
#[inline(never)]
fn clone_and_read(input: &[u64], rounds: usize) -> u64 {
    let mut total = 0_u64;

    for _ in 0..rounds {
        // to_vec 会分配新的 Vec 缓冲区，并深复制 input 中全部 100_000 个 u64。
        // black_box 防止编译器根据当前输入把这段复制优化掉。
        let copied = black_box(input.to_vec());
        total = total.wrapping_add(read_slice(black_box(copied.as_slice())));
        // copied 在本轮结束时析构，释放刚刚创建的堆内存。
    }

    black_box(total)
}

/// 只读取长度和第一个元素，避免把遍历整个 Vec 的成本混进本次示例。
#[inline(never)]
fn read_slice(data: &[u64]) -> u64 {
    data.len() as u64 + data.first().copied().unwrap_or_default()
}
